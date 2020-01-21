use std::io::Read;
use futures::Future;
use sqlx::mysql;
use futures::executor;

use boa::{
    builtins::{
        function::NativeFunctionData,
        value::{to_value, ResultValue, Value, ValueData},
    },
    exec::Interpreter,
};

use crate::make_builtin_fn;

pub fn connection(_this: &Value, _args: &[Value], _: &mut Interpreter) -> ResultValue {
    let pool = sqlx::MySqlPool::new("");
    let pool = executor::block_on(pool).unwrap();
    
    Ok(gc::Gc::new(ValueData::Number(&pool as *const _ as usize as f64)))
}

pub fn create_constructor(global: &Value) -> Value {
    let module = ValueData::new_obj(Some(global));

    //make_builtin_fn!(table, named "table", with length 1, of module);
    make_builtin_fn!(connection, named "connection", of module);
 
   module 
}