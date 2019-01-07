pub fn to_parts(string: &str) -> Vec<Part> {

    let mut char_indices = string.char_indices().peekable();
    let mut parts: Vec<Part> = Vec::new();
    let mut boundry = 0;

    while let Some((i, ch)) = char_indices.next() {

        let maybe_next_ch = char_indices.peek().map(|(_, next_ch)| next_ch);
        let ch_is_whitespace = ch.is_whitespace();

        if maybe_next_ch.map_or(false, |next_ch| next_ch.is_whitespace() == ch_is_whitespace) {
            continue;
        }

        let prev_boundry = boundry;
        boundry = i + ch.len_utf8();

        let str_part = &string[prev_boundry..boundry];

        if ch_is_whitespace {
            parts.push(Part::Whitespace(str_part))
        }
        else {
            parts.push(Part::Word(str_part))
        }
    }

    parts
}

#[cfg(test)]
mod test {
    use super::Part::{Word, Whitespace};

    #[test]
    fn to_parts_test() {
        assert_eq!(super::to_parts(""), []);
        assert_eq!(super::to_parts("  "), [Whitespace("  ")]);
        assert_eq!(super::to_parts("one-word"), [Word("one-word")]);
        assert_eq!(super::to_parts("alpha  bet ic"), [Word("alpha"), Whitespace("  "), Word("bet"), Whitespace(" "), Word("ic")]);
        assert_eq!(super::to_parts("alpha  bet ic   "), [Word("alpha"), Whitespace("  "), Word("bet"), Whitespace(" "), Word("ic"), Whitespace("   ")]);
        assert_eq!(super::to_parts(" alpha  bet ic   "), [Whitespace(" "), Word("alpha"), Whitespace("  "), Word("bet"), Whitespace(" "), Word("ic"), Whitespace("   ")]);
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
