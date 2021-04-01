use std::{collections::HashMap};
use boa::{
    property::PropertyDescriptor,
    class::{Class, ClassBuilder},
    gc::{Finalize, Trace},
    Context, Result, Value,
};

#[derive(Debug, Trace, Finalize)]
pub struct Violet {} 

impl Class for Violet {
    const NAME: &'static str = "Violet";

    const LENGTH: usize = 2;

    fn constructor(_this: &Value, _args: &[Value], _context: &mut Context) -> Result<Self> {
        Ok(Self {}) 
    }

    fn init(class: &mut ClassBuilder) -> Result<()> {
        class.static_method("table", 1, |_this, args, ctx| {
            if let Some(arg) = args.get(0) {
                if let Some(object) = arg.as_object() {
                    let mut table: HashMap<String, String> = HashMap::new();

                    for (key, value) in object.borrow().string_properties().next() {
                        match value { 
                            PropertyDescriptor::Data(desc) => {
                                table.insert(key.to_string(), desc.value().to_string(ctx)?.to_string());
                            },
                            _ => {}
                        }
                    }

                    return Ok(true.into()); // return `true`.
                }
            }

            Ok(false.into()) 
        });
        
        Ok(())
    }
}
