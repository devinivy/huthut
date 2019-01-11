pub fn annotate<T, U>(items: Vec<T>, make_annotation: fn(&T) -> U) -> Vec<(T, U)> {
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

        let str_part = &string[prev_boundary..boundary];

        if ch.is_whitespace() {
            parts.push(Part::Whitespace(str_part));
        } else {
            parts.push(Part::Word(str_part));
        }
    }

    parts
}

#[derive(Debug, PartialEq)]
pub enum Part<'a> {
    Word(&'a str),
    Whitespace(&'a str),
}

impl<'a> Part<'a> {
    pub fn get_string(&self) -> &'a str {
        match self {
            Part::Word(s) | Part::Whitespace(s) => s
        }
    }
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
            annotate(to_parts("alpha  bet ic"), |part| part.get_string().len()),
            [(Word("alpha"), 5), (Whitespace("  "), 2), (Word("bet"), 3), (Whitespace(" "), 1), (Word("ic"), 2)]
        );
    }
}
