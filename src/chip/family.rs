pub mod mem_region;

use mem_region::MemRegion;
use std::fmt::Display;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone)]
pub enum Family {
    STM32,
    NRF(MemRegion),
    RP2040,
}

impl Display for Family {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::STM32 => "stm32",
            Self::NRF(_) => "nrf",
            Self::RP2040 => "rp",
        })
    }
}
