use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::Read;
use std::sync::*;

use boa::realm::Realm;  
use boa::exec::Executor;

use boa::{
    builtins::{
        function::NativeFunctionData,
        object::{Object, ObjectKind, PROTOTYPE},
        value::{to_value, ResultValue, Value, ValueData},
    },
    exec::Interpreter,
};

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

pub fn table(_: &Value, args: &[Value], _: &mut Interpreter) -> ResultValue {
    Ok(to_value(1234))
}

pub fn stdin(_: &Value, args: &[Value], _: &mut Interpreter) -> ResultValue {
    let mut buf = String::default();

    match std::io::stdin().read_to_string(&mut buf) {
        Ok(_) => Ok(to_value(buf)),
        _ => Ok(gc::Gc::new(ValueData::Null))
    }
}


        /*
        let engine = &mut Executor::new(realm);
        let a = boa::forward_val(engine, "Violet.table(1)");
        */

fn create_constructor(global: &Value) -> Value {
    let module = ValueData::new_obj(Some(global));

    make_builtin_fn!(table, named "table", with length 1, of module);
    make_builtin_fn!(stdin, named "stdin", of module);
 
   module 
}


pub struct Isolate {
    pub buf: Arc<Mutex<String>>,
}

impl Isolate {
    pub fn new() -> Self {
        let buf = Arc::new(Mutex::new(String::default()));
         
        Self {
            buf
        }
    }


    pub fn eval<A: Into<String>>(&self, script: A) -> Result<String, ()> {
        let realm = Realm::create();
        let global = &realm.global_obj;

        global.set_field_slice(
            "Violet", create_constructor(global)
        );

        let engine = &mut Executor::new(realm);
        let a = boa::forward_val(engine, script.into().as_str());

        Ok(a.unwrap().to_string())
    }
}