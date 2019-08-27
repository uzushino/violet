use quick_js::{ Context, JsValue, ValueError };
use std::collections::HashMap;
use std::convert::TryFrom;

pub struct Isolate {
  context: quick_js::Context,
  buf: Arc<Mutex<String>>,
}

use std::sync::*;

type CallbackResult = std::sync::Arc<std::sync::Mutex<std::string::String>>;

struct MapWrap(pub HashMap<String, JsValue>);

impl TryFrom<JsValue> for MapWrap {
  type Error = ValueError;

  fn try_from(value: JsValue) -> Result<Self, Self::Error> {
      match value {
        JsValue::Object(v) => Ok(MapWrap(v)),
        _ => Err(ValueError::UnexpectedType)
      }
  }
}

struct VecMapWrap(pub Vec<MapWrap>);

impl TryFrom<JsValue> for VecMapWrap {
  type Error = ValueError;

  fn try_from(value: JsValue) -> Result<Self, Self::Error> {
      match value {
        JsValue::Array(a) => {
          let v = a
            .into_iter()
            .map(|v| TryFrom::try_from(v).unwrap())
            .collect::<Vec<MapWrap>>();
          Ok(VecMapWrap(v))
        }
        _ => Err(ValueError::UnexpectedType)
      }
  }
}

struct VecString(pub Vec<String>);

impl TryFrom<JsValue> for VecString {
  type Error = ValueError;

  fn try_from(value: JsValue) -> Result<Self, Self::Error> {
      match value {
        JsValue::Array(a) => {
          let v = a
            .into_iter()
            .map(|v| v.into_string().unwrap())
            .collect::<Vec<String>>();
          Ok(VecString(v))
        }
        _ => Err(ValueError::UnexpectedType)
      }
  }
}

impl Isolate {
  pub fn new() -> Result<Self, quick_js::ContextError> {
    let context = Context::new()?;
    let buf = Arc::new(Mutex::new(String::default()));
    let isolate = Isolate { context, buf };

    {
      let b = Arc::clone(&isolate.buf);

      isolate.context.add_callback("println", Self::println(b.clone())).unwrap();
      isolate.context.add_callback("read_to_string", Self::read_to_string()).unwrap();
      isolate.context.add_callback("table", Self::table()).unwrap();
      isolate.context.add_callback("command", Self::run_command()).unwrap();
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
  
  fn run_command() -> impl Fn(String, VecString) -> JsValue {
    |cmd: String, args: VecString| {
      let out = std::process::Command::new(cmd)
            .args(args.0.as_slice())
            .output()
            .unwrap()
            .stdout;

      JsValue::String(String::from_utf8(out).unwrap()) 
    }
  }

  fn table() -> impl Fn(VecMapWrap) -> JsValue {
    |b: VecMapWrap| {
      let mut rows = Vec::new();
      for m in b.0.iter() {
        let mut row = linked_hash_map::LinkedHashMap::new();

        for (k, v) in m.0.clone() {
          let s = match v {
            JsValue::String(s) => s.to_string(),
            JsValue::Int(i) => i.to_string(),
            JsValue::Float(f) => f.to_string(),
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
      
      let table = madato::mk_table(
        rows.as_slice(), 
        &Some(opt)
      );

      JsValue::String(format!("\n{}\n", table))
    }
  }

  pub fn eval<A: Into<String>>(&self, script: A) -> Result<String, quick_js::ExecutionError> {
    let _ = self.context.eval_as::<String>(script.into().as_str());
    let s = self.buf.lock().unwrap();

    Ok(s.to_string())
  }
}