use quick_js::{ Context, JsValue };
use  std::collections::HashMap;

pub struct Isolate {
  context: quick_js::Context,
  buf: Arc<Mutex<String>>,
}

use std::sync::*;

type CallbackResult = std::sync::Arc<std::sync::Mutex<std::string::String>>;

impl Isolate {
  pub fn new() -> Result<Self, quick_js::ContextError> {
    let context = Context::new()?;
    let buf = Arc::new(Mutex::new(String::default()));
    let isolate = Isolate { context, buf };

    {
      let b = Arc::clone(&isolate.buf);

      isolate.context.add_callback("println", Self::println(b.clone())).unwrap();
      isolate.context.add_callback("read_to_string", Self::read_to_string()).unwrap();
    }

    Ok(isolate)
  }

  fn println(b: CallbackResult) -> impl Fn(String) -> JsValue {
    move |a: String| {
      let mut c = b.lock().unwrap();

      (*c).push_str(a.as_str());
      (*c).push_str("\n");

      JsValue::Int(1i32)
    }
  }
  
  fn read_to_string() -> impl Fn(String) -> JsValue {
    |a: String| {
      let s = std::fs::read_to_string(a).unwrap_or_default();
      JsValue::String(s)
    }
  }

  fn table() -> impl Fn(HashMap<String, JsValue>, Vec<HashMap<String, JsValue>>) -> JsValue {

    |a: HashMap<String, JsValue>, b: Vec<HashMap<String, JsValue>>| {
      let mut rows = Vec::new();
      for m in b.iter() {
        let mut row = linked_hash_map::LinkedHashMap::new();
        for (k, v) in m {
          let s = match v {
            JsValue::String(s) => s.to_string(),
            _ => "".to_string()
          };
          row.insert(k.to_string(), s);
        }
        rows.push(row);
      }

      let opt = madato::types::RenderOptions{
        headings: None,
        ..Default::default()
      };
      let tbl = madato::mk_table(rows.as_slice(), &Some(opt));

      JsValue::Int(0)
    }
  }

  pub fn eval<A: Into<String>>(&self, script: A) -> Result<String, quick_js::ExecutionError> {
    let _ = self.context.eval_as::<String>(script.into().as_str());
    let s = self.buf.lock().unwrap();

    Ok(s.to_string())
  }
}