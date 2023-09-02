use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, Error};
use actix_files as fs;
use imageproc::drawing::draw_text_mut;
use image::{ImageFormat, load};
use rusttype::{Font, Scale};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}