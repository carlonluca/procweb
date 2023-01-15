use actix_web::{get, web, App, HttpServer, Responder};
mod pwdata;

#[macro_use]
extern crate log;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    let sample: pwdata::PWSample = pwdata::PWSample::default();
    web::Json(sample)
}

///
/// Entry point.
///
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    info!("Version {}", VERSION);

    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
