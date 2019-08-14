use quick_js::Context;

pub struct Isolate {
  context: quick_js::Context,
  buf: Arc<Mutex<String>>,
}

use std::sync::*;

impl Isolate {
  pub fn new() -> Result<Self, quick_js::ContextError> {
    let context = Context::new()?;
    let buf = Arc::new(Mutex::new(String::default()));
    let isolate = Isolate { context, buf };

    {
      let b = Arc::clone(&isolate.buf);

      isolate.context.add_callback("println", move |a: String| {
        let mut c = b.lock().unwrap();
        (*c).push_str(a.as_str());
        (*c).push_str("\n");
        1i32
      }).unwrap();
    }

    Ok(isolate)
  }

  pub fn eval<A: Into<String>>(&self, script: A) -> Result<String, quick_js::ExecutionError> {
    self.context.eval_as::<String>(script.into().as_str());
    let s = self.buf.lock().unwrap();
    Ok(s.to_string())
  }
}