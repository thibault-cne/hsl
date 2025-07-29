use std::process;

#[macro_use]
pub mod error;

pub struct Cmd<'prog> {
    inner: Vec<&'prog str>,
    quiet: bool,
}

impl<'prog> Cmd<'prog> {
    pub fn new(quiet: bool) -> Self {
        Self {
            inner: Vec::new(),
            quiet,
        }
    }

    pub fn append(&mut self, cmd: &'prog str) -> &mut Self {
        self.inner.push(cmd);
        self
    }

    pub fn run_and_reset(&mut self) -> error::Result<()> {
        if !self.quiet {
            info!("CMD: {}", self.inner.join(" "));
        }

        if self.inner.is_empty() {
            todo!("error in case the command is incomplete");
        }

        // Create the command and run it, with the previous check the first unwrap will never fail
        // TODO: handle this error
        let output = process::Command::new(self.inner[0])
            .args(self.inner[1..].iter())
            .output()
            .map_err(|err| new_error!(from err))?;

        if output.status.success() {
            // Clear the cmd buffer
            self.inner.clear();
            Ok(())
        } else {
            let kind = error::ErrorKind::CmdFailure {
                cmd: self.inner.join(" "),
                code: output.status.code(),
            };
            error!("{}", str::from_utf8_unchecked(&output.stderr));
            Err(new_error!(kind))
        }
    }
}

macro_rules! cmd_append {
    ($cmd:ident, $($arg:expr),*) => {
        $cmd
        $(
            .append($arg)
        )*;
    };
}
