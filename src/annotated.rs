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
}

#[derive(Debug, PartialEq)]
pub enum Part<'a> {
    Word(&'a str),
    Whitespace(&'a str),
}

/*
pub struct Annotation<'a, T> {
    string: &'a str,
    info: Option<T>,
}
*/
