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

    pub fn eval(&self, script: &str) -> Result<String, boa::Value> {
        let mut context = Context::new();

        context.register_global_class::<builtin::core::Violet>()?;
        
        let result = context.eval(script)?;
        result.to_string(&mut context).map(|s| s.to_string())
    }
}
