use crate::builtin;
use boa::exec::Executor;
use boa::realm::Realm;

pub struct Isolate {
    pub buf: String,
}

impl Isolate {
    pub fn new() -> Self {
        let buf = String::default();
        Self { buf }
    }

    pub fn eval<A: Into<String>>(&self, script: A) -> Result<String, gc::Gc<boa::builtins::value::ValueData>> {
        let realm = Realm::create();
        let global = &realm.global_obj;

        global.set_field_slice("Violet", builtin::core::create_constructor(global));
        global.set_field_slice("Mysql", builtin::mysql::create_constructor(global));

        let engine = &mut Executor::new(realm);
        
        boa::forward_val(engine, script.into().as_str())
            .map(|s| s.to_string())
    }
}
