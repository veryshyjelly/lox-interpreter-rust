use std::{collections::HashMap, fmt::Display};

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

impl Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, v) in &self.0 {
            writeln!(f, "key: {}, value: {}", k, v)?;
        }
        Ok(())
    }
}

impl Env {
    pub fn to_string_vec(v: &Vec<Env>) -> String {
        v.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}
