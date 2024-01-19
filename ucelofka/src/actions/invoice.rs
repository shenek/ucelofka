use anyhow::{anyhow, Result};
use fluent::fluent_args;
use git2::Repository;
use std::{collections::HashSet, convert::TryFrom, convert::TryInto, fs, path::Path};
use tera::{Context, Tera};

use crate::{
    actions,
    data::{
        invoice::{Invoice, Invoices},
        template::Templates,
    },
    storage::{Record, Records},
    translations::get_message,
};

struct WrappedInvoice(Invoice);

impl TryFrom<WrappedInvoice> for Context {
    type Error = anyhow::Error;
    fn try_from(wrapped_invoice: WrappedInvoice) -> Result<Self> {
        Self::from_serialize(wrapped_invoice.0).map_err(anyhow::Error::from)
    }
}

impl From<Invoice> for WrappedInvoice {
    fn from(val: Invoice) -> Self {
        WrappedInvoice(val)
    }
}

pub fn create(
    data_path: &Path,
    customer: &str,
    identity: &str,
    account: &str,
    entries: Vec<String>,
    git: bool,
    due: Option<usize>,
) -> Result<String> {
    let account = actions::account::get(data_path, account)?;
    let customer = actions::customer::get(data_path, customer)?;
    let identity = actions::identity::get(data_path, identity)?;
    let mut entries_vec = Vec::new();
    for entry in entries {
        let entry_item = actions::entry::get(data_path, &entry)?;
        entries_vec.push(entry_item);
    }
    // Test same currencies among entries
    let currencies: HashSet<String> = entries_vec.iter().map(|e| e.currency.clone()).collect();
    if currencies.len() > 1 {
        let mut currencies_vec = currencies.iter().map(String::as_str).collect::<Vec<_>>();
        currencies_vec.sort_unstable();
        let currencies_str = currencies_vec.join(", ");
        return Err(anyhow!(get_message(
            "entries-different-invoice",
            Some(fluent_args!["currencies" => currencies_str])
        )));
    }

    let invoices = list(data_path);
    let new_invoice = Invoice::new(
        identity,
        account,
        customer,
        &entries_vec,
        invoices?.invoices,
        due,
    );

    let invoice_path = data_path.join(Path::new("invoices"));

    new_invoice
        .store(&invoice_path)
        .map_err(|err| anyhow!("{}", err))?;

    if git {
        let git_path = Path::new("invoices").join(Path::new(&format!("{}.yml", new_invoice.id)));
        let repo = Repository::open(data_path)
            .map_err(|err| anyhow!("Faield to open git repository {}", err))?;

        let mut index = repo
            .index()
            .map_err(|err| anyhow!("Failed to get repo index ({})", err))?;

        index.add_path(&git_path).map_err(|err| {
            anyhow!(
                "Failed to add a file {} ({})",
                git_path.to_string_lossy(),
                err
            )
        })?;
        index
            .write()
            .map_err(|err| anyhow!("Failed to write to index ({})", err))?;
    }

    Ok(new_invoice.id.to_string())
}

pub fn ids(data_path: &Path) -> Result<String> {
    let data = list(data_path)?;
    Ok(data.ids().join("\n"))
}

pub fn list(data_path: &Path) -> Result<Invoices> {
    let invoice_path = data_path.join(Path::new("invoices"));
    Invoices::load(invoice_path.as_path())
}

pub fn get(data_path: &Path, id: &str) -> Result<Invoice> {
    list(data_path)?
        .get(id)
        .ok_or_else(|| anyhow!("Invoice {} not found.", id))
}

pub fn render(data_path: &Path, invoice: &str, template: &str, git: bool) -> Result<String> {
    // get the invoice data
    let invoice_path = data_path.join(Path::new("invoices"));
    let invoices = Invoices::load(invoice_path.as_path())?;
    let data = invoices
        .get(invoice)
        .ok_or_else(|| anyhow!("failed to find invoice {}", invoice))?;

    // Load the templates
    let templates_path = data_path.join(Path::new("templates"));
    let templates = Templates::load(&templates_path)?;

    let template_instance = templates
        .get(template)?
        .ok_or_else(|| anyhow!("failed to find template {}", template))?;

    // Render
    let templates_path_str = templates_path
        .to_str()
        .ok_or_else(|| anyhow!("Wrong path string"))?;
    let renderer = Tera::new(&format!("{}/*", templates_path_str)).map_err(|err| {
        anyhow!(
            "Failed to parse templates from {}: {}",
            templates_path_str,
            err
        )
    })?;

    let currency = data.entries[0].currency.clone();
    let wrapped_invoice: WrappedInvoice = data.into();
    let mut context: Context = wrapped_invoice.try_into()?;
    context.insert("currency", &currency);
    let output = match renderer.render(&template_instance.name[..], &context) {
        Ok(data) => data,
        Err(err) => return Err(anyhow!("{}", err)),
    };

    // Store output
    let suffix: String = match Path::new(template).extension() {
        Some(os_string) => os_string
            .to_str()
            .ok_or_else(|| anyhow!("Wrong path string"))?
            .to_string(),
        None => String::new(),
    };
    let output_name = format!("{}.{}", invoice, suffix);
    let output_path = data_path
        .join(Path::new("output"))
        .join(Path::new(&output_name[..]));

    fs::write(output_path.clone(), output)
        .map_err(|err| anyhow!("failed to write to output file: {}", err))?;

    if git {
        let git_path = Path::new("output").join(Path::new(&output_name));
        let repo = Repository::open(data_path)
            .map_err(|err| anyhow!("Faield to open git repository {}", err))?;

        let mut index = repo
            .index()
            .map_err(|err| anyhow!("Failed to get repo index ({})", err))?;

        index.add_path(&git_path).map_err(|err| {
            anyhow!(
                "Failed to add a file {} ({})",
                output_path.to_string_lossy(),
                err
            )
        })?;
        index
            .write()
            .map_err(|err| anyhow!("Failed to write to index ({})", err))?;
    }

    Ok(output_path.to_str().unwrap().into())
}
