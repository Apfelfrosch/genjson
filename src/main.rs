mod json;

use std::collections::HashMap;
use std::env;

fn main() {
    let mut to_set: HashMap<String, Vec<String>> = HashMap::new();
    let mut last_key: Option<String> = None;
    for (i, arg) in env::args().skip(1).enumerate() {
        if arg.starts_with('-') && !arg.starts_with("\\-") {
            let key = &arg[1..].to_string();
            if key.is_empty() {
                eprintln!("Error (Arg {i}): empty key");
                return;
            }
            last_key = Some(key.to_string());
        } else if let Some(key) = &last_key {
            if let Some(values) = to_set.get_mut(key) {
                values.push(arg);
            } else {
                to_set.insert(key.to_string(), vec![arg]);
            }
        } else {
            eprintln!("Error (Arg {i}): No key provived for {}", arg);
            return;
        }
    }
    let mut json_obj: HashMap<String, json::Val> = HashMap::new();
    for (k, v) in to_set {
        let json_val: json::Val = if v.len() == 1 {
            json::Val::parse_literal(&v[0]).expect("Could not parse literal")
        } else {
            let v = v
                .iter()
                .map(|s| json::Val::parse_literal(s).expect("Could not parse literal"))
                .collect();
            json::Val::Arr(v)
        };

        if !insert_into_object(&k, json_val, &mut json_obj) {
            return;
        }
    }
    println!("{}", json::Val::Obj(json_obj));
}

fn insert_into_object(key: &str, val: json::Val, obj: &mut HashMap<String, json::Val>) -> bool {
    let mut path = Vec::new();
    let mut current_path = String::new();
    let mut last_char = 'a';
    for (i, c) in key.chars().enumerate() {
        if c == '.' {
            if last_char == '.' || i == 0 {
                eprintln!("Error: empty path segment");
                return false;
            }

            if last_char != '\\' {
                path.push(std::mem::take(&mut current_path));
                last_char = c;
                continue;
            }
        }
        last_char = c;
        current_path.push(c);
    }
    if current_path.is_empty() {
        eprintln!("Error: empty path segment");
        return false;
    } else {
        path.push(current_path);
    }

    if path.len() == 1 {
        obj.insert(path[0].clone(), val);
    } else {
        let mut current_obj = obj;
        for component in path[..(path.len() - 1)].iter() {
            if !current_obj.contains_key(component) {
                current_obj.insert(component.clone(), json::Val::Obj(HashMap::new()));
            }
            current_obj = if let json::Val::Obj(x) = current_obj.get_mut(component).unwrap() {
                x
            } else {
                eprintln!(
                    "Error: not an object in component {} in path {:?}",
                    component, path
                );
                return false;
            };
        }
        current_obj.insert(path[path.len() - 1].clone(), val);
    }

    true
}
