use std::io::Read;
use std::{
    ops::{Deref, DerefMut},
};
use linked_hash_map::LinkedHashMap as HashMap;
use std::borrow::Borrow;
use boa::{
    builtins::{
        function::NativeFunctionData,
        value::{to_value, ResultValue, Value, ValueData},
        object::{InternalState, InternalStateCell, Object, ObjectKind, INSTANCE_PROTOTYPE, PROTOTYPE},
    },
    exec::Interpreter,
};

use crate::{ 
    make_builtin_fn, 
    builtin::value_to_vector 
};

use serde_json::{map::Map, Number as JSONNumber, Value as JSONValue};

pub fn stdin(_this: &Value, _args: &[Value], _: &mut Interpreter) -> ResultValue {
    let mut buf = String::default();

    match std::io::stdin().read_to_string(&mut buf) {
        Ok(_) => Ok(to_value(buf)),
        _ => Ok(gc::Gc::new(ValueData::Null))
    }
}

fn value_to_map(obj: &gc::GcCell<Object>) -> HashMap<String, String> {
    let mut new_obj = HashMap::new();

    for (k, v) in obj.borrow().internal_slots.iter() {
        if k != INSTANCE_PROTOTYPE {
            let s = match v.deref().borrow() {
                ValueData::String(s) => s.to_string(),
                ValueData::Number(n) => n.to_string(),
                ValueData::Null => "<NULL>".to_string(),
                ValueData::Boolean(b) => {
                    if *b {
                        "TRUE".to_string()
                    } else {
                        "FALSE".to_string()
                    }
                },
                _ => String::default(),
            };

            new_obj.insert(k.clone(), s);
        }
    }

    new_obj
}

pub fn table(_this: &Value, args: &[Value], _: &mut Interpreter) -> ResultValue {
    let args = args.get(0).ok_or(ValueData::Null)
        .map_err(|_| anyhow::Error::msg("Could not get 1st argument.")).unwrap();
    
    if let ValueData::Integer(length) = *args.get_field_slice("length").deref().borrow() {
        let arr = (0..length)
            .map(|idx| args.get_field_slice(&idx.to_string()))
            .map(|row| {
                match row.deref().borrow() {
                    &ValueData::Object(ref obj) => {
                        value_to_map(obj)
                    },
                    _ => HashMap::default()
                }
            }).collect::<Vec<_>>();
        dbg!(&arr);

        let opt = madato::types::RenderOptions {
            headings: None,
            ..Default::default()
        };
        
        let _table = madato::mk_table(arr.as_slice(), &Some(opt));

        dbg!(_table);
    }

    panic!();

    Ok(gc::Gc::new(ValueData::Null))
}

pub fn create_constructor(global: &Value) -> Value {
    let module = ValueData::new_obj(Some(global));

    make_builtin_fn!(table, named "table", with length 1, of module);
    make_builtin_fn!(stdin, named "stdin", of module);
 
   module 
}