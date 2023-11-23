use std::ffi::OsStr;

use error::OptionsError;

mod error;
mod flag;
mod help;
mod parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Option {
    pub source: String,
    pub output: String,
}

impl Option {
    pub fn parse<'args, I>(args: I) -> OptionsResult<'args>
    where
        I: IntoIterator<Item = &'args OsStr>,
    {
        use crate::option::parser::{Matches, Strictness};

        let Matches { flags, frees } = match flag::ALL_ARGS.parse(args, Strictness::UseLastArgument)
        {
            Ok(m) => m,
            Err(e) => return OptionsResult::InvalidOptions(OptionsError::ParseError(e)),
        };

        if let Some(help) = help::HelpString::deduce(&flags) {
            return OptionsResult::Help(help);
        }

        match Self::deduce(&flags) {
            Ok(p) => OptionsResult::Ok(p, frees),
            Err(e) => OptionsResult::InvalidOptions(e),
        }
    }

    fn deduce(matches: &parser::MatchedFlags) -> Result<Self, OptionsError> {
        let source = match matches.get(&flag::SOURCE)? {
            Some(os_str) => {
                if let Some(str) = os_str.to_str() {
                    str.to_string()
                } else {
                    return Err(OptionsError::BadArgument(&flag::SOURCE, os_str.into()));
                }
            }
            None => return Err(OptionsError::ArgumentNeedsValue(&flag::SOURCE)),
        };
        let output = match matches.get(&flag::OUTPUT)? {
            Some(os_str) => {
                if let Some(str) = os_str.to_str() {
                    str.to_string()
                } else {
                    return Err(OptionsError::BadArgument(&flag::OUTPUT, os_str.into()));
                }
            }
            None => return Err(OptionsError::ArgumentNeedsValue(&flag::OUTPUT)),
        };

        Ok(Self { source, output })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OptionsResult<'args> {
    Ok(Option, Vec<&'args OsStr>),

    InvalidOptions(OptionsError),

    Help(help::HelpString),
}
