use clap::ValueEnum;

#[derive(Debug, Clone, Default, PartialEq, ValueEnum)]
#[value()]
pub enum PanicHandler {
    #[default]
    Halt,
    Reset,
}

impl PanicHandler {
    pub(crate) fn str(&self) -> &str {
        match self {
            Self::Halt => "panic-halt",
            Self::Reset => "panic-reset",
        }
    }
}
