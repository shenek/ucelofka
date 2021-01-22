use std::sync::Mutex;

use actix_web::{error, web, HttpRequest, Result, Scope};

use crate::{actions::identity, data::identity::Identity};

use super::{UcelofkaData, WrappedError};

async fn get_identities(
    data: web::Data<Mutex<UcelofkaData>>,
    req: HttpRequest,
) -> Result<web::Json<Vec<Identity>>> {
    println!("{:?}", req);
    let data_guard = data.lock().unwrap();

    let identities = identity::list(&data_guard.data_dir_path).map_err(WrappedError::from)?;

    Ok(web::Json(identities.identities))
}

async fn get_identity(
    data: web::Data<Mutex<UcelofkaData>>,
    req: HttpRequest,
    web::Path((id,)): web::Path<(String,)>,
) -> Result<web::Json<Identity>> {
    println!("{:?}", req);
    let data_guard = data.lock().unwrap();

    let identity_object =
        identity::get(&data_guard.data_dir_path, &id).map_err(error::ErrorNotFound)?;

    Ok(web::Json(identity_object))
}

pub(super) fn identity_endpoint() -> Scope {
    web::scope("identity")
        .service(web::resource("/").route(web::get().to(get_identities)))
        .service(web::resource("/{id}").route(web::get().to(get_identity)))
}
