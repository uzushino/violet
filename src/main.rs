
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
            Arg::with_name("SERVER")
                .long("server")
                .short("s")
                .help("Running http server.")
        )
        .arg(
            Arg::with_name("PORT")
                .long("port")
                .short("p")
                .default_value_if("SERVER", None, "8088")
                .takes_value(true)
                .help("Sets a http server port.")
        )
        .get_matches();

    let interval = value_t!(matches, "INTERVAL", u64).unwrap_or(0);
    let input = match matches.value_of("FILE") {
        Some(file) => std::fs::read_to_string(file)?,
        None => {
            let mut buf = Vec::default();
            std::io::stdin().read_to_end(&mut buf)?;
            String::from_utf8(buf)?
        },
    };

    if matches.is_present("SERVER") {
        let port = value_t!(matches, "PORT", u64).unwrap_or(8088);
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

    let p = app.prompt.clone();

    let (tx, rx) = std::sync::mpsc::channel();
    let th = thread::spawn(move || {
        let _ = actix_rt::System::new("http-server");
        let bind_addr = format!("127.0.0.1:{}", port);

        let addr = HttpServer::new(move || {
            let p1 = p.clone();
            App::new().route("/", web::get().to(move || {
                let markdown = {
                    let a = p1.lock().unwrap();
                    a.evaluate().unwrap()
                };

                let parser = pulldown_cmark::Parser::new(markdown.as_str());
                let mut buf = String::new();
                pulldown_cmark::html::push_html(&mut buf, parser);

                HttpResponse::Ok()
                    .content_type("text/html")
                    .body(buf)
            }))
        })
        .bind(bind_addr)
        .unwrap()
        .start();

        let _ = tx.send(addr);
    });

    app.run()?;
    
    let _ = rx.recv()?.stop(true).wait();
    let _ = th.join();

    Ok(())
}

fn run(input: &str, interval: u64) -> Result<(), failure::Error> {
    let app = violet::App::new(
        input.to_string(),
        interval,
    );

    app.run()
}