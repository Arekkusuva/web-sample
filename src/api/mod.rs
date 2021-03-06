use std::collections::HashMap;
use std::convert::AsRef;

use iron::prelude::*;
use iron::Handler;
use iron::status;
use ijr::JsonResponseMiddleware;
use iron_auth::AuthConfigMiddleware;

use utils::logger::logger_factory;

mod controllers;
mod middlewares;
mod transport;
mod utils;

use self::controllers::Engage;
use self::middlewares::{LoggerMiddleware, ResponseTimeLoggerMiddleware};

// TODO: Add context.
pub struct Router {
    routes: HashMap<String, Box<Handler>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn get_full_path<P, M>(path: P, method: M) -> String
        where P: AsRef<str>,
              M: AsRef<str>,
    {
        format!("{}-{}", method.as_ref(), path.as_ref())
    }

    fn add_route<P, M, H>(&mut self, path: P, method: M, handler: H)
        where P: AsRef<str>,
              M: AsRef<str>,
              H: Handler,
    {
        let key = Router::get_full_path(path, method);
        self.routes.insert(key, Box::new(handler));
    }

    pub fn get<P, H>(&mut self, path: P, handler: H)
        where P: AsRef<str>,
              H: Handler,
    {
        self.add_route(path, "GET", handler);
    }

    pub fn post<P, H>(&mut self, path: P, handler: H)
        where P: AsRef<str>,
              H: Handler,
    {
        self.add_route(path, "POST", handler);
    }

}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&Router::get_full_path(req.url.path().join("/"), req.method.as_ref())) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound)),
        }
    }
}

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

pub fn start_listening(port: i32) {
    let logger = logger_factory();

    let router= Router::new().engage();
    let mut chain = Chain::new(router);

    chain.link_before(persistent::Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    let logger_middleware = LoggerMiddleware::new(&logger);
    chain.link_before(logger_middleware);
    chain.link_before(ResponseTimeLoggerMiddleware);
    chain.link_before(AuthConfigMiddleware::new(
        "secret".to_string(),
        "redis://localhost",
    ));
    chain.link_after(JsonResponseMiddleware::new());
    chain.link_after(ResponseTimeLoggerMiddleware);

    Iron::new(chain).http(format!("localhost:{}", port)).expect("http");
}
