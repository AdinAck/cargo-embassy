pub mod family;
pub mod target;

use crate::error::{Error, InvalidChip};
use std::str::FromStr;

pub(crate) struct Chip {
    pub family: family::Family,
    pub target: target::Target,
    pub name: String,
}

impl FromStr for Chip {
    type Err = Error;

    fn from_str(chip: &str) -> Result<Self, Self::Err> {
        use family::{mem_region::MemRegion, Family::*};
        use target::Target::*;

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
