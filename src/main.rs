extern crate bodyparser;
extern crate hyper;
extern crate iron;
extern crate persistent;

use persistent::Read;
use std::collections::HashMap;
use iron::prelude::*;
use iron::Handler;

const MAX_BODY_LENGTH:usize = 1024 * 1024 * 10;

struct Router {
    // Routes here are simply matched with the url path.
    routes: HashMap<String, Box<Handler>>,
}

impl Router {
    fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    fn add_route<H>(&mut self, path: String, handler: H)
    where
        H: Handler,
    {
        self.routes.insert(path, Box::new(handler));
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path().join("/")) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(iron::status::NotFound)),
        }
    }
}

fn main() {
    let mut router = Router::new();

    router.add_route("hello".to_string(), |_: &mut Request| {
        Ok(Response::with((iron::status::Ok, "Hello world !")))
    });

    router.add_route("hello/again".to_string(), |_: &mut Request| {
        Ok(Response::with((iron::status::Ok, "Hello again !")))
    });

    router.add_route("error".to_string(), |_: &mut Request| {
        Ok(Response::with(iron::status::BadRequest))
    });

    router.add_route("echo".to_string(), |request: & mut Request| {
    let response = match request.get::<bodyparser::Raw>() {
          Ok(Some(body)) => body,
          _ => "{}".to_string()
        };
        Ok(Response::with((iron::status::Ok, response)))
    });
    let mut chain = Chain::new(router);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    let _ = Iron::new(chain).http("localhost:9090");
}
