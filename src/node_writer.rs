use std::path::Path;
use crate::visitable::Visitable;

pub struct NodeWriter {

}

impl Visitable for NodeWriter {
    fn visit(&mut self, path: &Path) {
        println!("Visiting : {}", path.display());
    }
}
