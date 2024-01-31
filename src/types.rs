use clap::{Args, Parser, Subcommand, ValueEnum};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
pub enum InvalidChip {
    Unknown,
    Ambiguous,
}

#[derive(Debug)]
pub enum Error {
    CreateCargo,
    CreateFile(String),
    ChangeDir,
    InvalidChip(InvalidChip),
    CargoAdd(String),
}

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
    type Error = Error;

    fn try_from(chip: &str) -> Result<Self, Self::Error> {
        let family_raw = chip
            .get(..5)
            .map_or(Err(Error::InvalidChip(InvalidChip::Unknown)), |s| Ok(s))?;
        if family_raw.to_lowercase().as_str() == "stm32" {
            Ok(Self::STM32)
        } else if &family_raw[..3] == "nrf" {
            Ok(Self::NRF)
        } else {
            Err(Error::InvalidChip(InvalidChip::Unknown))
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
    type Error = Error;
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

        // strip the last character until the key exists
        for i in (1..=chip.len()).rev() {
            if let Some(target) = chip_target_map.get(&chip[..i]) {
                return Ok(target.clone());
            }
        }

        Err(Error::InvalidChip(InvalidChip::Unknown))
    }
}

pub(crate) struct NRFMemoryRegion {
    pub flash_origin: usize,
    pub flash_length: usize,

    pub ram_origin: usize,
    pub ram_length: usize,
}

impl NRFMemoryRegion {
    const NRF52805: Self = Self {
        flash_origin: 0,
        flash_length: 192,
        ram_origin: 0x2 << 28,
        ram_length: 24,
    };
    const NRF52810: Self = Self::NRF52805;
    const NRF52811: Self = Self::NRF52805;

    const NRF52820: Self = Self {
        flash_origin: 0,
        flash_length: 256,
        ram_origin: 0x2 << 28,
        ram_length: 32,
    };

    const NRF52832_XXAA: Self = Self {
        flash_origin: 0,
        flash_length: 512,
        ram_origin: 0x2 << 28,
        ram_length: 64,
    };
    const NRF52832_XXAB: Self = Self::NRF52820;

    const NRF52833: Self = Self {
        flash_origin: 0,
        flash_length: 512,
        ram_origin: 0x2 << 28,
        ram_length: 128,
    };

    const NRF52840: Self = Self {
        flash_origin: 0,
        flash_length: 1024,
        ram_origin: 0x2 << 28,
        ram_length: 256,
    };
}

impl TryFrom<&str> for NRFMemoryRegion {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "nrf52805" => Ok(Self::NRF52805),
            "nrf52810" => Ok(Self::NRF52810),
            "nrf52811" => Ok(Self::NRF52811),
            "nrf52820" => Ok(Self::NRF52820),
            "nrf52832" => Err(Error::InvalidChip(InvalidChip::Ambiguous)),
            "nrf52832_xxaa" => Ok(Self::NRF52832_XXAA),
            "nrf52832_xxab" => Ok(Self::NRF52832_XXAB),
            "nrf52833" => Ok(Self::NRF52833),
            "nrf52840" => Ok(Self::NRF52840),
            // TODO: nrf53x and nrf91x
            _ => Err(Error::InvalidChip(InvalidChip::Unknown)),
        }
    }
}

pub(crate) struct Chip {
    pub family: Family,
    pub target: Target,
    pub name: String,
    pub memory: Option<NRFMemoryRegion>,
}

impl TryFrom<&str> for Chip {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let family = Family::try_from(value)?;
        let target = Target::try_from(value)?;

        Ok(match family {
            Family::STM32 => Self {
                family,
                target,
                name: value.into(),
                memory: None,
            },
            Family::NRF => Self {
                family,
                target,
                // FRAGILE: "_" is used to coerce probe-rs chip search
                name: value.split("_").into_iter().next().unwrap().into(),
                memory: Some(NRFMemoryRegion::try_from(value)?),
            },
        })
    }
}

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
            Self::Halt => "panic-halt".into(),
            Self::Reset => "panic-reset".into(),
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
}

#[derive(Debug, Clone, Subcommand)]
pub enum EmbassyCommand {
    #[command(about = "Initializes an Embassy project in the current workspace")]
    Init(InitArgs),
    #[command(about = "Opens the Embassy documentation page in your web browser")]
    Docs,
}
