use std::{fs, path::Path};
use tera::Tera;

use crate::{
    actions,
    data::{
        invoice::{Invoice, Invoices},
        template::Templates,
    },
};

pub fn create<'a>(
    data_path: &'a Path,
    customer: &str,
    identity: &str,
    account: &str,
    entries: Vec<&str>,
) {
    let account = actions::account::get(data_path, account);
    let customer = actions::customer::get(data_path, customer);
    let identity = actions::identity::get(data_path, identity);
    for entry in entries {
        let entry_rec = actions::entry::get(data_path, entry);
    }
}

pub fn list(data_path: &Path) -> Invoices {
    let invoice_path = data_path.join(Path::new("invoices"));
    Invoices::load(invoice_path.as_path())
}

pub fn get(data_path: &Path, id: u64) -> Option<Invoice> {
    for invoice in list(data_path).invoices {
        if invoice.id == id {
            return Some(invoice);
        }
    }
    None
}

pub fn render(data_path: &Path, invoice: &str, template: &str) {
    // get the invoice data
    let invoice_id = invoice
        .parse::<u64>()
        .expect(&format!("{} is not a valid invoice number", invoice)[..]);
    let invoice_path = data_path.join(Path::new("invoices"));
    let invoices = Invoices::load(invoice_path.as_path());
    let data = invoices
        .get(invoice_id)
        .expect(&format!("failed to find invoice {}", invoice_id)[..]);

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

    let output = match renderer.render(&template_instance.name[..], data.into()) {
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
