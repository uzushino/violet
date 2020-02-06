use std::ops::{Deref, Index};
use std::borrow::Borrow;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use futures::executor;

use sqlx::{ 
    row::{ Row, RowIndex },
    pool::Pool,
    mysql::{ MySqlConnection }
};

use boa::{
    builtins::{
        function::NativeFunctionData,
        value::{from_value, to_value, ResultValue, Value, ValueData},
    },
    exec::Interpreter,
};

use lazy_static::lazy_static;

use crate::{ 
    make_builtin_fn,
    builtin::value_to_vector
};

lazy_static! {
    static ref GLOBAL: Arc<Mutex<Option<Pool<MySqlConnection>>>> = 
        Arc::new(Mutex::new(None));
}

pub fn connection(_this: &Value, args: &[Value], _: &mut Interpreter) -> ResultValue {
    let database = from_value::<String>(args.get(0).expect("Could not get argument").clone()).unwrap();
    let pool = sqlx::MySqlPool::new(database.as_str());
    let ref mut conn = *GLOBAL.lock().unwrap();
    *conn = Some(executor::block_on(pool).unwrap());

    Ok(gc::Gc::new(ValueData::Null))
}

fn value_to_argument(names:&Value, types: &Value, sql: &Value) -> anyhow::Result<(Vec<String>, Vec<String>, String)> {
    let names = value_to_vector(names.deref())?;
    let types = value_to_vector(types.deref())?;
    let sql = from_value::<String>(sql.borrow().clone())
        .map_err(anyhow::Error::msg)?;

    Ok((names, types, sql))
}

use futures::prelude::*;

pub fn _query(_this: &Value, args: &[Value], _: &mut Interpreter) -> anyhow::Result<Value> {
    let args0 = args.get(0).ok_or(ValueData::Null)
        .map_err(|_| anyhow::Error::msg("Could not get 1st argument."))?;
    let args1 = args.get(0).ok_or(ValueData::Null)
        .map_err(|_| anyhow::Error::msg("Could not get 2nd argument."))?;
    let args2 = args.get(1).ok_or(ValueData::Null)
        .map_err(|_| anyhow::Error::msg("Could not get 3rd argument."))?;

    let (types, sql) = value_to_argument(args0, args1, args3)?;

    let ref mut conn = GLOBAL.lock().unwrap();
    let conn = &mut conn.as_ref().unwrap();
    let mut h: Vec<String> = Vec::new();

    let fut = sqlx::query(sql.as_str())
        .fetch(conn)
        .for_each(|row| {
            let row = row.unwrap();
            let m: HashMap<String, String> = HashMap::new();

            for i in 0..row.len()  {
                let typ = types.index(i);
                let v = match typ.as_str() {
                    "int" => i.try_get(&row).map(|i: i32| i.to_string()),
                    "string" => i.try_get(&row).map(|v: String| v.to_string()),
                    "bool" => i.try_get(&row).map(|b: bool| {
                        if b {
                            return "True".to_string();
                        }
                        "False".to_string()
                    }),
                    _ => Ok(String::default())
                };

                h.push(v.unwrap_or_default());
            }

            future::ready(())
        });

    executor::block_on(fut);

    Ok(gc::Gc::new(ValueData::Null))
}


pub fn query(_this: &Value, args: &[Value], interpreter: &mut Interpreter) -> ResultValue {
    _query(_this, args, interpreter).unwrap();
    Ok(gc::Gc::new(ValueData::Null))
}

pub fn create_constructor(global: &Value) -> Value {
    let module = ValueData::new_obj(Some(global));

    //make_builtin_fn!(table, named "table", with length 1, of module);
    make_builtin_fn!(connection, named "connection", of module);
    make_builtin_fn!(query, named "query", with length 2, of module);
 
   module 
}