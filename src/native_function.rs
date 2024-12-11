use std::time::{self, UNIX_EPOCH};

use crate::evaluate::Object;

pub fn clock() -> Object {
    let res = time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    Object::Number(res as f64)
}
