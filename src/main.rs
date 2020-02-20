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
        .arg(
            Arg::with_name("INTERVAL")
                .short("i")
                .long("interval")
                .help("Sets a show markdown time interval(second).")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("AUTO_SAVE")
                .long("auto_save")
                .help("Sets the Auto save file when evaluate changed.")
                .takes_value(true),
        )
        .get_matches();

    let interval = value_t!(matches, "INTERVAL", u64).unwrap_or(0);
    let file = value_t!(matches, "FILE", String).unwrap();
    let auto_save = value_t!(matches, "AUTO_SAVE", bool).unwrap_or(false);
    let input = std::fs::read_to_string(file.clone()).unwrap();

    run(file, input.as_str(), auto_save, interval)?;

    Ok(())
}

fn run(file: String, input: &str, auto_save: bool, interval: u64) -> Result<(), failure::Error> {
    let app = violet::App::new(file, input.to_string(), auto_save, interval);
    app.run()
}
