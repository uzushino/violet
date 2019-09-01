
use clap::{Arg, crate_version, value_t};
use std::io::Read;
use actix_web::{web, App, HttpServer, HttpResponse};
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
            Arg::with_name("AUTO_SAVE")
                .long("auto_save")
                .help("Sets the Auto save file when evaluate changed.")
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
                .default_value_if("SERVER", None, "8088")
                .takes_value(true)
                .help("Sets a http server port.")
        )
        .arg(
            Arg::with_name("HOST")
                .long("host")
                .default_value_if("SERVER", None, "127.0.0.1")
                .takes_value(true)
                .help("Sets a http server address.")
        )
        .get_matches();

    let interval = value_t!(matches, "INTERVAL", u64).unwrap_or(0);
    let file = value_t!(matches, "FILE", String).unwrap();
    let input = std::fs::read_to_string(file.clone()).unwrap();

    if matches.is_present("SERVER") {
        let port = value_t!(matches, "PORT", u64).unwrap_or(8088);
        let host = value_t!(matches, "HOST", String).unwrap_or("127.0.0.1".to_owned());
        let bind_addr = format!("{}:{}", host, port);

        run_with_server(file, input.as_str(), interval, bind_addr)?;
    } else {
        run(file, input.as_str(), interval)?;
    }

    Ok(())
}

fn run_with_server(file: String, input: &str, interval: u64, bind_addr: String) -> Result<(), failure::Error> {
    let app = violet::App::new(
        file,
        input.to_string(),
        interval,
    );

    let p = app.prompt.clone();
    let (tx, rx) = std::sync::mpsc::channel();

    thread::spawn(move || {
        let sys = actix_rt::System::new("http-server");

        let addr = HttpServer::new(move || {
            App::new()
            .data(p.clone())
            .route("/", web::get().to(|data: web::Data<violet::AppData>| {
                let markdown = {
                    let a = data.lock().unwrap();
                    a.evaluate().unwrap()
                };
                let opts = comrak::ComrakOptions {
                    ext_table: true,
                    ..Default::default()
                };
                let buf = comrak::markdown_to_html(
                    markdown.as_str(), 
                    &opts
                );

                HttpResponse::Ok()
                    .content_type("text/html")
                    .body(buf)
            }))
        })
        .bind(bind_addr)
        .unwrap()
        .shutdown_timeout(1)
        .start();

        let _ = tx.send(addr);
        let _ = sys.run();
    });

    app.run()?;

    rx.recv()?.stop(true).wait().unwrap();

    app.prompt.lock().and_then(|f| {
        // manually out of raw mode.
        f.stdout.suspend_raw_mode().unwrap();
        
        Ok(())
    }).unwrap();

    Ok(())
}

fn run(file: String, input: &str, interval: u64) -> Result<(), failure::Error> {
    let app = violet::App::new(
        file,
        input.to_string(),
        interval,
    );

    app.run()
}