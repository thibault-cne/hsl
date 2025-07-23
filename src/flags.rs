pub struct Flags {
    pub source_files: Vec<&'static str>,
    pub target_name: Option<&'static str>,
    pub output_path: Option<&'static str>,

    pub run: bool,
    pub help: bool,
    pub quiet: bool,
}

impl Flags {
    pub fn parse(default_target: Option<&'static str>) -> Self {
        let mut flags = Flags {
            source_files: Vec::new(),
            target_name: default_target,
            output_path: None,
            run: false,
            help: false,
            quiet: false,
        };

        // Skip first arg as it is the executable name
        let mut args = std::env::args().skip(1);

        'args: loop {
            let Some(arg) = args.next() else {
                break 'args;
            };

            // We have that start of a flag so we parse it
            if arg.starts_with("-") {
                flags.parse_arg(&mut args, arg);
            }
        }

        flags
    }

    fn parse_arg<I: Iterator<Item = String>>(&mut self, args: &mut I, arg: String) {
        let mut ptr = 0;
        let mut dash_cpt = 0;
        let arg = arg.leak();
        let bytes = arg.as_bytes();

        'arg: loop {
            // In case we have an arg without value
            if ptr == arg.len() {
                let arg_name = &bytes[(0 + dash_cpt)..ptr];
                match arg_name {
                    b"t" | b"target" => self.target_name = Some(get_next_arg(args)),
                    b"o" | b"output" => self.output_path = Some(get_next_arg(args)),
                    b"help" => {
                        self.help = true;
                        break 'arg;
                    }
                    b"r" | b"run" => self.run = true,
                    b"q" | b"quiet" => self.quiet = true,
                    _ => (),
                }
                break 'arg;
            }

            match bytes[ptr] {
                b'-' => {
                    ptr += 1;
                    dash_cpt += 1;
                }
                b'=' => {
                    // We reached the end of the arg name
                    let arg_name = &bytes[(0 + dash_cpt)..ptr];

                    // # SAFETY: This is safe because indexes are between 0 and `arg.len()` and arg is made of valid UTF-8 char.
                    let arg_value = unsafe { arg.get_unchecked((ptr + 1)..arg.len()) };

                    match arg_name {
                        b"t" | b"target" => self.target_name = Some(arg_value),
                        b"o" | b"output" => self.output_path = Some(arg_value),
                        _ => todo!(),
                    }

                    break 'arg;
                }
                _ => ptr += 1,
            }
        }

        // Add the rest of the arguments as input files
        args.for_each(|a| {
            // Check if the file as the correct extension else raise an error
            if !a.ends_with(".hsl") {
                todo!()
            } else {
                self.source_files.push(a.leak());
            }
        });
    }
}

fn get_next_arg<I: Iterator<Item = String>>(args: &mut I) -> &'static str {
    let Some(arg) = args.next() else { todo!() };

    arg.leak()
}
