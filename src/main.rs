extern crate geopost;
extern crate clap;
extern crate hyper;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use std::fs::File;
use std::net::SocketAddr;

use clap::{App, Arg};
use slog::Drain;

const COUNTRY_CSV_HELP: &str = r#"
Path to CSV describing zip codes for given country.
This should be set in <country_code>=<file_path> format."#;

fn load_countries<'a, I>(handler: &mut geopost::Server, specs: I, logger: &slog::Logger)
where
    I: Iterator<Item = &'a str>,
{
    for country_spec in specs {
        let parts: Vec<_> = country_spec.splitn(2, "=").collect();

        if parts.len() != 2 {
            panic!("Invalid country specification, required format <country_code>=<file_path>")
        }

        let country = parts[0];
        let path = parts[1];

        let file = File::open(path).expect("Cannot open file");
        let codes = geopost::zip_codes::ZipCodes::load_from(file).expect("Invalid file format");

        debug!(logger, "Added entry"; "country_code" => country, "file" => path);

        handler.add(country, codes);
    }
}

fn main() {
    let matches = App::new("GeoPost")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("country-csv")
                .long("country-csv")
                .short("c")
                .value_name("COUNTRY_CSV")
                .required(true)
                .multiple(true)
                .help(COUNTRY_CSV_HELP),
        )
        .arg(
            Arg::with_name("listen")
                .long("listen")
                .value_name("address")
                .short("l")
                .help("Address with port to listen on. By default 0.0.0.0:5000"),
        )
        .get_matches();

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, o!());

    let addr: SocketAddr = matches
        .value_of("listen")
        .unwrap_or("0.0.0.0:5000")
        .parse()
        .expect("Invalid listen specification");

    let mut handler = geopost::Server::new(logger.new(o!("server" => "true")));
    load_countries(
        &mut handler,
        matches.values_of("country-csv").unwrap(),
        &logger,
    );

    info!(
        logger,
        "Start server";
            "addr" => format!("{}", addr),
            "ip" => format!("{}", addr.ip()),
            "port" => addr.port(),
    );
    geopost::start(handler, addr);
}
