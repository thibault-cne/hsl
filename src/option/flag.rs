use crate::option::parser::{Arg, Args, TakesValue};

// search options
pub static VERSION: Arg = Arg {
    short: Some(b'v'),
    long: "version",
    takes_value: TakesValue::Forbidden,
};
pub static HELP: Arg = Arg {
    short: Some(b'h'),
    long: "help",
    takes_value: TakesValue::Forbidden,
};

// general options
pub static SOURCE: Arg = Arg {
    short: Some(b's'),
    long: "source",
    takes_value: TakesValue::Necessary(None),
};
pub static OUTPUT: Arg = Arg {
    short: Some(b'o'),
    long: "output",
    takes_value: TakesValue::Necessary(None),
};
pub static TARGET: Arg = Arg {
    short: Some(b't'),
    long: "target",
    takes_value: TakesValue::Optional(Some(&["armv8", "armv7", "x86"])),
};

// All args
pub static ALL_ARGS: Args = Args(&[&VERSION, &HELP, &SOURCE, &OUTPUT, &TARGET]);
