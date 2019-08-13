use comrak::{ 
    parse_document, 
    Arena, 
    ComrakOptions,

    nodes::{ AstNode, NodeValue }
};

use pulldown_cmark::Options;
use std::io::Write;
use syntect::parsing::SyntaxSet;

use crate::script::Isolate;

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F) where F : Fn(&'a AstNode<'a>) {
    f(node);
    for c in node.children() {
        iter_nodes(c, f);
    }
}

fn make_options() -> Options {
    let mut opts = Options::empty();

    //opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    opts
}

pub struct Markdown<T: Write> {
    pub stdout: T,
    pub input: String,
}

impl<T: Write + Send > Markdown<T> {
  pub fn new(stdout: T, input: String) -> Markdown<T> {
    Markdown {
        stdout,
        input
    }
  }
    
  pub fn flush(&mut self) -> Result<(), failure::Error> {
    self.stdout.flush()?;

    Ok(())
  }

  pub fn parse<A>(&self, input: &str, cb: A) ->  Result<String, failure::Error> where A: Fn(String) -> String {
    let arena = Arena::new();

    let root = parse_document(
        &arena,
        input,
        &ComrakOptions::default());

    iter_nodes(root, &|node| {
        let mut data = node.data.borrow_mut();
       
        match &mut data.value {
            &mut NodeValue::CodeBlock(ref mut code) => {
                if code.fenced {
                    let orig = std::mem::replace(&mut code.literal, vec![]);
                    let script = String::from_utf8(orig).unwrap();
                    let result = cb(script);

                    std::mem::replace(&mut data.value, NodeValue::Text(result.into()));
                }
            }
            _ => {}
        }
    });

    let mut md = vec![];

    comrak::format_commonmark(
        root, 
        &ComrakOptions::default(), 
        &mut md 
    )?;

    Ok(String::from_utf8(md)?)
  }

  pub fn render(&mut self) -> Result<(), failure::Error> {
    let syntax_set = SyntaxSet::load_defaults_nonewlines();
    let isolate = Isolate::new()?;
    let changed = self.parse(self.input.as_str(), |script| {
        let a = isolate.eval(script).unwrap();
        a
    })?;
    let parser = pulldown_cmark::Parser::new_ext(
        changed.as_str(),
        make_options()
    );
    let cd = std::env::current_dir()?;
    let size = mdcat::TerminalSize::detect().unwrap_or_default();

    write!(self.stdout, "{}", termion::cursor::Goto(1, 1))?;
    write!(self.stdout, "{}", termion::clear::All)?;

    let mut s: Vec<u8> = Vec::default();

    mdcat::push_tty(
        &mut s, 
        mdcat::TerminalCapabilities::ansi(), 
        size,
        parser, 
        &cd, 
        mdcat::ResourceAccess::LocalOnly,
        syntax_set,
    )?;

    // Change \n to \n\r for New line in raw_mode.
    let t = String::from_utf8(s).unwrap()
        .split('\n')
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join("\n\r");

    write!(self.stdout, "{}", t)?;

    Ok(())
  }
}