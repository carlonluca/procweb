use std::sync::Arc;
use std::sync::Mutex;

use actix_web::{get, web, App, HttpServer, Responder};
use pwsampler::PWSampler;
mod pwsampler;
mod pwdata;

#[get("/api/samples")]
async fn get_samples(data: web::Data<Arc<Mutex<PWSampler>>>) -> impl Responder {
    let samples = data.lock().unwrap().samples();
    let _samples = samples.lock().unwrap();
    let __samples = &*_samples;
    web::Json(__samples.clone())
}

///
/// Entry point.
///
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    log::info!("Version {}", VERSION);

    let sampler = Arc::new(Mutex::new(pwsampler::PWSampler::new()));
    sampler.lock().unwrap().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(sampler.clone()))
            .service(get_samples)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
