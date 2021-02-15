use std::borrow::Borrow;
use std::ops::Deref;

use boa::{
    Value,
    builtins::{
        object::Object,
    }
};
use std::collections::HashMap;

pub mod core;

//#[cfg(feature = "mysql")]
pub mod mysql;

pub fn value_to_string(data: &Value) -> anyhow::Result<String> {
    let s = match data.deref().borrow() {
        Value::String(s) => s.to_string(),
        Value::Integer(n) => n.to_string(),
        Value::Null => "<NULL>".to_string(),
        Value::Boolean(b) => {
            if *b {
                "TRUE".to_string()
            } else {
                "FALSE".to_string()
            }
        }
        _ => String::default(),
    };

    Ok(s)
}

pub fn value_to_vector(value: &Value) -> anyhow::Result<Vec<String>> {
    match value.deref().borrow() {
        &Value::Object(ref x) => {
            if x.deref().borrow().kind != ObjectKind::Array {
                return Ok(Vec::default());
            }

            if let Value::Integer(length) = *value.get_field_slice("length").deref().borrow() {
                let values = (0..length)
                    .map(|idx| value.get_field_slice(&idx.to_string()))
                    .map(|data| value_to_string(data.deref()).unwrap())
                    .collect::<Vec<String>>();

                return Ok(values);
            }

            Ok(Vec::default())
        }
        _ => Ok(Vec::default()),
    }
}

pub fn vector_to_value(this: &Value, args: Vec<Value>) -> Value {
    let length = Property::new()
        .value(to_value(args.len() as i32))
        .writable(true)
        .configurable(false)
        .enumerable(false);

    this.set_prop("length".to_string(), length);
    this.set_kind(ObjectKind::Array);

    for (n, value) in args.iter().enumerate() {
        this.set_field_slice(&n.to_string(), value.clone());
    }

    this.clone()
}

pub fn hashmap_to_value(this: &Value, args: HashMap<String, Value>) -> Value {
    let obj = Object::default();
    let object = Value::Object(gc::GcCell::new(obj));

    let length = Property::new()
        .value(to_value(args.len() as i32))
        .writable(true)
        .configurable(false)
        .enumerable(false);
    
    this.set_prop("length".to_string(), length);

    for (k, v) in args.iter() {
        object.set_field_slice(k, gc::Gc::new(v.clone()));
    }

    gc::Gc::new(object)
}
