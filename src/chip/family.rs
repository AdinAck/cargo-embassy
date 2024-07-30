pub mod esp;
pub mod mem_region;

use esp::Variant;
use mem_region::MemRegion;
use std::fmt::Display;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone)]
pub enum Family {
    STM,
    NRF(MemRegion),
    ESP(Variant),
}

impl Display for Family {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::STM => "stm32",
            Self::NRF(_) => "nrf",
            Self::ESP(_) => "esp",
        })
    }
}
