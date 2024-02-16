use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
#[value()]
pub enum Softdevice {
    S112,
    S113,
    S122,
    S132,
    S140,
}

impl Softdevice {
    pub(crate) fn str(&self) -> &str {
        match self {
            Self::S112 => "s112",
            Self::S113 => "s113",
            Self::S122 => "s122",
            Self::S132 => "s132",
            Self::S140 => "s140",
        }
    }
}
