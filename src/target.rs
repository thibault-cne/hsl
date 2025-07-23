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

    pub fn name(self) -> &'static str {
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
