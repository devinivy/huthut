use std::iter::{Iterator, Peekable};
use std::str::CharIndices;

pub struct PartIterator<'a> {
    string: &'a str,
    char_indices: Peekable<CharIndices<'a>>,
    boundary: usize,
}

impl<'a> PartIterator<'a> {
    pub fn new(string: &'a str) -> PartIterator<'a> {
        PartIterator {
            string,
            char_indices: string.char_indices().peekable(),
            boundary: 0,
        }
    }
}

impl<'a> Iterator for PartIterator<'a> {
    type Item = Part<'a>;

    fn next(&mut self) -> Option<Part<'a>> {
        while let Some((index, ch)) = self.char_indices.next() {

            if let Some((_, next_ch)) = self.char_indices.peek() {
                if next_ch.is_whitespace() == ch.is_whitespace() {
                    continue;
                }
            }

            let prev_boundary = self.boundary;
            self.boundary = index + ch.len_utf8();

            let word = &self.string[prev_boundary..self.boundary];

            if ch.is_whitespace() {
                return Some(Part::Whitespace(word));
            } else {
                return Some(Part::Word(word));
            }
        }
        None
    }
}

#[derive(Debug, PartialEq)]
pub enum Part<'a> {
    Word(&'a str),
    Whitespace(&'a str),
}

#[cfg(test)]
mod test {
    use super::{Part::*, *};

    #[test]
    fn to_parts_ascii() {

        let to_parts = |string| PartIterator::new(string).collect::<Vec<_>>();

        assert_eq!(to_parts(""), []);
        assert_eq!(to_parts("  "), [Whitespace("  ")]);
        assert_eq!(to_parts("one-word"), [Word("one-word")]);
        assert_eq!(to_parts("alpha  bet ic"), [Word("alpha"), Whitespace("  "), Word("bet"), Whitespace(" "), Word("ic")]);
        assert_eq!(to_parts("alpha  bet ic   "), [Word("alpha"), Whitespace("  "), Word("bet"), Whitespace(" "), Word("ic"), Whitespace("   ")]);
        assert_eq!(to_parts(" alpha  bet ic   "), [Whitespace(" "), Word("alpha"), Whitespace("  "), Word("bet"), Whitespace(" "), Word("ic"), Whitespace("   ")]);
    }
}
