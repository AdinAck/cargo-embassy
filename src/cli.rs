use clap::{Args, Parser, Subcommand, ValueEnum};
use std::fmt::Display;

#[derive(Debug, Clone, ValueEnum)]
#[value()]
pub enum Family {
    STM32,
    NRF,
}

impl Display for Family {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::STM32 => f.write_str("stm32"),
            Self::NRF => f.write_str("nrf"),
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
#[value()]
pub enum Target {
    Thumbv6,
    Thumbv7,
    Thumbv7e,
    Thumbv7f,
    Thumbv8,
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Thumbv6 => f.write_str("thumbv6m-none-eabi"),
            Self::Thumbv7 => f.write_str("thumbv7m-none-eabi"),
            Self::Thumbv7e => f.write_str("thumbv7em-none-eabi"),
            Self::Thumbv7f => f.write_str("thumbv7em-none-eabihf"),
            Self::Thumbv8 => f.write_str("thumbv8m.main-none-eabihf"),
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

#[derive(Debug, Clone, Subcommand)]
pub enum EmbassyCommand {
    #[command(about = "Initializes an Embassy project in the current workspace")]
    Init {
        #[arg(help = "The name of the Embassy project to create.")]
        name: String,

        #[arg(long, value_enum)]
        family: Family,

        #[arg(long)]
        chip: String,

        #[arg(long, value_enum)]
        target: Target,

        #[arg(
            long,
            help = "If provided, will use the version of Embassy from this commit, otherwise the latest version will be used."
        )]
        commit: Option<String>,
    },
    #[command(about = "Opens the Embassy documentation page in your web browser")]
    Docs,
}
