pub const PUNCTS: &[(&str, super::token::TokenKind)] = &[
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

pub const KEYWORDS: &[(&str, super::token::TokenKind)] = &[
    ("Execute order", T![OFnCall]),
    ("Order executed", T![CFnCall]),
    ("A long time ago in a", T![OFnDecl1]),
    ("far, far away...", T![OFnDecl2]),
    ("May the force be with you.", T![CFnDecl]),
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
    ("Starfield", T![Variadic]),
    ("Hypersignal", T![OExtrnFn]),
    ("Jamsignal", T![CExtrnFn]),
    ("Cargo", T![OFnParams]),
    ("UnloadCargo", T![CFnParams]),
];

pub const TYPES: &[(&str, super::token::TokenKind)] = &[
    ("Credit", T![TyInt]),
    ("Holotext", T![TyString]),
    ("Signal", T![TyBool]),
];
