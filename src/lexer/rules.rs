use regex::Regex;

use crate::lexer::token::TokenKind;

lazy_static! {
    static ref STRING_REGEX: Regex = Regex::new(r#"^"((\\"|\\\\)|[^\\"])*""#).unwrap();
    static ref COMMENT_REGEX: Regex = Regex::new(r#"^//[^\n]*\n"#).unwrap();
    static ref FLOAT_REGEX: Regex =
        Regex::new(r#"^((\d+(\.\d+)?)|(\.\d+))([Ee](\+|-)?\d+)?"#).unwrap();
    static ref IDENTIFIER_REGEX: Regex = Regex::new(r##"^([A-Za-z]|_)([A-Za-z]|_|\d)*"##).unwrap();
}

pub(crate) struct Rule {
    pub kind: TokenKind,
    pub matches: fn(&str) -> Option<usize>,
}

fn match_quote(input: &str, quote: &str) -> Option<usize> {
    input.starts_with(quote).then(|| quote.len())
}

fn match_regex(input: &str, r: &Regex) -> Option<usize> {
    r.find(input).map(|regex_match| regex_match.end())
}

pub(crate) fn get_rules() -> Vec<Rule> {
    vec![
        Rule {
            kind: T![start],
            matches: |input| match_quote(input, "A long time ago in a galaxy far, far away..."),
        },
        Rule {
            kind: T![end],
            matches: |input| match_quote(input, "May the force be with you."),
        },
        Rule {
            kind: T![let],
            matches: |input| match_quote(input, "I am a big deal in the resistance."),
        },
        Rule {
            kind: T![let],
            matches: |input| match_quote(input, "The force is strong with this one."),
        },
        Rule {
            kind: T![let],
            matches: |input| match_quote(input, "That's one hell of a pilot."),
        },
        Rule {
            kind: T![init],
            matches: |input| match_quote(input, "Who, mesa ?"),
        },
        Rule {
            kind: T![init],
            matches: |input| match_quote(input, "Judge me by my size, do you ?"),
        },
        Rule {
            kind: T![int],
            matches: |input| {
                input
                    .char_indices()
                    .take_while(|(_, c)| c.is_ascii_digit())
                    .last()
                    .map(|(pos, _)| pos + 1)
            },
        },
        Rule {
            kind: T![string],
            matches: |input| match_regex(input, &STRING_REGEX),
        },
        Rule {
            kind: T![comment],
            matches: |input| match_regex(input, &COMMENT_REGEX),
        },
        Rule {
            kind: T![ident],
            matches: |input| match_regex(input, &IDENTIFIER_REGEX),
        },
        Rule {
            kind: T![float],
            matches: |input| match_regex(input, &FLOAT_REGEX),
        },
        Rule {
            kind: T![print],
            matches: |input| match_quote(input, "You're eyes can deceive you; don't trust them."),
        },
        Rule {
            kind: T![print],
            matches: |input| match_quote(input, "You'll find I'm full of surprises."),
        },
    ]
}
