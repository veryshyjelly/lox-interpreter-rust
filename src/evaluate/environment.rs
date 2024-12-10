use std::collections::HashMap;

use super::Object;

pub struct Env(pub HashMap<String, Object>);
