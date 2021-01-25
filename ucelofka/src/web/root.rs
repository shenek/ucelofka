use actix_files::file_extension_to_mime;
use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse, Scope};
use include_dir::{include_dir, Dir};

static INDEX: Dir = include_dir!("../webapp/");

async fn get_root(_req: HttpRequest, web::Path((path,)): web::Path<(String,)>) -> HttpResponse {
    if path.is_empty() {
        let file = INDEX
            .get_file("index.html")
            .expect("index.html should be always present");
        HttpResponse::Ok()
            .content_type("text/html")
            .body(file.contents())
    } else {
        INDEX.get_file(&path).map_or_else(
            || HttpResponse::new(StatusCode::NOT_FOUND),
            |file| {
                let extension = path.rsplitn(2, '.').collect::<Vec<&str>>()[0];
                HttpResponse::Ok()
                    .content_type(&file_extension_to_mime(extension).to_string())
                    .body(file.contents())
            },
        )
    }
}

pub(super) fn root_endpoint() -> Scope {
    web::scope("").service(web::resource("/{path:.*}").route(web::get().to(get_root)))
}
