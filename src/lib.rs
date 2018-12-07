extern crate nickel;
extern crate typemap;
extern crate plugin;
extern crate sled;

pub use middleware::{ SledMiddleware, SledRequestExtensions };

mod middleware;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}