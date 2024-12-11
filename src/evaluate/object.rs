use std::fmt::Display;
use std::sync::Arc;

use super::RuntimeError;

#[derive(Clone)]
pub enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
    Object(String),
    Function(ExFun),
    Nil,
}

pub type ExFun = Arc<dyn Fn(Vec<Object>) -> Result<Object, RuntimeError> + Send + Sync>;

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::Object::*;
        let v = match self {
            Number(n) => n.to_string(),
            String(s) => s.clone(),
            Boolean(v) => v.to_string(),
            Object(v) => format!("Object {v}"),
            Function(v) => format!("function"),
            Nil => "nil".into(),
        };
        write!(f, "{v}")
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Object::Number(n) => {
                if let Some(m) = other.get_number() {
                    *n == m
                } else {
                    false
                }
            }
            Object::String(s) => {
                if let Some(t) = other.get_string() {
                    s == &t
                } else {
                    false
                }
            }
            Object::Boolean(b) => {
                if let Some(c) = other.get_bool() {
                    b == &c
                } else {
                    false
                }
            }
            Object::Object(o) => match other {
                Object::Object(d) => o == d,
                _ => false,
            },
            Object::Nil => match other {
                Object::Nil => true,
                _ => false,
            },
            Object::Function(_) => false,
        }
    }
}

impl Object {
    pub fn get_number(&self) -> Option<f64> {
        match self {
            Object::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn get_string(&self) -> Option<String> {
        match self {
            Object::String(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn get_bool(&self) -> Option<bool> {
        match self {
            Object::Boolean(v) => Some(*v),
            _ => None,
        }
    }

    pub fn get_function(&self) -> Option<ExFun> {
        match self {
            Object::Function(v) => Some(v.clone()),
            _ => None,
        }
    }
}
