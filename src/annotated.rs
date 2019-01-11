pub fn annotate<T, U>(items: Vec<T>, make_annotation: fn(&T) -> U) -> Vec<(T, U)> {
    items.into_iter()
        .map(|item| {
            let annotation = make_annotation(&item);
            (item, annotation)
        })
        .collect()
}

pub fn to_parts(string: &str) -> Vec<Part> {

    // In a general purpose function, we'd want to handle empty input here,
    // before the allocation(s) that I assume happen below, but this is
    // processing tweets which are unlikely to be empty. --SK

    // Both these passes should probably use str::find because it may
    // be implemented in platform specific assembly or other cleverness,
    // but the implementation is more complex, so maybe later. --SK

    // Preallocate. Even when a prepass is required, it's usually worth,
    // although I haven't tested in this case. However, I suspect the
    // best way here is some data analysis: we could write a script
    // to find maybe the 95th percentile count for tweets we care about
    // and hard code that.

    let mut n = 0;

    // For each boundary, increment n. A boundary is a place where a
    // character is in a different class from the one before. The
    // (non-existent) character before the beginning of the string is
    // in a class by itself.

    #[derive(PartialEq, Eq)]
    enum Class {
        Before = 0,
        Word,
        White,
    }
    let mut prev_class = Class::Before;

    let mut iterator = string.char_indices();
    while let Some((_, c)) = iterator.next() {
        let cur_class = if c.is_whitespace() {
            Class::White
        } else {
            Class::Word
        };
        if cur_class != prev_class {
            n += 1;
        }
        prev_class = cur_class;
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

        let str_part = &string[prev_boundary..boundary];

        if ch.is_whitespace() {
            parts.push(Part::Whitespace(str_part));
        }
        else {
            parts.push(Part::Word(str_part));
        }
    }

    parts
}

#[cfg(test)]
mod test {
    use super::{Part::*, *};

    #[test]
    fn to_parts_ascii() {
        assert_eq!(to_parts(""), []);
        assert_eq!(to_parts("  "), [Whitespace("  ")]);
        assert_eq!(to_parts("one-word"), [Word("one-word")]);
        assert_eq!(to_parts("alpha  bet ic"), [Word("alpha"), Whitespace("  "), Word("bet"), Whitespace(" "), Word("ic")]);
        assert_eq!(to_parts("alpha  bet ic   "), [Word("alpha"), Whitespace("  "), Word("bet"), Whitespace(" "), Word("ic"), Whitespace("   ")]);
        assert_eq!(to_parts(" alpha  bet ic   "), [Whitespace(" "), Word("alpha"), Whitespace("  "), Word("bet"), Whitespace(" "), Word("ic"), Whitespace("   ")]);
    }

    #[test]
    fn annotate_length() {
        assert_eq!(
            annotate(
                to_parts("alpha  bet ic"),
                |part| part.get_string().len()
            ),
            [
                (Word("alpha"), 5),
                (Whitespace("  "), 2),
                (Word("bet"), 3),
                (Whitespace(" "), 1),
                (Word("ic"), 2),
            ]
        );
    }
}

#[derive(Debug, PartialEq)]
pub enum Part<'a> {
    Word(&'a str),
    Whitespace(&'a str),
}

impl<'a> Part<'a> {
    fn get_string(&self) -> &'a str {
        match self {
            Part::Word(s) | Part::Whitespace(s) => s
        }
    }
}
