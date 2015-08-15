extern crate iron;
extern crate router;

// To build, $ cargo test
// To use, go to http://127.0.0.1:3000/test

use iron::{Iron, Request, Response, IronResult};
use iron::status;
use router::{Router};
use std::sync::{Arc, Mutex};

fn main() {
    let mut raw_messages : Vec<String> = Vec::new();
    let mut messages = Arc::new(Mutex::new(raw_messages));
    let mut messages_clone = messages.clone();
    let mut router = Router::new();

    fn all_messages_handler(req: &mut Request, msgs: &Vec<String>) -> IronResult<Response> {
        Ok(Response::with((status::Ok, msgs.connect("\n"))))
    };

    fn handler(req: &mut Request, msgs: &mut Vec<String>) -> IronResult<Response> {
        let query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
        msgs.push(query.to_string());
        Ok(Response::with((status::Ok, query)))
    };

    router.get("/msg/:query",  move |r: &mut Request| handler(r, &mut messages.lock().unwrap()));
    router.get("/all", move |r: &mut Request| all_messages_handler(r, &messages_clone.lock().unwrap()));

    Iron::new(router).http("localhost:3000").unwrap();
}
