mod application;
mod collector;
mod error;
mod opt;

use application::App;
use opt::Opt;

type Result<T, E = error::Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    App::execute(Opt::from_args())
}

// use regex::RegexBuilder;

// fn main() {
//     let content = include_str!("../test-foo.txt");
//     let pattern = RegexBuilder::new(r#"<!--.+?-->"#)
//         .dot_matches_new_line(true)
//         .build()
//         .unwrap();

//     for cap in pattern.find_iter(content) {
//         println!("{}", cap.as_str());
//     }

//     for element in pattern.split(content).filter(|element| !is_whitespace(element)) {
//         println!("{:?}", element.trim());
//     }
// }

// fn is_whitespace(s: &str) -> bool {
//     s.bytes().all(|u| u.is_ascii_whitespace())
// }
