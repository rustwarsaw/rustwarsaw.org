extern crate iron;

use std::io::net::ip::Ipv4Addr;

use iron::mime::Mime;
use iron::prelude::*;
use iron::status;

fn main() {
    let port = 3000;
    Iron::new(|&: _: &mut Request| {
        let content_type: Mime = "text/html".parse().unwrap();
        Ok(Response::with((status::Ok, "Hello Rust Warsaw!")).set(content_type))
    }).listen((Ipv4Addr(127, 0, 0, 1), port)).unwrap();
}
