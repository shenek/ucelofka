use std::sync::Mutex;

use actix_web::{error, web, HttpRequest, Result, Scope};

use crate::{actions::template, data::template::Template};

use super::{UcelofkaData, WrappedError};

async fn get_templates(
    data: web::Data<Mutex<UcelofkaData>>,
    req: HttpRequest,
) -> Result<web::Json<Vec<Template>>> {
    println!("{:?}", req);
    let data_guard = data.lock().unwrap();

    let templates = template::list(&data_guard.data_dir_path).map_err(WrappedError::from)?;

    Ok(web::Json(templates.templates))
}

async fn get_template(
    data: web::Data<Mutex<UcelofkaData>>,
    req: HttpRequest,
    web::Path((id,)): web::Path<(String,)>,
) -> Result<web::Json<Template>> {
    println!("{:?}", req);
    let data_guard = data.lock().unwrap();

    let template_object =
        template::get(&data_guard.data_dir_path, &id).map_err(error::ErrorNotFound)?;

    Ok(web::Json(template_object))
}

pub(super) fn template_endpoint() -> Scope {
    web::scope("template")
        .service(web::resource("/").route(web::get().to(get_templates)))
        .service(web::resource("/{id}").route(web::get().to(get_template)))
}
