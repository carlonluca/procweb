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
    HttpResponse,
    http::{
        StatusCode
    }
};
use pwsampler::PWSampler;
use std::include_bytes;
use std::collections::HashMap;
use mime;
mod pwsampler;
mod pwreader;
mod pwdata;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    pid: i64
}

#[get("/api/samples")]
async fn get_samples(data: web::Data<Arc<Mutex<PWSampler>>>) -> impl Responder {
    let samples = data.lock().unwrap().samples();
    let _samples = samples.lock().unwrap();
    let __samples = &*_samples;
    web::Json(__samples.clone())
}

#[get("/{filename:.*}")]
async fn get_web(filename: web::Path<String>) -> HttpResponse {
    let res: HashMap<&str, &'static [u8]> = HashMap::from([
        ("index.html", include_bytes!("../../procweb-webui/dist/procweb-webui/index.html") as &'static [u8]),
        ("main.js", include_bytes!("../../procweb-webui/dist/procweb-webui/main.js")),
        ("polyfills.js", include_bytes!("../../procweb-webui/dist/procweb-webui/polyfills.js")),
        ("runtime.js", include_bytes!("../../procweb-webui/dist/procweb-webui/runtime.js")),
        ("styles.css", include_bytes!("../../procweb-webui/dist/procweb-webui/styles.css")),
        ("386.js", include_bytes!("../../procweb-webui/dist/procweb-webui/386.js")),
        ("icon.svg", include_bytes!("../../icon.svg"))
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
    log::info!("PID: {}", cli.pid);

    let sampler = Arc::new(Mutex::new(pwsampler::PWSampler::new(cli.pid)));
    sampler.lock().unwrap().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(sampler.clone()))
            .service(get_samples)
            .service(get_web)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
