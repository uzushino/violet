use std::borrow::Borrow;
use std::ops::Deref;

use boa::builtins::{object::ObjectKind, value::ValueData};

pub mod core;

//#[cfg(feature = "mysql")]
pub mod mysql;

#[macro_export]
macro_rules! make_builtin_fn {
    ($fn:ident, named $name:expr, with length $l:tt, of $p:ident) => {
        let $fn = to_value($fn as NativeFunctionData);
        $fn.set_field_slice("length", to_value($l));
        $p.set_field_slice($name, $fn);
    };

    ($fn:ident, named $name:expr, of $p:ident) => {
        make_builtin_fn!($fn, named $name, with length 0, of $p);
    };
}

pub fn value_to_string(data: &ValueData) -> anyhow::Result<String> {
    let s = match data.deref().borrow() {
        ValueData::String(s) => s.to_string(),
        ValueData::Number(n) => n.to_string(),
        ValueData::Null => "<NULL>".to_string(),
        ValueData::Boolean(b) => {
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

pub fn value_to_vector(value: &ValueData) -> anyhow::Result<Vec<String>> {
    match value.deref().borrow() {
        &ValueData::Object(ref x) => {
            if x.deref().borrow().kind != ObjectKind::Array {
                return Ok(Vec::default());
            }

            if let ValueData::Integer(length) = *value.get_field_slice("length").deref().borrow() {
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
