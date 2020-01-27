use std::sync::*;

use boa::realm::Realm;  
use boa::exec::Executor;
use crate::builtin;

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
            "Violet", builtin::core::create_constructor(global)
        );
        global.set_field_slice(
            "Mysql", builtin::mysql::create_constructor(global)
        );

        let engine = &mut Executor::new(realm);
        let a = boa::forward_val(engine, script.into().as_str());

        dbg!(&a);

        Ok(a.unwrap().to_string())
    }
}