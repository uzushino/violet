
use clap::{Arg, crate_version};
use std::io::Read;

fn main() -> Result<(), failure::Error> {
    env_logger::init();

    let matches = clap::App::new("")
        .version(crate_version!())
        .arg(
            Arg::with_name("FILE")
        ).get_matches();
    
    let input: String = match matches.value_of("FILE") {
        Some(file) => std::fs::read_to_string(file)?,
        None => {
            let mut buf = Vec::default();
            std::io::stdin().read_to_end(&mut buf)?;
            String::from_utf8(buf)?
        },
    };

    run(input.as_str())?;

    Ok(())
}

fn run(input: &str) -> Result<(), failure::Error> {
    let app = violet::App::new(input.to_string());
    app.run()
}