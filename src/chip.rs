pub mod family;
pub mod target;

use family::esp::Variant;

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
            ("stm32c0", (STM, Thumbv6)),
            ("stm32f0", (STM, Thumbv6)),
            ("stm32f1", (STM, Thumbv7)),
            ("stm32f2", (STM, Thumbv7)),
            ("stm32f3", (STM, Thumbv7e)),
            ("stm32f4", (STM, Thumbv7e)),
            ("stm32f7", (STM, Thumbv7e)),
            ("stm32g0", (STM, Thumbv6)),
            ("stm32g4", (STM, Thumbv7f)),
            ("stm32h5", (STM, Thumbv8)),
            ("stm32h7", (STM, Thumbv7e)),
            ("stm32l0", (STM, Thumbv6)),
            ("stm32l1", (STM, Thumbv7)),
            ("stm32l4", (STM, Thumbv7e)),
            ("stm32l5", (STM, Thumbv8)),
            ("stm32u5", (STM, Thumbv8)),
            ("stm32wb", (STM, Thumbv7e)),
            ("stm32wba", (STM, Thumbv8)),
            ("stm32wl", (STM, Thumbv7e)),
            // ESP32
            ("esp32c3", (ESP(Variant::C3), Risc32Imc)),
            ("esp32s2", (ESP(Variant::S2), XTensaS2)),
            ("esp32s3", (ESP(Variant::S3), XTensaS3)),
        ];

        let (family, target) = chips
            .iter()
            .find_map(|(s, (f, t))| chip.starts_with(s).then(|| (f.clone(), t.clone())))
            .ok_or(match chip {
                "nrf52832" => Error::InvalidChip(InvalidChip::Ambiguous),
                _ => Error::InvalidChip(InvalidChip::Unknown),
            })?;

        Ok(Self {
            name: match &family {
                STM => chip.to_string(),
                // FRAGILE: "_" is used to coerce probe-rs chip search
                NRF(_) => chip.split('_').next().unwrap().to_string(),
                ESP(variant) => variant.to_string(),
            },
            family,
            target,
        })
    }
}
