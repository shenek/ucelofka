use std::{fs, path::Path};
use tera::Tera;

use crate::data::invoice::Invoices;
use crate::data::template::Templates;

pub fn create<'a>(
    data_path: &'a Path,
    customer: &str,
    identity: &str,
    account: &str,
    entries: &[&str],
) {
}

pub fn list(data_path: &Path) {
    let invoice_path = data_path.join(Path::new("invoices"));
    println!("{}", Invoices::load(invoice_path.as_path()));
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
