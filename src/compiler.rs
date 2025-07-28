use crate::arena::Arena;
use crate::flags::Flags;

pub struct Compiler<'prog> {
    pub arena: &'prog Arena<'prog>,
    pub flags: Flags<'prog>,

    pub target: crate::target::Target,
    pub program: crate::ir::Program<'prog>,
    pub program_path: &'prog str,
    pub object_path: &'prog str,
    pub output_path: &'prog str,
}

impl<'prog> Compiler<'prog> {
    pub fn new(arena: &'prog Arena<'prog>, default_target: Option<&'prog str>) -> Option<Self> {
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

        let program_path =
            build_program_path(arena, flags.output_path, flags.source_files[0], target);

        let Some((object_path, output_path)) = build_object_and_output_path(arena, program_path)
        else {
            return None;
        };

        Some(Self {
            arena,
            flags,
            target,
            program: crate::ir::Program::new(),
            program_path,
            object_path,
            output_path,
        })
    }

    pub fn program_mut(&mut self) -> &mut crate::ir::Program<'prog> {
        &mut self.program
    }

    pub fn try_for_each_source_files<F>(&mut self, mut func: F) -> Result<(), ()>
    where
        F: FnMut(&mut Self, &str) -> Result<(), ()>,
    {
        let source_files = core::mem::replace(&mut self.flags.source_files, vec![]);
        let res = source_files.iter().try_for_each(|f| func(self, f));
        let _ = core::mem::replace(&mut self.flags.source_files, source_files);
        res
    }
}

fn build_program_path<'prog>(
    arena: &'prog Arena,
    output_path: Option<&'prog str>,
    source_file: &'prog str,
    target: crate::target::Target,
) -> &'prog str {
    if let Some(program_path) = output_path {
        if crate::fs::get_file_extension(program_path) == "" {
            arena.strdup(&format!("{program_path}{}", target.file_ext()))
        } else {
            program_path
        }
    } else {
        // SAFETY: this is safe because `flags.source_files` is not empty
        let program_path = crate::fs::strip_extension(source_file);
        arena.strdup(&format!("{program_path}{}", target.file_ext()))
    }
}

fn build_object_and_output_path<'prog>(
    arena: &'prog Arena,
    program_path: &'prog str,
) -> Option<(&'prog str, &'prog str)> {
    let files = crate::fs::Files::new(program_path);

    let Some(object_path) = files.object_path.to_str() else {
        return None;
    };

    let Some(output_path) = files.output_path.to_str() else {
        return None;
    };

    Some((arena.strdup(object_path), arena.strdup(output_path)))
}
