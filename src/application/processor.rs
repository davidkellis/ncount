use regex::{Regex, RegexBuilder};

pub struct Processor {
    comment: Regex,
}

impl Processor {
    fn new() -> Self {
        let comment = RegexBuilder::new(r#"<!--.+?-->"#)
            .dot_matches_new_line(true)
            .build()
            .unwrap();

        Self {
            comment
        }
    }
}
