use std::io::Read;

use boa::{
    builtins::{
        function::NativeFunctionData,
        value::{to_value, ResultValue, Value, ValueData},
    },
    exec::Interpreter,
};

use crate::{ 
    make_builtin_fn, 
    builtin::value_to_vector 
};

pub fn stdin(_this: &Value, _args: &[Value], _: &mut Interpreter) -> ResultValue {
    let mut buf = String::default();

    match std::io::stdin().read_to_string(&mut buf) {
        Ok(_) => Ok(to_value(buf)),
        _ => Ok(gc::Gc::new(ValueData::Null))
    }
}

pub fn table(_this: &Value, args: &[Value], _: &mut Interpreter) -> anyhow::Result<Value> {
    let args = args.get(0).ok_or(ValueData::Null)
        .map_err(|_| anyhow::Error::msg("Could not get 1st argument."))?;

    let mut rows = Vec::new();
    let ary = value_to_vector(args);

    /*
    for m in b.0.iter() {
        let mut row = linked_hash_map::LinkedHashMap::new();
        for (k, v) in m.0.clone() {
            let s = match v {
                JsValue::String(s) => s.to_string(),
                JsValue::Int(i) => i.to_string(),
                JsValue::Float(f) => f.to_string(),
                _ => "".to_string(),
            };
            
            row.insert(k.to_string(), s);
        }
        rows.push(row);
    }
    */

    let opt = madato::types::RenderOptions {
        headings: None,
        ..Default::default()
    };

    let table = madato::mk_table(rows.as_slice(), &Some(opt));
    
    Ok(gc::Gc::new(ValueData::Null))
}

pub fn create_constructor(global: &Value) -> Value {
    let module = ValueData::new_obj(Some(global));

    //make_builtin_fn!(table, named "table", with length 1, of module);
    make_builtin_fn!(stdin, named "stdin", of module);
 
   module 
}