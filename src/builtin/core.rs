use boa::{
    Context, 
    object::{GcObject, ObjectData},
    builtins::{
        function::{ NativeFunction, make_builtin_fn},
   }, exec::Interpreter, };
use linked_hash_map::LinkedHashMap as HashMap;
use std::{borrow::Borrow, io::Read, ops::Deref};
use crate::{
    builtin::value_to_string,
};

pub fn stdin(_this: &Value, _args: &[Value], _: &mut Interpreter) -> ResultValue {
    let mut buf = String::default();

    match std::io::stdin().read_to_string(&mut buf) {
        Ok(_) => Ok(to_value(buf)),
        _ => Ok(gc::Gc::new(ValueData::Null)),
    }
}

fn value_to_map(obj: &gc::GcCell<Object>) -> HashMap<String, String> {
    let mut new_obj = HashMap::new();

    for (k, property) in obj.borrow().properties.iter() {
        let value = property.value.as_ref();

        if let Some(v) = value {
            let s = value_to_string(v.deref().borrow());
            new_obj.insert(k.clone(), s.unwrap());
        }
    }

    new_obj
}

pub fn table(_this: &Value, args: &[Value], _: &mut Interpreter) -> ResultValue {
    let args = args
        .get(0)
        .ok_or(ValueData::Null)
        .map_err(|_| anyhow::Error::msg("Could not get 1st argument."))
        .unwrap();

    let mut table = String::default();

    if let ValueData::Integer(length) = *args.get_field_slice("length").deref().borrow() {
        let arr = (0..length)
            .map(|idx| args.get_field_slice(&idx.to_string()))
            .map(|row| match row.deref().borrow() {
                &ValueData::Object(ref obj) => value_to_map(obj),
                _ => HashMap::default(),
            })
            .collect::<Vec<_>>();

        let opt = madato::types::RenderOptions {
            headings: None,
            ..Default::default()
        };

        table = madato::mk_table(arr.as_slice(), &Some(opt));
    }

    Ok(gc::Gc::new(ValueData::String(table)))
}

pub fn create_constructor(context: &Context) -> GcObject {
    let mut core = context.construct_object();

    make_builtin_fn(table, "table", &core, 1, context);
    make_builtin_fn(stdin, "stdin", &core, 1, context);

    core 
}
