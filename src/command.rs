use std::process;

pub struct Cmd(Vec<&'static str>);

impl Cmd {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn append(mut self, cmd: &'static str) -> Self {
        self.0.push(cmd);
        self
    }

    pub fn run_and_reset(&mut self) -> Result<(), ()> {
        if self.0.len() < 1 {
            todo!("error in case the command is incomplete");
        }

        // Create the command and run it, with the previous check the first unwrap will never fail
        process::Command::new(self.0[0])
            .args(self.0[1..].iter())
            .output()
            .map_err(|_| ())?;

        // Clear the cmd buffer
        self.0.clear();
        Ok(())
    }
}
