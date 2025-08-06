#![allow(dead_code)]

use std::ffi::OsString;

use crate::flags::parser::{Arg, Flag, ShortArg, Values};

/// Errors that can occur when parsing flags.
#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    /// A flag that needs a value but was not given one.
    NeedsValue { flag: Flag, values: Option<Values> },

    /// A flag that can't take value and was given one.
    ForbiddenValue { flag: Flag },

    /// An unknown short argument
    UnknownShortArgument { short: ShortArg },

    /// An unknown long argument, therefore argument.
    UnknownArgument { arg: OsString },
}

/// Errors that can occur when parsing flags into filters.
#[derive(PartialEq, Clone)]
pub enum FlagsError {
    /// When a duplicated flag is found in strict mode.
    Duplicate(Flag, Flag),

    /// The user entered an illegal value for an argument.
    BadArgument(&'static Arg, OsString),

    /// When a flag needs a value but was not given one.
    ArgumentNeedsValue(&'static Arg),

    /// When a parsing error occurs.
    ParseError(ParseError),

    /// When an invalid program path is passed to the flags
    InvalidProgramPath,
}

impl core::fmt::Display for FlagsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Duplicate(f1, f2) => writeln!(f, "duplicated flag {f1} and {f2}"),
            Self::BadArgument(arg, got) => {
                use crate::flags::parser::TakesValue;

                match arg.takes_value {
                    TakesValue::Necessary(Some(val)) => writeln!(
                        f,
                        "bad argument for flags {}, expected necessary values in [{}] and got {}",
                        arg.long,
                        val.join(","),
                        got.to_str().unwrap_or("")
                    ),
                    TakesValue::Optional(Some(val)) => writeln!(
                        f,
                        "bad argument for flags {}, expected necessary values in [{}] and got {}",
                        arg.long,
                        val.join(","),
                        got.to_str().unwrap_or("")
                    ),
                    _ => Ok(()),
                }
            }
            Self::ArgumentNeedsValue(arg) => writeln!(f, "argument {arg} need a value"),
            _ => Ok(()),
        }
    }
}
