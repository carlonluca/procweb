use std::sync::Arc;
use std::sync::Mutex;
use actix_web::http::header::ContentType;
use clap::Parser;
use actix_web::{
    get,
    web,
    App,
    HttpServer,
    Responder,
    HttpResponse
};
use pwsampler::PWSampler;
use std::include_bytes;
use std::collections::HashMap;
mod pwsampler;
mod pwreader;
mod pwdata;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    pid: i64,
    /// TCP port of the HTTP server
    #[arg(short, long, default_value_t = 3000)]
    port: u16
}

#[get("/api/samples")]
async fn get_samples(data: web::Data<Arc<Mutex<PWSampler>>>) -> impl Responder {
    let samples = data.lock().unwrap().samples();
    let _samples = samples.lock().unwrap();
    let __samples = &*_samples;
    web::Json(__samples.clone())
}

#[get("/api/setup")]
async fn get_setup(data: web::Data<Arc<Mutex<PWSampler>>>) -> impl Responder {
    web::Json(data.lock().unwrap().setup())
}

#[get("/{filename:.*}")]
async fn get_web(filename: web::Path<String>) -> HttpResponse {
    let res: HashMap<&str, &'static [u8]> = HashMap::from([
        ("index.html", include_bytes!("../index.html") as &'static [u8]),
        ("main.js", include_bytes!("../main.js")),
        ("polyfills.js", include_bytes!("../polyfills.js")),
        ("runtime.js", include_bytes!("../runtime.js")),
        ("styles.css", include_bytes!("../styles.css")),
        ("386.js", include_bytes!("../386.js")),
        ("icon.svg", include_bytes!("../icon.svg"))
    ]);
    let _filename = if filename.is_empty() {
        web::Path::from(String::from("index.html"))
    } else {
        filename
    };
        
    match res.get(_filename.as_str()) {
        None => HttpResponse::NotFound().into(),
        Some(v) => {
            match mime_guess::from_path(_filename.as_str()).first() {
                None => HttpResponse::NotFound().into(),
                Some(mime) => {
                    HttpResponse::Ok()
                        .insert_header(ContentType(mime))
                        .body(v.clone())
                }
            }
        }
    }
}

///
/// Entry point.
///
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    log::info!("Version {}", VERSION);

    let sampler = Arc::new(Mutex::new(pwsampler::PWSampler::new(cli.pid)));
    sampler.lock().unwrap().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(sampler.clone()))
            .service(get_samples)
            .service(get_setup)
            .service(get_web)
    })
    .bind(("0.0.0.0", cli.port))?
    .run()
    .await
}
