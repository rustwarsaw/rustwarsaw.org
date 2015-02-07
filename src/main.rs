#![feature(io)]

extern crate docopt;
extern crate iron;
extern crate "rustc-serialize" as rustc_serialize;

use std::old_io::net::ip::Ipv4Addr;

use docopt::Docopt;

use iron::mime::Mime;
use iron::prelude::*;
use iron::status;

static USAGE: &'static str = "
Usage: rustwarsaw [options]

Options:
    -p <port>, --port <port>  Port number [default: 3000]
    -h, --help  Show help and exit
";

#[derive(RustcDecodable)]
struct Args {
    flag_port: u16,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.help(true).decode())
        .unwrap_or_else(|e| e.exit());
    println!("Starting server on port {}", args.flag_port);
    Iron::new(|&: _: &mut Request| {
        let content_type: Mime = "text/html".parse().unwrap();
        Ok(Response::with((status::Ok, "Hello Rust Warsaw!")).set(content_type))
    }).listen((Ipv4Addr(127, 0, 0, 1), args.flag_port)).unwrap();
}
