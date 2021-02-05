use boa::{
    Context, 
    object::{GcObject, ObjectData},
    builtins::{
        function::{ NativeFunction, make_builtin_fn},
    }, 
    exec::Interpreter,
    Result, 
    Value,
    value::RcString,
};
use linked_hash_map::LinkedHashMap as HashMap;
use std::{borrow::Borrow, convert::TryInto, io::Read, ops::Deref};
use crate::{
    builtin::value_to_string,
};

pub fn stdin(_this: &Value, _args: &[Value], _: &mut Context) -> Result<Value> {
    let mut buf = String::default();

    match std::io::stdin().read_to_string(&mut buf) {
        Ok(_) => Ok(Value::String(RcString::from(buf))),
        _ => Ok(Value::Null),
    }
}

fn value_to_map(obj: &GcObject) -> HashMap<String, String> {
    let mut new_obj = HashMap::new();

    for (k, value) in obj.borrow().string_properties().next() {
        let value = value;
        if let v = value {
            let s = value_to_string(v.deref().borrow());
            if let Ok(key) = k.try_into() {
                new_obj.insert(key, s.unwrap());
            }
        }
    }

    new_obj
}

pub fn table(this: &Value, args: &[Value], context: &mut Context) -> Result<Value> {
    let fst = args
        .get(0)
        .ok_or(Value::Null)
        .map_err(|_| anyhow::Error::msg("Could not get 1st argument."))
        .unwrap();

    let mut table = String::default();

    if let Value::Integer(length) = fst.get_field("length", context)? {
        let arr = (0..length as usize)
            .map(|idx: usize| args.get(idx))
            .map(|row| match row.borrow() {
                Some(Value::Object(ref obj)) => value_to_map(obj),
                _ => HashMap::default(),
            })
            .collect::<Vec<_>>();

        let opt = madato::types::RenderOptions {
            headings: None,
            ..Default::default()
        };

        table = madato::mk_table(arr.as_slice(), &Some(opt));
    }

    Ok(Value::String(RcString::from(table)))
}

pub fn create_constructor(context: &Context) -> GcObject {
    let mut core = context.construct_object();

    make_builtin_fn(table, "table", &core, 1, context);
    make_builtin_fn(stdin, "stdin", &core, 1, context);

    core 
}
