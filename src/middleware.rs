use nickel::{Request, Response, Continue, Middleware, MiddlewareResult};
use typemap::Key;
use plugin::Extensible;
use sled::{ConfigBuilder, Tree};
    
pub struct SledMiddleware { pub tree: Tree }

impl SledMiddleware {
    pub fn new(path: &str) -> SledMiddleware {
        //let config = ConfigBuilder::new().temporary(true).build();
        //let tree = Tree::start(config).unwrap();

        let tree = Tree::start_default(path).expect(&format!("given path was not found. path: {}", path));
        
        SledMiddleware { tree: tree }
    }
}

impl Key for SledMiddleware { type Value = Tree; }

impl<T> Middleware<T> for SledMiddleware {
    fn invoke<'mw, 'conn>(&self, req: &mut Request<'mw, 'conn, T>, res: Response<'mw, T>) -> MiddlewareResult<'mw, T> {
        req.extensions_mut().insert::<SledMiddleware>(self.tree.clone());
        Ok(Continue(res))
    }
}

pub trait SledRequestExtensions {
    fn sled_conn(&self) -> &Tree;
}

impl<'a, 'b, T> SledRequestExtensions for Request<'a, 'b, T> {
    fn sled_conn(&self) -> &Tree {
        self.extensions().get::<SledMiddleware>().unwrap()
    }
}
