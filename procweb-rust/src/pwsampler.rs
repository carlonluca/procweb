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

use std::sync::{Arc, Mutex};

///
/// Trait for sampler classes. Every sampler has a code to sample
/// data and a method to return the samples.
/// 
pub trait PWSampler<S>: Sync + Send {
    fn sample(&mut self) -> Option<S>;
    fn samples(&self) -> Arc<Mutex<Vec<S>>>;
}
