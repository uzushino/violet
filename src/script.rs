use crate::builtin;
use boa::Context;

pub struct Isolate {
    pub buf: String,
}

impl Isolate {
    pub fn new() -> Self {
        let buf = String::default();
        Self { buf }
    }

    pub fn eval<A: Into<String>>(&self, script: A) -> Result<String, gc::Gc<boa::builtins::value::ValueData>> {
        let mut context = Context::new();
        context.register_global_class::<builtin::core::Violet>();
        let global = &realm.global_object;

        global.("Violet", builtin::core::create_constructor(global));
        global.insert("Mysql", builtin::mysql::create_constructor(global));

        let engine = &mut Executor::new(realm);
        
        boa::forward_val(engine, script.into().as_str())
            .map(|s| s.to_string())
    }
}
