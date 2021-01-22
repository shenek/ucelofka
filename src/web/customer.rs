use std::sync::Mutex;

use actix_web::{error, web, HttpRequest, Result, Scope};

use crate::{actions::customer, data::customer::Customer};

use super::{UcelofkaData, WrappedError};

async fn get_customers(
    data: web::Data<Mutex<UcelofkaData>>,
    req: HttpRequest,
) -> Result<web::Json<Vec<Customer>>> {
    println!("{:?}", req);
    let data_guard = data.lock().unwrap();

    let customers = customer::list(&data_guard.data_dir_path).map_err(WrappedError::from)?;

    Ok(web::Json(customers.customers))
}

async fn get_customer(
    data: web::Data<Mutex<UcelofkaData>>,
    req: HttpRequest,
    web::Path((id,)): web::Path<(String,)>,
) -> Result<web::Json<Customer>> {
    println!("{:?}", req);
    let data_guard = data.lock().unwrap();

    let customer_object =
        customer::get(&data_guard.data_dir_path, &id).map_err(error::ErrorNotFound)?;

    Ok(web::Json(customer_object))
}

pub(super) fn customer_endpoint() -> Scope {
    web::scope("customer")
        .service(web::resource("/").route(web::get().to(get_customers)))
        .service(web::resource("/{id}").route(web::get().to(get_customer)))
}
