use std::collections::HashMap;

use super::Object;

#[derive(Default, Clone)]
pub struct Env(pub HashMap<String, Object>);
