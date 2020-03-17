use comrak::{
    nodes::{AstNode, NodeValue},
    parse_document, Arena, ComrakOptions,
};

use pulldown_cmark::Options;
use std::fs;
use std::io::{BufWriter, Write};
use syntect::parsing::SyntaxSet;

use crate::script::Isolate;

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
where
    F: Fn(&'a AstNode<'a>),
{
    f(node);
    for c in node.children() {
        iter_nodes(c, f);
    }
}

pub fn make_options() -> Options {
    let mut opts = Options::empty();

    // opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    opts
}

pub struct Markdown {
    pub input: String,
}

impl Markdown {
    pub fn new(input: impl ToString) -> Markdown {
        Markdown {
            input: input.to_string(),
        }
    }

    pub fn parse<A>(&self, cb: A) -> Result<String, failure::Error>
    where
        A: Fn(String) -> String,
    {
        let arena = Arena::new();
        let root = parse_document(&arena, self.input.as_str(), &ComrakOptions::default());

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
        comrak::format_commonmark(root, &ComrakOptions::default(), &mut md)?;

        let text = String::from_utf8(md)?;

        Ok(text)
    }

    pub fn evaluate(&self) -> Result<String, failure::Error> {
        let isolate = Isolate::new();

        self.parse(move |script| {
            isolate.eval(script).unwrap_or_default()
        })
    }

    pub fn save_as(&self, path: &str) -> Result<(), failure::Error> {
        let text = self.evaluate()?;
        let file = fs::File::create(path)?;
        let mut f = BufWriter::new(file);

        f.write_all(text.as_bytes())?;

        Ok(())
    }

    pub fn to_tty(&mut self) -> Result<String, failure::Error> {
        let syntax_set = SyntaxSet::load_defaults_nonewlines();
        let text = self.evaluate()?;
        let parser = pulldown_cmark::Parser::new_ext(text.as_str(), make_options());

        let cd = std::env::current_dir()?;
        let mut s: Vec<u8> = Vec::default();

        match mdcat::TerminalSize::detect() {
            Some(size) => {
                mdcat::push_tty(
                    &mut s,
                    mdcat::TerminalCapabilities::none(),
                    size,
                    parser,
                    &cd,
                    mdcat::ResourceAccess::LocalOnly,
                    syntax_set,
                )?;

                String::from_utf8(s)
            }
            None => Ok(String::default()),
        }
    }

    pub fn to_html(&self) -> Result<String, failure::Error> {
        let markdown = self.evaluate()?;
        let opts = comrak::ComrakOptions {
            ext_table: true,
            ext_tasklist: true,
            ..Default::default()
        };
        
        let s = comrak::markdown_to_html(markdown.as_str(), &opts);

        Ok(s)
    }
}
