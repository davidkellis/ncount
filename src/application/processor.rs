use regex::{Regex, RegexBuilder, Split};

pub struct Processor {
    comment: Regex,
    heading: Regex,
    paragraph: Regex,
    word: Regex,
}

impl Processor {
    pub fn new() -> Self {
        let comment = RegexBuilder::new(r#"<!--.+?-->"#)
            .dot_matches_new_line(true)
            .build()
            .unwrap();

        Self {
            comment,
            heading: Regex::new(r#"#+\W*(.+)"#).unwrap(),
            paragraph: Regex::new(r#".+(\n|$)"#).unwrap(),
            word: Regex::new(r#"(\d+:\d+)|([\w']+\b)"#).unwrap(),
        }
    }

    pub fn read_events<'p, 't>(&'p self, text: &'t str) -> Events<'p, 't> {
        Events {
            processor: self,
            slices: self.comment.split(text),
        }
    }
}

pub struct Events<'p, 't> {
    processor: &'p Processor,
    slices: Split<'p, 't>,
}

pub enum Event<'t> {
    Count { words: usize, paragraphs: usize },
    Heading(&'t str),
}

impl<'p, 't> Iterator for Events<'p, 't> {
    type Item = Event<'t>;

    fn next(&mut self) -> Option<Self::Item> {
        // It looks to me like, because of the fact that I'm potentially splitting paragraphs
        // with these comments I'm pulling out, it's not possible to return by paragraphs or
        // anything like that. As a result, I'm going to return a count consisting of the number
        // of words AND paragraphs in a split. This also means I get to iterate over each block
        // of text twice, which is not ideal, but I guess I'm over it.

        let text = dbg!(self.slices.next()?);

        if let Some(captures) = self.processor.heading.captures(text) {
            return Some(Event::Heading(captures.get(1).unwrap().as_str().trim()));
        }

        Some(Event::Count {
            paragraphs: self.processor.paragraph.find_iter(text).count(),
            words: self.processor.word.find_iter(text).count(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Event, Processor};

    #[test]
    fn sample_is_counted_correctly() {
        let processor = Processor::new();
        let text = include_str!("../../resources/sample.md");
        let words = processor.read_events(text).fold(0, |a, b| match b {
            Event::Count { words, .. } => a + words,
            Event::Heading(_) => a,
        });

        // Word counts hyphenated-words as a single word. I've decided to count them as two. For
        // this reason, my word count will differ from Word's word count to some degree. In this
        // case, my count (322) is one higher than theirs (321).
        assert_eq!(322, words);

        // At present, the above count comes out wrong because we're capturing any headers that 
        // come through but then throwing away any subsequent text. This means the only text that
        // is counted is what which follows a comment but NOT a header.

        // What we're eventually gonna end up doing here, if we aren't careful, is rewriting pest.
        // It's possible that the best way forward here is to take what I have with regard to a
        // framework and import the pest processor code into it.
    }
}
