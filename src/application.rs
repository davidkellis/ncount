mod tree;

use crate::opt::Opt;
use tree::Tree;

pub struct App;

impl App {
    pub fn execute(opt: Opt) -> crate::Result<()> {
        let tree = Tree::new(&opt.pattern)?;
        
        unimplemented!()
    }
}
