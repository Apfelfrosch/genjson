use std::{collections::HashMap, fmt::Display};

#[derive(PartialEq, Clone)]
pub enum Val {
    Null,
    Str(String),
    Num(String),
    Bool(bool),
    Arr(Vec<Val>),
    Obj(HashMap<String, Val>),
}

pub fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('/', "\\/")
        .replace('\n', "\\n")
        .replace('\t', "\\t")
        .replace('\r', "\\r")
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Val::Null => write!(f, "null"),
            Val::Str(s) => write!(f, "\"{}\"", json_escape(s)),
            Val::Num(s) => write!(f, "{}", s),
            Val::Bool(b) => {
                if *b {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            Val::Arr(v) => {
                let mut res = String::new();
                for (i, val) in v.iter().enumerate() {
                    res.push_str(&format!("{}", val));
                    if i < v.len() - 1 {
                        res.push(',');
                    }
                }
                write!(f, "[{}]", res)
            }
            Val::Obj(m) => {
                let mut res = String::from("{");
                let mut needs_comma = false;
                for (k, v) in m {
                    if needs_comma {
                        res.push(',');
                    } else {
                        needs_comma = true;
                    }
                    res.push_str(&format!("\"{}\": {}", json_escape(k), v));
                }
                res.push('}');
                write!(f, "{}", res)
            }
        }
    }
}
