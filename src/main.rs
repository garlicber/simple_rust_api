extern crate iron;
extern crate router;

// To build, $ cargo test
// To use, go to http://127.0.0.1:3000/test

use iron::{Iron, Request, Response, IronResult};
use iron::status;
use router::{Router};

fn main() {
    let mut messages = Vec::new();
    let mut router = Router::new();

    let all_messages_handler = |req: &mut Request| -> IronResult<Response> {
            Ok(Response::with((status::Ok, messages.connect("\n"))))
    };

    let handler =|req: &mut Request| -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>()
            .unwrap().find("query").unwrap_or("/");
        messages.push(*query);
            Ok(Response::with((status::Ok, "Hallo ".to_string() + *query)))
    };

    router.get("/", handler);
    router.get("/msg/:query", handler);
    router.get("/all", all_messages_handler);

    Iron::new(router).http("localhost:3000").unwrap();


}
