use clap::{crate_version, value_t, Arg};

fn main() -> Result<(), failure::Error> {
    env_logger::init();

    let matches = clap::App::new("")
        .version(crate_version!())
        .arg(
            Arg::with_name("FILE")
                .required(true)
                .help("Sets a render markdown."),
        )
        .get_matches();

    let file = value_t!(matches, "FILE", String).unwrap();
    let input = std::fs::read_to_string(file.clone()).unwrap();
    let markdown = run(input.as_str())?;

    println!("{}", markdown);

    Ok(())
}

fn run(input: &str) -> Result<String, failure::Error> {
    let app = violet::App::new(input.to_string());
    app.run()
}
