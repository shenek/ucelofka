use std::sync::Mutex;

use actix_web::{error, web, HttpRequest, Result, Scope};

use crate::{actions::account, data::account::Account};

use super::{UcelofkaData, WrappedError};

async fn get_accounts(
    data: web::Data<Mutex<UcelofkaData>>,
    req: HttpRequest,
) -> Result<web::Json<Vec<Account>>> {
    println!("{:?}", req);
    let data_guard = data.lock().unwrap();

    let accounts = account::list(&data_guard.data_dir_path).map_err(WrappedError::from)?;

    Ok(web::Json(accounts.accounts))
}

async fn get_account(
    data: web::Data<Mutex<UcelofkaData>>,
    req: HttpRequest,
    path: web::Path<(String,)>,
) -> Result<web::Json<Account>> {
    println!("{:?}", req);
    let data_guard = data.lock().unwrap();

    let account_object =
        account::get(&data_guard.data_dir_path, &path.0).map_err(error::ErrorNotFound)?;

    Ok(web::Json(account_object))
}

pub(super) fn account_endpoint() -> Scope {
    web::scope("account")
        .service(web::resource("/").route(web::get().to(get_accounts)))
        .service(web::resource("/{id}").route(web::get().to(get_account)))
}
