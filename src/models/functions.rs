use std::sync::{Arc, Mutex};

use serde::Serialize;

#[derive(Serialize)]
pub struct Output {
    pub result: i32,
}


