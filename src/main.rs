extern crate iron;

use iron::mime::Mime;
use iron::prelude::*;
use iron::status;

fn main() {
    Iron::new(|&: _: &mut Request| {
        let content_type: Mime = "text/html".parse().unwrap();
        Ok(Response::with((status::Ok, "Hello Rust Warsaw!")).set(content_type))
    }).listen("localhost:3000").unwrap();
}
