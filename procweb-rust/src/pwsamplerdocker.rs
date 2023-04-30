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
use serde_json;
use tokio::runtime::Runtime;
use tokio::task::LocalSet;
use awc::{ClientBuilder, Connector};
use std::str::from_utf8;
use std::sync::{Arc, Mutex};
use std::path::Path;
use crate::pwudsconnector::UdsConnector;
use crate::PWSampler;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub struct PWSampleDockerContainer {
    pub id: String,
    pub name: String,
    pub image: String,
    pub cpu: f64
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct PWSampleDocker {
    pub containers: Vec<PWSampleDockerContainer>
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct PWSetupDocker {
}

#[derive(Serialize, Deserialize)]
pub struct PWDockerContainer {
    pub Id: String,
    pub Names: Vec<String>
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

impl PWSampler<PWSampleDocker, PWSetupDocker> for PWSamplerDocker {
    fn sample(&mut self) -> Option<PWSampleDocker> {
        let local = LocalSet::new();
        let rt = Runtime::new().unwrap();
        let join = local.spawn_local(async move {
            let socket_path = Path::new("/var/run/docker.sock");
            let connector = Connector::new().connector(UdsConnector::new(socket_path));
            let client = ClientBuilder::new().connector(connector).finish();
            let data = client.get("http://localhost/containers/json?all=true")
                .send()
                .await
                .unwrap()
                .body()
                .await
                .unwrap();
            let containers: Vec<PWDockerContainer> = serde_json::from_str(from_utf8(&data).unwrap()).unwrap();
            log::warn!("Containers: {:?}", containers.len());
            containers
        });
        
        let containers = match rt.block_on(join) {
            Err(_) => {
                return None;
            },
            Ok(v) => v
        };

        let mut containers_sample = Vec::<PWSampleDockerContainer>::new();
        for container in containers {
            containers_sample.push(PWSampleDockerContainer {
                id: container.Id,
                name: String::new(),
                image: String::new(),
                cpu: 0f64
            });
        }

        Some(PWSampleDocker {
            containers: containers_sample
        })
    }

    fn samples(&self) -> Arc<Mutex<Vec<PWSampleDocker>>> {
        self.samples.clone()
    }

    fn setup(&self) -> &PWSetupDocker {
        &PWSetupDocker {  }
    }
}
