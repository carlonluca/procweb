/**
 * Copyright (C) 2023 Luca Carlon. All rights reserved.
 * 
 * This file is part of procweb-rust.
 * 
 * procweb-rust is free software: you can redistribute it and/or modify it under the terms of the GNU
 * General Public License as published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 * 
 * procweb-rust is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 * without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
 * PURPOSE. See the GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License along with procweb-rust. If not,
 * see <https://www.gnu.org/licenses/>.
 */

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
use pwsamplerdocker::PWSampleDocker;
use pwsamplerdocker::PWSetupDocker;
use pwsamplerproc::PWSamplerProc;
use pwsamplerthread::PWSamplerThread;
use pwsampler::PWSampler;
use pwsamplerdocker::PWSamplerDocker;
use pwdata::{PWSampleProc, PWSetupProc};
use std::include_bytes;
use std::collections::HashMap;

mod pwsamplerthread;
mod pwsamplerdocker;
mod pwsamplerproc;
mod pwsampler;
mod pwreader;
mod pwdata;
mod pwudsconnector;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    pid: i64,
    /// TCP port of the HTTP server
    #[arg(short, long, default_value_t = 3000)]
    port: u16
}

#[get("/api/proc/samples")]
async fn get_samples(data: web::Data<Arc<Mutex<PWSamplerProc>>>) -> impl Responder {
    let samples = data.lock().unwrap().samples();
    let _samples = samples.lock().unwrap();
    let __samples = &*_samples;
    web::Json(__samples.clone())
}

#[get("/api/proc/setup")]
async fn get_setup(data: web::Data<Arc<Mutex<PWSamplerProc>>>) -> impl Responder {
    web::Json(data.lock().unwrap().setup().clone())
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

    let sampler = Arc::new(Mutex::new(PWSamplerProc::new(cli.pid)));
    let mut sampler_thread = PWSamplerThread::<PWSampleProc, PWSetupProc>::new(
        sampler.clone()
    );
    sampler_thread.start();

    let sampler_docker = Arc::new(Mutex::new(PWSamplerDocker::new()));
    let mut sampler_thread_docker = PWSamplerThread::<PWSampleDocker, PWSetupDocker>::new(
        sampler_docker.clone()
    );
    sampler_thread_docker.start();

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
