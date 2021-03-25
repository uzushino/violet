use std::borrow::Borrow;
use std::ops::Deref;
use boa::Value;

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
