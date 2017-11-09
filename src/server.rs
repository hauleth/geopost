use std::collections::HashMap;

use futures;
use futures::Future;
use hyper;
use hyper::StatusCode;
use hyper::header::ContentLength;
use hyper::server::{Request, Response, Service};
use slog;

use zip_codes::ZipCodes;

macro_rules! some_404 {
    ($what:expr, $self:expr) => {
        match $what {
            Some(data) => data,
            _ => return $self.not_found(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Server {
    data: HashMap<String, ZipCodes>,
    logger: slog::Logger,
}

impl Server {
    pub fn new(logger: slog::Logger) -> Self {
        Server {
            logger: logger,
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, code: &str, country: ZipCodes) {
        self.data.insert(code.to_lowercase().to_string(), country);
    }
}

impl Server {
    fn respond_with<'a, I>(&self, mut path: I, country: &ZipCodes) -> <Self as Service>::Future
    where
        I: Iterator<Item = String>,
    {
        let zip = some_404!(path.next(), self);
        let region = some_404!(country.find(&zip), self);
        info!(self.logger, "Respond for country"; "zip" => &zip);
        let json = ::serde_json::to_string(region).unwrap();

        let resp = Response::new()
            .with_header(ContentLength(json.len() as u64))
            .with_body(json);

        Box::new(futures::future::ok(resp))
    }

    fn not_found(&self) -> <Self as Service>::Future {
        info!(self.logger, "Path not found");
        Box::new(futures::future::ok(
            Response::new().with_status(StatusCode::NotFound),
        ))
    }
}

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        info!(self.logger, "Incoming request"; "path" => req.path());

        let mut path = req.path().split('/').skip(1).map(|s| {
            s.to_lowercase().to_string()
        });
        let country_code = some_404!(path.next(), self);

        match self.data.get(&country_code) {
            Some(region) => self.respond_with(path, region),
            _ => self.not_found(),
        }
    }
}
