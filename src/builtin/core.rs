use std::io::Read;

use boa::{
    builtins::{
        function::NativeFunctionData,
        value::{to_value, ResultValue, Value, ValueData},
    },
    exec::Interpreter,
};

use crate::make_builtin_fn;

/*
pub fn table(_this: &Value, _args: &[Value], _: &mut Interpreter) -> ResultValue {
    Ok(to_value(1234))
}
*/

pub fn stdin(_this: &Value, _args: &[Value], _: &mut Interpreter) -> ResultValue {
    let mut buf = String::default();

    match std::io::stdin().read_to_string(&mut buf) {
        Ok(_) => Ok(to_value(buf)),
        _ => Ok(gc::Gc::new(ValueData::Null))
    }
}

pub fn create_constructor(global: &Value) -> Value {
    let module = ValueData::new_obj(Some(global));

    //make_builtin_fn!(table, named "table", with length 1, of module);
    make_builtin_fn!(stdin, named "stdin", of module);
 
   module 
}