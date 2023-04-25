use actix_web::Error;
use awc::error::SendRequestError;
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

use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use tokio::io::Interest;
use tokio::net::UnixStream;
use awc::{ClientBuilder, Connector, SendClientRequest, ClientResponse, Client, ResponseBody};
use std::sync::{Arc, Mutex};
use std::path::Path;
use crate::pwudsconnector::UdsConnector;
use crate::PWSampler;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct PWSampleDocker {

} 

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct PWSetupDocker {

}

pub struct PWSamplerDocker {
    samples: Arc<Mutex<Vec<PWSampleDocker>>>
}

impl PWSamplerDocker {
    pub fn new() -> PWSamplerDocker {
        PWSamplerDocker {
            samples: Arc::new(Mutex::new(Vec::new()))
        }
    }
}

enum MyError {
    SendRequestError,
    PayloadError
}

impl PWSampler<PWSampleDocker, PWSetupDocker> for PWSamplerDocker {
    fn sample(&mut self) -> Option<PWSampleDocker> {
        /*let rt  = Runtime::new().unwrap();
        rt.block_on(async {
            let socket_path = Path::new("/var/run/docker.sock");
        let connector = Connector::new().connector(UdsConnector::new(socket_path));
        let client = ClientBuilder::new().connector(connector).finish();
            let data = client.get("http://localhost/version")
                .send()
                .await
                .unwrap()
                .body()
                .await
                .unwrap();
            log::warn!("Data: {:?}", data);
        });*/

        /*let rt  = Runtime::new().unwrap();
        let stream = match rt.block_on(UnixStream::connect(Path::new("/var/run/docker.sock"))) {
            Err(e) => {
                log::warn!("Error occurred trying to connect to docker socket: {:?}", e);
                return None;
            },
            Ok(v) => v
        };*/
        

        Some(PWSampleDocker {

        })
    }

    fn samples(&self) -> Arc<Mutex<Vec<PWSampleDocker>>> {
        self.samples.clone()
    }

    fn setup(&self) -> &PWSetupDocker {
        &PWSetupDocker {  }
    }
}
