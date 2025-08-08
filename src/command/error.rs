use std::fmt;

pub type Result<T> = core::result::Result<T, Error>;

pub struct Error {
    kind: ErrorKind,
    loc: Location,
}

pub enum ErrorKind {
    EmptyCmd,
    CmdFailure { cmd: String, code: Option<i32> },
    Io(std::io::Error),
}

pub struct Location {
    file: &'static str,
    line: u32,
}

impl Error {
    pub fn new(kind: ErrorKind, loc: Location) -> Self {
        Self { kind, loc }
    }
}

impl Location {
    pub fn new(file: &'static str, line: u32) -> Self {
        Self { file, line }
    }
}

impl core::error::Error for Error {}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{} - {}", self.loc.file, self.loc.line, self.kind)
    }
}

impl core::error::Error for ErrorKind {}

impl fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;

        match self {
            Io(kind) => write!(f, "io: {kind}"),
            CmdFailure { cmd, code } => {
                if let Some(code) = code {
                    write!(
                        f,
                        "cmd failure: cmd `{cmd}` exited with status code `{code}`"
                    )
                } else {
                    write!(f, "cmd failure: process {cmd} terminated by signal")
                }
            }
            EmptyCmd => write!(f, "cmd empty"),
        }
    }
}

impl From<(std::io::Error, Location)> for Error {
    fn from(value: (std::io::Error, Location)) -> Self {
        let (err, loc) = value;
        let kind = err.downcast::<ErrorKind>().unwrap_or_else(ErrorKind::Io);
        Self::new(kind, loc)
    }
}

macro_rules! new_error {
    ($kind:tt) => {{
        let loc = $crate::command::error::Location::new(file!(), line!());
        $crate::command::error::Error::new($kind, loc)
    }};
    (from $err:tt) => {{
        let loc = $crate::command::error::Location::new(file!(), line!());
        Into::<$crate::command::error::Error>::into(($err, loc))
    }};
}
