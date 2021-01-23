use actix_files::file_extension_to_mime;
use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse, Scope};
use include_dir::{include_dir, Dir};

static ASSETS: Dir = include_dir!("static/");

async fn get_static(_req: HttpRequest, web::Path((path,)): web::Path<(String,)>) -> HttpResponse {
    if let Some(file) = ASSETS.get_file(&path) {
        HttpResponse::Ok()
            .content_type(&file_extension_to_mime(&path).to_string())
            .body(file.contents())
    } else {
        HttpResponse::new(StatusCode::NOT_FOUND)
    }
}

pub(super) fn static_endpoint() -> Scope {
    web::scope("static").service(web::resource("/{path:.*}").route(web::get().to(get_static)))
}
