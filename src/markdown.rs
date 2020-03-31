use comrak::{
    nodes::{AstNode, NodeValue},
    parse_document, Arena, ComrakOptions,
};

use pulldown_cmark::Options;
use std::fs;
use std::io::{BufWriter, Write};

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

    pub fn parse<A>(&self, cb: A) -> anyhow::Result<String>
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

    pub fn evaluate(&self) -> anyhow::Result<String> {
        self.parse(|script| {
            let isolate = Isolate::new();

            isolate.eval(script).unwrap_or_default()
        })
    }

    pub fn save_as(&self, path: &str) -> anyhow::Result<()> {
        let file = fs::File::create(path)?;
        let text = self.evaluate()?;
        let mut f = BufWriter::new(file);

        f.write_all(text.as_bytes())?;

        Ok(())
    }

    pub fn to_html(&self) -> anyhow::Result<String> {
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
