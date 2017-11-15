extern crate iron;
extern crate router;

use std::env;
use iron::prelude::*;
use iron::status;
use router::Router;


fn get_server_port() -> u16 {
    env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080)
}

fn main() {
    let mut router = Router::new();

    router.get("/", handler, "index");
    router.get("/:query", handler, "query");


    Iron::new(router).http(("0.0.0.0", get_server_port())).unwrap();

    fn handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }
}
