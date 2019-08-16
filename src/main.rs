
use clap::{Arg, crate_version};
use std::io::Read;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::sync::mpsc;
use std::thread;
use futures::future::Future;

fn main() -> Result<(), failure::Error> {
    env_logger::init();

    let matches = clap::App::new("")
        .version(crate_version!())
        .arg(
            Arg::with_name("FILE")
                .required(true)
                .help("Sets a render markdown.")
        )
        .arg(
            Arg::with_name("INTERVAL")
                .short("i")
                .long("interval")
                .help("Sets a show markdown time interval(second).")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("SERVER")
                .long("server")
                .short("s")
                .help("Running http server.")
        )
        .arg(
            Arg::with_name("PORT")
                .long("port")
                .short("p")
                .default_value_if("SERVER", None, "8080")
                .takes_value(true)
                .help("Sets a http server port.")
        )
        .get_matches();

    let interval = matches
        .value_of("INTERVAL")
        .and_then(|i: &str| i.to_owned().parse().ok())
        .unwrap_or(0u64);

    let input = match matches.value_of("FILE") {
        Some(file) => std::fs::read_to_string(file)?,
        None => {
            let mut buf = Vec::default();
            std::io::stdin().read_to_end(&mut buf)?;
            String::from_utf8(buf)?
        },
    };

    if matches.value_of("SERVER").is_some() {
        let port = matches.value_of("PORT")
            .and_then(|v| v.parse().ok())
            .unwrap_or(8088);
        run_with_server(input.as_str(), interval, port)?;
    } else {
        run(input.as_str(), interval)?;
    }

    Ok(())
}

fn run_with_server(input: &str, interval: u64, port: u64) -> Result<(), failure::Error> {
    let app = violet::App::new(
        input.to_string(),
        interval,
    );

    let prompt = app.prompt.clone();
    thread::spawn(move || {
        let sys = actix_rt::System::new("http-server");
        
        HttpServer::new(move || {
            let prompt = prompt.clone();

            App::new().route("/", web::get().to(move || {
                let a = prompt.lock().unwrap();
                a.evaluate()
            }))
        })
        .bind(format!("127.0.0.1:{}", port))
        .unwrap()
        .start();

        let _ = sys.run();
    });
    
    app.run()
}

fn run(input: &str, interval: u64) -> Result<(), failure::Error> {
    let app = violet::App::new(
        input.to_string(),
        interval,
    );
    app.run()
}