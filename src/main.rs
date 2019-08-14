
use clap::{Arg, crate_version};
use std::io::Read;

fn main() -> Result<(), failure::Error> {
    env_logger::init();

    let matches = clap::App::new("")
        .version(crate_version!())
        .arg(
            Arg::with_name("FILE")
        )
        .arg(
            Arg::with_name("INTERVAL")
        )
        .get_matches();

    let interval = matches
        .value_of("INTERVAL")
        .and_then(|i: &str| i.to_owned().parse().ok())
        .unwrap_or(0u64);

    let input: String = match matches.value_of("FILE") {
        Some(file) => std::fs::read_to_string(file)?,
        None => {
            let mut buf = Vec::default();
            std::io::stdin().read_to_end(&mut buf)?;
            String::from_utf8(buf)?
        },
    };

    run(input.as_str(), interval)?;

    Ok(())
}

fn run(input: &str, interval: u64) -> Result<(), failure::Error> {
    let app = violet::App::new(
        input.to_string(),
        interval,
    );
    app.run()
}