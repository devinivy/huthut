use std::ops::{Range, Index};
use std::slice::SliceIndex;

pub fn annotate<T, U>(items: Vec<T>, make_annotation: impl Fn(&T) -> U) -> Vec<(T, U)> {
    items.into_iter()
        .map(|item| {
            let annotation = make_annotation(&item);
            (item, annotation)
        })
        .collect()
}


pub fn to_parts(string: &str) -> Vec<Part> {
    // While it's unlikely the empty input fast path will be triggered here,
    // knowing the input is non-empty simplifies some code below.
    if s.len() < 1 {
        return Vec::with_capacity(0); // does not allocate
    }

    // Both these passes should probably use str::find because it may
    // be implemented in platform specific assembly or other cleverness,
    // but the implementation is more complex, so maybe later. --SK

    // Preallocate. Even when a prepass is required, it's usually worth,
    // although I haven't tested in this case. However, I suspect the
    // best way here is some data analysis: we could write a script
    // to find maybe the 95th percentile count for tweets we care about
    // and hard code that. --SK

    let mut n = 0;
    // prev is initialized so the first character will always trigger
    // an increment because this doesn't process end of string.
    // Seems like there ought to be a way to get the first UTF-8 character
    // from a string without so many extraneous concepts. --SK
    let mut prev = !string.chars().next().unwrap().is_whitespace();
    for c in string.chars() {
        if prev != c.is_whitespace() {
            n += 1;
            prev = !prev;
        }
    }
    // println!("{} {}", n, string);

    let mut parts: Vec<Part> = Vec::with_capacity(n);
    let mut char_indices = string.char_indices().peekable();
    let mut boundary = 0;

    while let Some((index, ch)) = char_indices.next() {

        if let Some((_, next_ch)) = char_indices.peek() {
            if next_ch.is_whitespace() == ch.is_whitespace() {
                continue;
            }
        }

        let prev_boundary = boundary;
        boundary = index + ch.len_utf8();

        if ch.is_whitespace() {
            parts.push(Part::Whitespace(prev_boundary..boundary));
        } else {
            parts.push(Part::Word(prev_boundary..boundary));
        }
    }

    parts
}

#[derive(Debug, PartialEq)]
pub enum Part {
    Word(Range<usize>),
    Whitespace(Range<usize>),
}

impl Index<&Part> for str {
    type Output = str;

    fn index(&self, part: &Part) -> &str {
        match part {
            Part::Word(range) | Part::Whitespace(range) => range.clone().index(self)
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Part::*, *};

    #[test]
    fn to_parts_ascii() {
        assert_eq!(to_parts(""), []);
        assert_eq!(to_parts("  "), [Whitespace(0..2)]);
        assert_eq!(to_parts("one-word"), [Word(0..8)]);
        assert_eq!(to_parts("alpha  bet ic"), [Word(0..5), Whitespace(5..7), Word(7..10), Whitespace(10..11), Word(11..13)]);
        assert_eq!(to_parts("alpha  bet ic   "), [Word(0..5), Whitespace(5..7), Word(7..10), Whitespace(10..11), Word(11..13), Whitespace(13..16)]);
        assert_eq!(to_parts(" alpha  bet ic   "), [Whitespace(0..1), Word(1..6), Whitespace(6..8), Word(8..11), Whitespace(11..12), Word(12..14), Whitespace(14..17)]);
    }

    #[test]
    fn annotate_length() {
        let string = "alpha  bet ic";
        assert_eq!(
            annotate(to_parts(string), |part| string[part].len()),
            [(Word(0..5), 5), (Whitespace(5..7), 2), (Word(7..10), 3), (Whitespace(10..11), 1), (Word(11..13), 2)]
        );
    }
}
