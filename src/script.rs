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
        
        context.eval(script)
    }
}
