use failure::Fail;
use std::{fs, path::Path};
use tera::{Context, Tera};

use crate::{
    actions,
    data::{
        invoice::{Invoice, Invoices},
        template::Templates,
        Record, Records,
    },
};

#[derive(Fail, Debug)]
#[fail(display = "Create invoice failed {}", msg)]
pub struct CreateError {
    msg: String,
}

pub fn create(
    data_path: &Path,
    customer: &str,
    identity: &str,
    account: &str,
    entries: Vec<&str>,
) -> Result<String, CreateError> {
    let account = actions::account::get(data_path, account).ok_or_else(|| CreateError {
        msg: format!("Account {} not found.", account),
    })?;
    let customer = actions::customer::get(data_path, customer).ok_or_else(|| CreateError {
        msg: format!("Customer {} not found.", customer),
    })?;
    let identity = actions::identity::get(data_path, identity).ok_or_else(|| CreateError {
        msg: format!("Identity {} not found.", identity),
    })?;
    let mut entries_vec = Vec::new();
    for entry in entries {
        let entry_item = actions::entry::get(data_path, entry).ok_or_else(|| CreateError {
            msg: format!("Entry {} not found.", entry),
        })?;
        entries_vec.push(entry_item);
    }
    let invoices = list(data_path);
    let new_invoice = Invoice::new(identity, account, customer, entries_vec, invoices.invoices);

    let invoice_path = data_path.join(Path::new("invoices"));

    new_invoice
        .store(&invoice_path)
        .map_err(|err| CreateError {
            msg: format!("{}", err),
        })?;

    Ok(new_invoice.id.to_string())
}

pub fn list(data_path: &Path) -> Invoices {
    let invoice_path = data_path.join(Path::new("invoices"));
    Invoices::load(invoice_path.as_path())
}

pub fn get(data_path: &Path, id: &str) -> Option<Invoice> {
    list(data_path).get(id)
}

pub fn render(data_path: &Path, invoice: &str, template: &str) {
    // get the invoice data
    let invoice_path = data_path.join(Path::new("invoices"));
    let invoices = Invoices::load(invoice_path.as_path());
    let data = invoices
        .get(invoice)
        .expect(&format!("failed to find invoice {}", invoice)[..]);

    // Load the templates
    let templates_path = data_path.join(Path::new("templates"));
    let templates = Templates::load(&templates_path);

    let template_instance = templates
        .get(template)
        .expect(&format!("failed to find template {}", template)[..]);

    // Render
    let templates_path_str = templates_path.to_str().unwrap();
    let renderer = Tera::new(&format!("{}/*", templates_path_str)[..])
        .expect(&format!("Failed to parse templates from {}", templates_path_str)[..]);

    let context: Context = data.into();
    let output = match renderer.render(&template_instance.name[..], &context) {
        Ok(data) => data,
        Err(err) => panic!(format!("{}", err)),
    };

    // Store output
    let suffix: String = match Path::new(template).extension() {
        Some(os_string) => os_string.to_str().unwrap().to_string(),
        None => String::new(),
    };
    let output_name = format!("{}.{}", invoice, suffix);
    let output_path = data_path
        .join(Path::new("output"))
        .join(Path::new(&output_name[..]));

    fs::write(output_path, output).expect("failed to write to output file");
}
