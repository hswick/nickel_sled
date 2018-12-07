#[macro_use] extern crate nickel;
extern crate nickel_sled;

use nickel::{Nickel, HttpRouter};
use nickel_sled::{SledMiddleware, SledRequestExtensions};

fn main() {
    let mut server = Nickel::new();
    
    let s = SledMiddleware::new();
    
    server.utilize(s);
    
    server.get("/", middleware! { |request|
        let _tree = request.sled_conn();
    });
    
    server.listen("127.0.0.1:6767").unwrap();
}