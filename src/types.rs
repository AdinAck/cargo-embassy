pub mod chip;
pub mod error;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, Default, ValueEnum)]
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

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
pub enum Cargo {
    Embassy(Embassy),
}

#[derive(Args, Debug, Clone)]
pub struct Embassy {
    #[command(subcommand)]
    pub command: EmbassyCommand,
}

#[derive(Debug, Clone, Args)]
pub struct InitArgs {
    #[arg(help = "The name of the Embassy project to create.")]
    pub name: String,

    #[arg(long = "chip", help = "Specifies the target chip.")]
    pub chip_name: String,

    #[arg(value_enum, long, help = "Selects the panic handler.", default_value_t = PanicHandler::Halt)]
    pub panic_handler: PanicHandler,

    #[arg(long, help = "Configure for use with a Softdevice (NRF only).")]
    pub softdevice: Option<Softdevice>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum EmbassyCommand {
    #[command(about = "Initializes an Embassy project in the current workspace")]
    Init(InitArgs),
    #[command(about = "Opens the Embassy documentation page in your web browser")]
    Docs,
}
