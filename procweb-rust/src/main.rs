use std::sync::Arc;
use std::sync::Mutex;

use actix_web::{get, web, App, HttpServer, Responder};
use log::{debug, error, log_enabled, info, Level};
use pwsampler::PWSampler;
mod pwsampler;
mod pwdata;

#[get("/api/samples")]
async fn getSamples(data: web::Data<Arc<Mutex<PWSampler>>>) -> impl Responder {
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
        App::new().app_data(sampler.clone()).service(getSamples)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
