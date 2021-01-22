use std::sync::Mutex;

use actix_web::{error, web, HttpRequest, Result, Scope};

use crate::{actions::entry, data::entry::Entry};

use super::{UcelofkaData, WrappedError};

async fn get_entries(
    data: web::Data<Mutex<UcelofkaData>>,
    req: HttpRequest,
) -> Result<web::Json<Vec<Entry>>> {
    println!("{:?}", req);
    let data_guard = data.lock().unwrap();

    let entries = entry::list(&data_guard.data_dir_path).map_err(WrappedError::from)?;

    Ok(web::Json(entries.entries))
}

async fn get_entry(
    data: web::Data<Mutex<UcelofkaData>>,
    req: HttpRequest,
    web::Path((id,)): web::Path<(String,)>,
) -> Result<web::Json<Entry>> {
    println!("{:?}", req);
    let data_guard = data.lock().unwrap();

    let entry_object = entry::get(&data_guard.data_dir_path, &id).map_err(error::ErrorNotFound)?;

    Ok(web::Json(entry_object))
}

pub(super) fn entry_endpoint() -> Scope {
    web::scope("entry")
        .service(web::resource("/").route(web::get().to(get_entries)))
        .service(web::resource("/{id}").route(web::get().to(get_entry)))
}
