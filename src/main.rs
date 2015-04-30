extern crate docopt;
extern crate iron;
extern crate mount;
extern crate rustc_serialize;

use docopt::Docopt;

use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use mount::Mount;

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

fn hello(_: &mut Request) -> IronResult<Response> {
    let content_type: Mime = "text/html".parse().unwrap();
    Ok(Response::with((status::Ok, "Hello Rust Warsaw!")).set(content_type))
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.help(true).decode())
        .unwrap_or_else(|e| e.exit());
    println!("Starting server on port {}", args.flag_port);
    let mut mount = Mount::new();
    mount.mount("/", hello);
    Iron::new(mount).http(("127.0.0.1", args.flag_port)).unwrap();
}
