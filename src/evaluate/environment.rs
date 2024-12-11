use std::collections::HashMap;

use super::Object;

#[derive(Default, Clone)]
pub struct Env(pub HashMap<String, Object>);

pub fn find_id<'a>(id: &String, envs: &'a mut Vec<Env>) -> Option<&'a mut Env> {
    for ev in envs.iter_mut().rev() {
        if ev.0.contains_key(id) {
            return Some(ev);
        }
    }
    None
}
