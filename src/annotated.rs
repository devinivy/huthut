use std::iter::{Iterator};

pub struct PartIterator<'a> {
    s: &'a str,
    start: usize,
    white: bool,
}

impl<'a> PartIterator<'a> {
    pub fn new(s: &'a str) -> PartIterator<'a> {
        PartIterator {
            s,
            start: 0,
            // Seems like there ought to be a way to get the first UTF-8
            // character from a string without so many extraneous concepts.
            // --SK
            white: match s.chars().next() {
                Some(c) => c.is_whitespace(),
                None => false, // ignored
            },
        }
    }
}

fn is_not_whitespace(c: char) -> bool {
    return !c.is_whitespace();
}

impl<'a> Iterator for PartIterator<'a> {
    type Item = Part<'a>;

    fn next(&mut self) -> Option<Part<'a>> {
        if self.start >= self.s.len() {
            return None
        }

        let pattern = if self.white {
            is_not_whitespace
        } else {
            char::is_whitespace
        };

        // This could use unwrap_or, but I suspect that would evaluate the
        // argument, self.s.len(), even if it's not needed.
        let end = match self.s[self.start..].find(pattern) {
            Some(i) => self.start + i,
            None => self.s.len(),
        };

        let substr = &self.s[self.start..end];

        let part = Some(if self.white {
            Part::Whitespace(substr)
        } else {
            Part::Word(substr)
        });

        self.start = end;
        self.white = !self.white;

        return part
    }

    // Should be able to get prealloc by implementing size_hint
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.size_hint

    // And hopefully the empty input case is handled with no allocations
    // by higher levels.
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
