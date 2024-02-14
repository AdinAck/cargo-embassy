use clap::{Args, Parser, Subcommand, ValueEnum};
use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum InvalidChip {
    Unknown,
    Ambiguous,
}

#[derive(Debug)]
pub enum Error {
    CargoAdd(String),
    ChangeDir,
    CreateCargo,
    CreateFile(String),
    ErroneousSoftdevice,
    InvalidChip(InvalidChip),
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone)]
pub enum Family {
    STM32,
    NRF(MemRegion),
}

impl Display for Family {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::STM32 => "stm32",
            Self::NRF(_) => "nrf",
        })
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
        f.write_str(match self {
            Self::Thumbv6 => "thumbv6m-none-eabi",
            Self::Thumbv7 => "thumbv7m-none-eabi",
            Self::Thumbv7e => "thumbv7em-none-eabi",
            Self::Thumbv7f => "thumbv7em-none-eabihf",
            Self::Thumbv8 => "thumbv8m.main-none-eabihf",
        })
    }
}

#[derive(Clone, Debug)]
pub(crate) struct MemRegion {
    pub flash_origin: usize,
    pub flash_length: usize,

    pub ram_origin: usize,
    pub ram_length: usize,
}

impl MemRegion {
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

pub(crate) struct Chip {
    pub family: Family,
    pub target: Target,
    pub name: String,
}

impl FromStr for Chip {
    type Err = Error;

    fn from_str(chip: &str) -> Result<Self, Self::Err> {
        use Family::*;
        use Target::*;

        let chips = [
            // nRF
            ("nrf52805", (NRF(MemRegion::NRF52805), Thumbv7f)),
            ("nrf52810", (NRF(MemRegion::NRF52810), Thumbv7f)),
            ("nrf52811", (NRF(MemRegion::NRF52811), Thumbv7f)),
            ("nrf52820", (NRF(MemRegion::NRF52820), Thumbv7f)),
            ("nrf52832_xxaa", (NRF(MemRegion::NRF52832_XXAA), Thumbv7f)),
            ("nrf52832_xxab", (NRF(MemRegion::NRF52832_XXAB), Thumbv7f)),
            ("nrf52833", (NRF(MemRegion::NRF52833), Thumbv7f)),
            ("nrf52840", (NRF(MemRegion::NRF52840), Thumbv7f)),
            // TODO: nrf53x and nrf91x
            // STM
            ("stm32c0", (STM32, Thumbv6)),
            ("stm32f0", (STM32, Thumbv6)),
            ("stm32f1", (STM32, Thumbv7)),
            ("stm32f2", (STM32, Thumbv7)),
            ("stm32f3", (STM32, Thumbv7e)),
            ("stm32f4", (STM32, Thumbv7e)),
            ("stm32f7", (STM32, Thumbv7e)),
            ("stm32g0", (STM32, Thumbv6)),
            ("stm32g4", (STM32, Thumbv7e)),
            ("stm32h5", (STM32, Thumbv8)),
            ("stm32h7", (STM32, Thumbv7e)),
            ("stm32l0", (STM32, Thumbv6)),
            ("stm32l1", (STM32, Thumbv7)),
            ("stm32l4", (STM32, Thumbv7e)),
            ("stm32l5", (STM32, Thumbv8)),
            ("stm32u5", (STM32, Thumbv8)),
            ("stm32wb", (STM32, Thumbv7e)),
            ("stm32wba", (STM32, Thumbv8)),
            ("stm32wl", (STM32, Thumbv7e)),
        ];

        let (family, target) = chips
            .iter()
            .find_map(|(s, (f, t))| chip.starts_with(s).then(|| (f.clone(), t.clone())))
            .ok_or(match chip {
                "nrf52832" => Error::InvalidChip(InvalidChip::Ambiguous),
                _ => Error::InvalidChip(InvalidChip::Unknown),
            })?;

        Ok(Self {
            name: match family {
                STM32 => chip.to_string(),
                // FRAGILE: "_" is used to coerce probe-rs chip search
                NRF(_) => chip.split('_').next().unwrap().to_string(),
            },
            family,
            target,
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
