use actix_files::file_extension_to_mime;
use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse, Scope};
use include_dir::{include_dir, Dir};

static STATIC: Dir = include_dir!("../static/");

async fn get_static(_req: HttpRequest, web::Path((path,)): web::Path<(String,)>) -> HttpResponse {
    STATIC.get_file(&path).map_or_else(
        || HttpResponse::new(StatusCode::NOT_FOUND),
        |file| {
            HttpResponse::Ok()
                .content_type(&file_extension_to_mime(&path).to_string())
                .body(file.contents())
        },
    )
}

pub(super) fn static_endpoint() -> Scope {
    web::scope("static").service(web::resource("/{path:.*}").route(web::get().to(get_static)))
}
