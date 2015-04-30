extern crate docopt;
extern crate iron;
extern crate mount;
extern crate rustc_serialize;
extern crate staticfile;

use docopt::Docopt;

use iron::prelude::*;
use mount::Mount;
use staticfile::Static;

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
    let mut mount = Mount::new();
    mount.mount("/", Static::new("www/"));
    Iron::new(mount).http(("127.0.0.1", args.flag_port)).unwrap();
}
