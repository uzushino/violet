use std::collections::HashMap;

use boa::{
    class::{Class, ClassBuilder},
    gc::{Finalize, Trace},
    property::Attribute,
    Context, Result, Value,
};

#[derive(Debug, Trace, Finalize)]
pub struct Violet {} 

impl Class for Violet {
    const NAME: &'static str = "Violet";

    const LENGTH: usize = 2;

    fn constructor(_this: &Value, args: &[Value], context: &mut Context) -> Result<Self> {
        Ok(Self {}) 
    }

    fn init(class: &mut ClassBuilder) -> Result<()> {
        class.static_method("table", 1, |_this, args, _ctx| {
            if let Some(arg) = args.get(0) {
                if let Some(object) = arg.as_object() {
                    if object.is::<HashMap<String, String>>() {
                        return Ok(true.into()); // return `true`.
                    }
                }
            }

            Ok(false.into()) // otherwise `false`.
        });
        
        class.static_property(
            "staticProperty",
            "Im a static property",
            Attribute::WRITABLE | Attribute::ENUMERABLE | Attribute::PERMANENT,
        );

        Ok(())
    }
}
