extern crate csv;
extern crate futures;
extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate slog;

pub mod zip_codes;
pub mod server;

pub use server::Server;
pub use zip_codes::ZipCodes;

use std::net::SocketAddr;
use std::sync::Arc;

use hyper::server::Http;

pub fn start(handler: Server, addr: SocketAddr) -> Result<(), hyper::Error> {
    let handler = Arc::new(handler);

    let server = Http::new().bind(&addr, move || Ok(handler.clone()))?;
    server.run()
}
