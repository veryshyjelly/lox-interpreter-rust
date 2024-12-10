use std::collections::HashMap;

use super::Object;

#[derive(Default)]
pub struct Env(pub HashMap<String, Object>);
