mod json;

use std::collections::HashMap;

use json::Val;

fn main() {
    let mut obj = HashMap::new();
    obj.insert("greeting".to_string(), Val::Str("Moin".into()));
    obj.insert("restart".to_string(), Val::Bool(false));
    obj.insert(
        "names".to_string(),
        Val::Arr(vec![Val::Str("Robin".into()), Val::Str("Karina".into())]),
    );
    obj.insert("copy".into(), Val::Obj(obj.clone()));
    println!("{}", Val::Obj(obj));
}
