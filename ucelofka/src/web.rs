mod account;
mod customer;
mod entry;
mod identity;
mod invoice;
mod root;
mod template;

use std::{fmt, path::PathBuf, sync::Mutex};

use actix_web::{error::ResponseError, web, App, HttpServer};
use anyhow::Result;
use tokio::runtime::Runtime;

#[derive(Clone)]
struct UcelofkaData {
    data_dir_path: PathBuf,
}

#[derive(Debug)]
struct WrappedError {
    error: anyhow::Error,
}

impl fmt::Display for WrappedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(f)
    }
}

impl ResponseError for WrappedError {}

impl From<anyhow::Error> for WrappedError {
    fn from(err: anyhow::Error) -> Self {
        Self { error: err }
    }
}

#[actix_web::main]
async fn actix_main(port: u16, data_dir: PathBuf) -> std::io::Result<()> {
    let data = web::Data::new(Mutex::new(UcelofkaData {
        data_dir_path: data_dir,
    }));
    println!("Starting ucelfka web on http://localhost:{}", port);
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(
                web::scope("api")
                    .service(account::account_endpoint())
                    .service(customer::customer_endpoint())
                    .service(entry::entry_endpoint())
                    .service(identity::identity_endpoint())
                    .service(invoice::invoice_endpoint())
                    .service(template::template_endpoint()),
            )
            .service(root::root_endpoint())
    })
    .bind(("localhost", port))?
    .workers(1)
    .run()
    .await
}

pub fn run(port: u16, data_dir: PathBuf) -> Result<()> {
    let rt = Runtime::new().unwrap();
    rt.block_on(async { actix_main(port, data_dir) })
        .map_err(|e| e.into())
}
