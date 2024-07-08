use std::{collections::HashMap, fmt::Display};

#[derive(PartialEq, Clone, Debug)]
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

impl Val {
    pub fn parse_literal(s: &str) -> Option<Val> {
        if s.len() >= 2 && s.starts_with('"') && s.ends_with('"') {
            Some(Val::Str(s[1..(s.len() - 1)].to_string()))
        } else if s == "null" {
            Some(Val::Null)
        } else if s == "true" {
            Some(Val::Bool(true))
        } else if s == "false" {
            Some(Val::Bool(false))
        } else if s.is_empty() {
            Some(Val::Str(String::new()))
        } else if s.chars().next().unwrap().is_ascii_digit()
            && s.chars().filter(|s| *s == '.').count() <= 1
        {
            Some(Val::Num(s.to_string()))
        } else {
            Some(Val::Str(s.to_string()))
        }
    }
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
                    res.push_str(&format!("\"{}\":{}", json_escape(k), v));
                }
                res.push('}');
                write!(f, "{}", res)
            }
        }
    }
}
