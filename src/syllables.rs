use std::collections::HashMap;
use inflector::string::singularize;
use lazy_static::lazy_static;
use regex::{Regex, Captures};

// Adapted from MIT-licensed https://github.com/words/syllable

pub fn count(word: &str) -> usize {

    let len = word.len();

    if len == 0 {
        return 0;
    }

    if len < 3 {
        return 1;
    }

    if let Some(&syllables) = PROBLEMATICS.get(&word) {
        return syllables;
    }

    if let Some(&syllables) = PROBLEMATICS.get::<str>(&singularize::to_singular(&word)) {
        return syllables;
    }

    let mut syllables: isize = 0;

    let counter_word = REGEX_TRIPLE.replace_all(&word, |_: &Captures| {
        syllables += 3;
        String::from("")
    });

    let counter_word = REGEX_DOUBLE.replace_all(&counter_word, |_: &Captures| {
        syllables += 2;
        String::from("")
    });

    let counter_word = REGEX_SINGLE.replace_all(&counter_word, |_: &Captures| {
        syllables += 1;
        String::from("")
    });

    // Count vowel groups

    syllables += REGEX_VOWEL_GROUPING.find_iter(&counter_word).count() as isize;

    // Adjust for combos that are actually monosyllabic

    syllables -= REGEX_MONOSYLLABIC_X.find_iter(&counter_word).count() as isize;
    syllables -= REGEX_MONOSYLLABIC_Y.find_iter(&counter_word).count() as isize;

    // Adjust for combos that are actually double-syllabic

    syllables += REGEX_DOUBLE_SYLLABIC_X.find_iter(&counter_word).count() as isize;
    syllables += REGEX_DOUBLE_SYLLABIC_Y.find_iter(&counter_word).count() as isize;
    syllables += REGEX_DOUBLE_SYLLABIC_Z.find_iter(&counter_word).count() as isize;
    syllables += REGEX_DOUBLE_SYLLABIC_W.find_iter(&counter_word).count() as isize;

    if syllables <= 0 {
        1
    } else {
        syllables as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_exceptions() {

        assert_eq!(count(""), 0);
        assert_eq!(count("n"), 1);
        assert_eq!(count("mm"), 1);
        assert_eq!(count("eurydice"), 4);
        assert_eq!(count("phoebes"), 2);
        assert_eq!(count("kiloberry"), 4);
    }
}

lazy_static! {
    static ref REGEX_MONOSYLLABIC_X: Regex = Regex::new(r"(?x)
        cia(?:l|$)|
        tia|
        cius|
        cious|
        [^aeiou]giu|
        [aeiouy][^aeiouy]ion|
        iou|
        sia$|
        eous$|
        [oa]gue$|
        .[^aeiuoycgltdb]{2,}ed$|
        .ely$|
        ^jua|
        uai|
        eau|
        ^busi$|
        (?:[aeiouy](?:
            [bcfgklmnprsvwxyz]|
            ch|
            dg|
            g[hn]|
            lch|
            l[lv]|
            mm|
            nch|
            n[cgn]|
            r[bcnsv]|
            squ|
            s[chkls]|
            th
        )ed$)|
        (?:[aeiouy](?:
            [bdfklmnprstvy]|
            ch|
            g[hn]|
            lch|
            l[lv]|
            mm|
            nch|
            nn|
            r[nsv]|
            squ|
            s[cklst]|
            th
        )es$)
    ").unwrap();

    static ref REGEX_MONOSYLLABIC_Y: Regex = Regex::new(r"(?x)
    [aeiouy](?:
        [bcdfgklmnprstvyz]|
        ch|
        dg|
        g[hn]|
        l[lv]|
        mm|
        n[cgn]|
        r[cnsv]|
        squ|
        s[cklst]|
        th
    )e$").unwrap();

    static ref REGEX_DOUBLE_SYLLABIC_X: Regex = Regex::new(r"(?x)
    (?:
        (?:qq|ww|rr|tt|pp|ss|dd|ff|gg|hh|jj|kk|ll|zz|xx|cc|vv|bb|nn|mm)l| # Was ([^aeiouy])\\1l, but we don't have backreferences
        [^aeiouy]ie(?:r|s?t)|
        [aeiouym]bl|
        eo|
        ism|
        asm|
        thm|
        dnt|
        snt|
        uity|
        dea|
        gean|
        oa|
        ua|
        react?|
        orbed| # Cancels `.[^aeiuoycgltdb]{2,}ed$,`
        eings?|
        [aeiouy]sh?e[rs]
    )$").unwrap();

    static ref REGEX_DOUBLE_SYLLABIC_Y: Regex = Regex::new(r"(?x)
        [^gq]ua[^auieo]|
        [aeiou]{3}|
        ^(?:ia|mc|coa[dglx].)|
        ^re(app|es|im|us)
    ").unwrap();

    static ref REGEX_DOUBLE_SYLLABIC_Z: Regex = Regex::new(r"(?x)
        [^aeiou]y[ae]|
        [^l]lien|
        riet|
        dien|
        iu|
        io|
        ii|
        uen|
        real|
        iell|
        eo[^aeiou]|
        [aeiou]y[aeiou]
    ").unwrap();

    static ref REGEX_DOUBLE_SYLLABIC_W: Regex = Regex::new(r"[^s]ia").unwrap();

    static ref REGEX_SINGLE: Regex = Regex::new(r"(?x)
    ^(?:
        un|
        fore|
        ware|
        none?|
        out|
        post|
        sub|
        pre|
        pro|
        dis|
        side
    )|
    (?:
        ly|
        less|
        some|
        ful|
        ers?|
        ness|
        cians?|
        ments?|
        ettes?|
        villes?|
        ships?|
        sides?|
        ports?|
        shires?|
        tion(?:ed|s)?
    )$").unwrap();

    static ref REGEX_DOUBLE: Regex = Regex::new(r"(?x)
    ^(?:
        above|
        anti|
        ante|
        counter|
        hyper|
        afore|
        agri|
        infra|
        intra|
        inter|
        over|
        semi|
        ultra|
        under|
        extra|
        dia|
        micro|
        mega|
        kilo|
        pico|
        nano|
        macro
    )|
    (?:
        fully|
        berry|
        woman|
        women|
        edly
    )$").unwrap();

    static ref REGEX_TRIPLE: Regex = Regex::new(r"(?:ology|ologist|onomy|onomist)$").unwrap();

    static ref REGEX_SPLIT: Regex = Regex::new(r"\b").unwrap();

    static ref REGEX_APOSTROPHE: Regex = Regex::new(r"['’]").unwrap();

    static ref REGEX_NONALPHABETIC: Regex = Regex::new(r"[^a-z]").unwrap();

    static ref REGEX_VOWEL_GROUPING: Regex = Regex::new(r"[aeiouy]+").unwrap();

    static ref PROBLEMATICS: HashMap<&'static str, usize> = [
        ("abalone", 4),
        ("abare", 3),
        ("abed", 2),
        ("abruzzese", 4),
        ("abbruzzese", 4),
        ("aborigine", 5),
        ("acreage", 3),
        ("adame", 3),
        ("adieu", 2),
        ("adobe", 3),
        ("anemone", 4),
        ("apache", 3),
        ("aphrodite", 4),
        ("apostrophe", 4),
        ("ariadne", 4),
        ("cafe", 2),
        ("calliope", 4),
        ("catastrophe", 4),
        ("chile", 2),
        ("chloe", 2),
        ("circe", 2),
        ("coyote", 3),
        ("epitome", 4),
        ("every", 2),
        ("everywhere", 3),
        ("forever", 3),
        ("gethsemane", 4),
        ("guacamole", 4),
        ("hyperbole", 4),
        ("jesse", 2),
        ("jukebox", 2),
        ("karate", 3),
        ("machete", 3),
        ("maybe", 2),
        ("newlywed", 3),
        ("people", 2),
        ("pulse", 1),
        ("recipe", 3),
        ("riverbed", 3),
        ("sesame", 3),
        ("shoreline", 2),
        ("simile", 3),
        ("snuffleupagus", 5),
        ("sometimes", 2),
        ("syncope", 3),
        ("tamale", 3),
        ("yosemite", 4),
        ("daphne", 2),
        ("eurydice", 4),
        ("euterpe", 3),
        ("hermione", 4),
        ("penelope", 4),
        ("persephone", 4),
        ("phoebe", 2),
        ("waterbed", 3),
        ("zoe", 2),
    ]
        .iter().cloned().collect();
}
