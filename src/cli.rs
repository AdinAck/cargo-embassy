use clap::{Args, Parser, Subcommand, ValueEnum};
use std::{collections::HashMap, fmt::Display};

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

impl TryFrom<&str> for Family {
    type Error = ();

    fn try_from(chip: &str) -> Result<Self, Self::Error> {
        let family_raw = chip.get(..5).expect("Invalid chip name: Too short.");
        if family_raw.to_lowercase().as_str() == "stm32" {
            Ok(Self::STM32)
        } else if &family_raw[..3] == "nrf" {
            Ok(Self::NRF)
        } else {
            Err(())
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

impl TryFrom<&str> for Target {
    type Error = ();
    fn try_from(chip: &str) -> Result<Self, Self::Error> {
        let mut chip_target_map = HashMap::new();

        // stm
        chip_target_map.insert("stm32f0", Target::Thumbv6);
        chip_target_map.insert("stm32f1", Target::Thumbv7);
        chip_target_map.insert("stm32f2", Target::Thumbv7);
        chip_target_map.insert("stm32f3", Target::Thumbv7e);
        chip_target_map.insert("stm32f4", Target::Thumbv7e);
        chip_target_map.insert("stm32f7", Target::Thumbv7e);
        chip_target_map.insert("stm32c0", Target::Thumbv6);
        chip_target_map.insert("stm32g0", Target::Thumbv6);
        chip_target_map.insert("stm32g4", Target::Thumbv7e);
        chip_target_map.insert("stm32h5", Target::Thumbv8);
        chip_target_map.insert("stm32h7", Target::Thumbv7e);
        chip_target_map.insert("stm32l0", Target::Thumbv6);
        chip_target_map.insert("stm32l1", Target::Thumbv7);
        chip_target_map.insert("stm32l4", Target::Thumbv7e);
        chip_target_map.insert("stm32l5", Target::Thumbv8);
        chip_target_map.insert("stm32u5", Target::Thumbv8);
        chip_target_map.insert("stm32wb", Target::Thumbv7e);
        chip_target_map.insert("stm32wba", Target::Thumbv8);
        chip_target_map.insert("stm32wl", Target::Thumbv7e);

        // nrf
        chip_target_map.insert("nrf52", Target::Thumbv7f);
        chip_target_map.insert("nrf53", Target::Thumbv8);
        chip_target_map.insert("nrf91", Target::Thumbv8);

        for (series, target) in chip_target_map {
            if chip.find(series).is_some() {
                return Ok(target);
            }
        }

        Err(())
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

        #[arg(long)]
        chip: String,

        #[arg(
            long,
            help = "If provided, will use the version of Embassy from this commit, otherwise the latest version will be used."
        )]
        commit: Option<String>,
    },
    #[command(about = "Opens the Embassy documentation page in your web browser")]
    Docs,
}
