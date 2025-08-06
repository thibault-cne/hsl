enum_with_order! {
    #[derive(Clone, Copy)]
    enum Target in TARGET_ORDER {
        AArch64Darwin
    }
}

impl Target {
    pub fn file_ext(self) -> &'static str {
        match self {
            Self::AArch64Darwin => "",
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::AArch64Darwin => "aarch64-darwin",
        }
    }

    pub fn by_name(name: &str) -> Option<Self> {
        TARGET_ORDER
            .iter()
            .find(|target| target.name() == name)
            .copied()
    }
}

macro_rules! target_names {
    ($name:ident => $($target:tt),*) => {
        pub const $name: &[&'static str] = &[
            $(
                Target::$target.name(),
            ),*
        ];
    };
}

target_names! {TARGET_NAMES => AArch64Darwin}
