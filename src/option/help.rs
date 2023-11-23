use std::fmt;

use crate::option::flag;
use crate::option::parser::MatchedFlags;

/// The help string.
/// This string is printed when the user asks for help.
static USAGE: &str = "Usage: 
    hsl [options]

META OPTIONS
    -h, --help          show this!
    -v, --version       show the version of search

COMPILATION OPTIONS
    -s, --source        the source file to compile
    -o, --output        the output file to produce
    -t, --target        the targeted architecture (must be in [armv8, armv7, x86])
";

/// A struct that represents the help string.
#[derive(Debug, PartialEq, Clone)]
pub struct HelpString;

impl HelpString {
    /// Deduce a HelpString from the given matches flags.
    pub fn deduce(matches: &MatchedFlags<'_>) -> Option<Self> {
        if matches.count(&flag::HELP) > 0 {
            Some(Self)
        } else {
            None
        }
    }
}

/// Implement the Display trait for HelpString.
/// This allows us to print the help string.
impl fmt::Display for HelpString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", USAGE)?;

        writeln!(f)
    }
}
