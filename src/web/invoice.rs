use std::sync::Mutex;

use actix_web::{error, web, HttpRequest, Result, Scope};

use crate::{actions::invoice, data::invoice::Invoice};

use super::{UcelofkaData, WrappedError};

async fn get_invoices(
    data: web::Data<Mutex<UcelofkaData>>,
    req: HttpRequest,
) -> Result<web::Json<Vec<Invoice>>> {
    println!("{:?}", req);
    let data_guard = data.lock().unwrap();

    let invoices = invoice::list(&data_guard.data_dir_path).map_err(WrappedError::from)?;

    Ok(web::Json(invoices.invoices))
}

async fn get_invoice(
    data: web::Data<Mutex<UcelofkaData>>,
    req: HttpRequest,
    web::Path((id,)): web::Path<(String,)>,
) -> Result<web::Json<Invoice>> {
    println!("{:?}", req);
    let data_guard = data.lock().unwrap();

    let invoice_object =
        invoice::get(&data_guard.data_dir_path, &id).map_err(error::ErrorNotFound)?;

    Ok(web::Json(invoice_object))
}

pub(super) fn invoice_endpoint() -> Scope {
    web::scope("invoice")
        .service(web::resource("/").route(web::get().to(get_invoices)))
        .service(web::resource("/{id}").route(web::get().to(get_invoice)))
}
