use crate::arena::Arena;
use crate::flags::Flags;
use crate::parser::slt::Visitor;

pub struct Compiler<'prog> {
    pub arena: &'prog Arena,
    pub flags: Flags<'prog>,

    pub target: crate::target::Target,
}

impl<'prog> Compiler<'prog> {
    pub fn new(arena: &'prog Arena, default_target: Option<&'prog str>) -> Option<Self> {
        let flags = Flags::parse(default_target, &arena);

        if flags.help {
            eprint!("{}", crate::USAGE);
            return None;
        }

        if flags.source_files.is_empty() {
            todo!()
        }

        let Some(target) = flags.target_name.and_then(crate::target::Target::by_name) else {
            eprint!("{}", crate::USAGE);
            return None;
        };

        Some(Self {
            arena,
            flags,
            target,
        })
    }
}
