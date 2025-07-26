use std::process;

pub struct Cmd {
    inner: Vec<&'static str>,
    quiet: bool,
}

impl Cmd {
    pub fn new(quiet: bool) -> Self {
        Self {
            inner: Vec::new(),
            quiet,
        }
    }

    pub fn append(&mut self, cmd: &'static str) -> &mut Self {
        self.inner.push(cmd);
        self
    }

    pub fn run_and_reset(&mut self) -> Result<(), ()> {
        if !self.quiet {
            println!("RUNNING: {}", self.inner.join(" "));
        }

        if self.inner.len() < 1 {
            todo!("error in case the command is incomplete");
        }

        // Create the command and run it, with the previous check the first unwrap will never fail
        process::Command::new(self.inner[0])
            .args(self.inner[1..].iter())
            .output()
            .map_err(|_| ())?;

        // Clear the cmd buffer
        self.inner.clear();
        Ok(())
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
