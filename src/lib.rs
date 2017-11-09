extern crate csv;
extern crate futures;
extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate slog;
extern crate signal;
extern crate nix;

pub mod zip_codes;
pub mod server;

pub use server::Server;
pub use zip_codes::ZipCodes;

use std::net::SocketAddr;
use std::sync::Arc;

use hyper::server::Http;
use futures::sync::oneshot;
use futures::Future;

pub fn start<F>(srv: Server, addr: SocketAddr, stop: F) -> Result<(), hyper::Error>
where
    F: futures::Future,
{
    let srv = Arc::new(srv);

    let server = Http::new().bind(&addr, move || Ok(srv.clone()))?;
    server.run_until(stop.map(|_| ()).map_err(|_| ()))
}

pub fn trap(tx: oneshot::Sender<()>, logger: slog::Logger) {
    use nix::libc::c_int;
    use nix::sys::signal::{SIGTERM, SIGINT};

    let trap = signal::trap::Trap::trap(&[SIGTERM, SIGINT]);
    for sig in trap {
        info!(logger, "Stopping due to signal"; "signal" => sig as c_int);

        return tx.send(()).unwrap();
    }
}
