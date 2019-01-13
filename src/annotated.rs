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

    let mut parts: Vec<Part> = Vec::new();
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
