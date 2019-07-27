mod processor;
mod tree;

use crate::{collector::Collector, opt::Opt};
use processor::Processor;
use tree::Tree;

pub struct App;

impl App {
    pub fn execute(opt: Opt) -> crate::Result<()> {
        let processor = Processor::new();
        let mut collector = Collector::new();

        for path in Tree::new(&opt.patterns)? {
            let name = path.file_name().expect("Tree iteration provides ONLY the paths of files.");

            println!("{:?}", name.to_str());
        }

        unimplemented!()
    }
}
