pub const PUNCTS: &[(&'static str, super::token::TokenKind)] = &[
    ("Never tell me the odds!", T![Mod]),
    ("Not to worry, we are still flying half a ship.", T![Div]),
    ("There's too many of them!", T![Mul]),
    ("Proceed with the countdown.", T![Minus]),
    (
        "Your lightsabers will make a fine addition to my collection.",
        T![Plus],
    ),
    ("-", T![Not]),
];

pub const KEYWORDS: &[(&'static str, super::token::TokenKind)] = &[
    ("Execute order", T![OFnCall]),
    ("Order executed", T![CFnCall]),
    ("A long time ago in a galaxy far, far away...", T![OProgram]),
    ("May the force be with you.", T![CProgram]),
    ("Do, or do not. There is no try.", T![If]),
    ("These aren't the droids you're looking for.", T![Else]),
    ("You have failed me for the last time.", T![IfEnd]),
    ("I am your father.", T![Assign]),
    ("Judge me by my size, do you ?", T![Assign]),
    ("Who, mesa ?", T![Assign]),
    ("What a piece of junk!", T![OAssign]),
    ("The garbage will do.", T![CAssign]),
    ("I am a big deal in the resistance.", T![Let]),
    ("The force is strong with this one.", T![Let]),
    ("That's one hell of a pilot.", T![Let]),
    ("From a certain point of view.", T![True]),
    ("That's impossible!", T![False]),
];
