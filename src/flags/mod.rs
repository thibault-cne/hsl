use core::fmt;

use error::FlagsError;

mod error;
mod parser;

pub struct Flags<'args> {
    program_path: &'args str,

    pub output_path: Option<&'args str>,
    pub source_files: Vec<&'args str>,
    pub target_name: Option<&'args str>,
    pub quiet: bool,
    pub run: bool,
}

impl<'args> Flags<'args> {
    pub fn parse<I>(
        program_path: Option<&'args std::ffi::OsString>,
        args: I,
        default_target: Option<&'args str>,
    ) -> FlagsResult<'args>
    where
        I: IntoIterator<Item = &'args std::ffi::OsStr>,
    {
        use parser::{Matches, Strictness};

        let Matches { flags, frees } = match ALL_ARGS.parse(args, Strictness::UseLastArgument) {
            Ok(m) => m,
            Err(e) => return FlagsResult::InvalidFlags(error::FlagsError::ParseError(e)),
        };

        let Some(program_path) = program_path.and_then(|p| p.to_str()) else {
            return FlagsResult::InvalidFlags(FlagsError::InvalidProgramPath);
        };

        if let Some(help) = HelpString::deduce(program_path, &flags) {
            return FlagsResult::Help(help);
        }

        match Self::deduce(program_path, default_target, &flags, frees) {
            Ok(p) => FlagsResult::Ok(p),
            Err(e) => FlagsResult::InvalidFlags(e),
        }
    }

    fn deduce(
        program_path: &'args str,
        default_target: Option<&'args str>,
        matches: &parser::MatchedFlags<'args>,
        frees: Vec<&'args std::ffi::OsStr>,
    ) -> Result<Self, FlagsError> {
        let output_path = match matches.get(&OUTPUT)? {
            Some(os_str) => Some(
                os_str
                    .to_str()
                    .ok_or(FlagsError::BadArgument(&OUTPUT, os_str.into()))?,
            ),
            None => None,
        };

        let target_name = match matches.get(&TARGET)? {
            Some(os_str) => Some(
                os_str
                    .to_str()
                    .ok_or(FlagsError::BadArgument(&OUTPUT, os_str.into()))?,
            ),
            None => default_target,
        };

        let run = matches.get(&RUN)?.is_some();
        let quiet = matches.get(&QUIET)?.is_some();

        Ok(Self {
            program_path,
            output_path,
            source_files: frees
                .iter()
                .flat_map(|s| s.to_str())
                .filter(|s| s.ends_with(".hsl"))
                .collect(),
            target_name,
            quiet,
            run,
        })
    }

    pub fn help_string(&self) -> HelpString<'args> {
        HelpString(self.program_path)
    }
}

pub enum FlagsResult<'args> {
    Ok(Flags<'args>),
    InvalidFlags(error::FlagsError),
    Help(HelpString<'args>),
}

pub struct HelpString<'args>(&'args str);

impl<'args> HelpString<'args> {
    fn deduce(program_path: &'args str, matches: &parser::MatchedFlags<'args>) -> Option<Self> {
        if matches.count(&HELP) > 0 {
            Some(Self(program_path))
        } else {
            None
        }
    }
}

impl<'args> fmt::Display for HelpString<'args> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "USAGE:\n{:4}{} [OPTIONS] [...]\n\nOPTIONS:\n{}",
            "", self.0, ALL_ARGS
        )
    }
}

use parser::{Arg, Args, TakesValue};

static HELP: Arg = Arg {
    short: Some(b'h'),
    long: "help",
    takes_value: TakesValue::Forbidden,
    description: "show this message!",
};

static OUTPUT: Arg = Arg {
    short: Some(b'o'),
    long: "output",
    takes_value: TakesValue::Optional(None),
    description: "the path to the output file to produce",
};

static TARGET: Arg = Arg {
    short: Some(b't'),
    long: "target",
    takes_value: TakesValue::Optional(Some(crate::target::TARGET_NAMES)),
    description: "the targeted architecture",
};

static RUN: Arg = Arg {
    short: Some(b'r'),
    long: "run",
    takes_value: TakesValue::Forbidden,
    description: "run the program after compilation",
};

static QUIET: Arg = Arg {
    short: Some(b'q'),
    long: "quiet",
    takes_value: TakesValue::Forbidden,
    description: "quiet the steps of compilation and run",
};

static ALL_ARGS: Args = Args(&[&HELP, &OUTPUT, &TARGET, &RUN, &QUIET]);
