#[derive(Clone, Debug)]
pub struct MemRegion {
    pub flash_origin: usize,
    pub flash_length: usize,

    pub ram_origin: usize,
    pub ram_length: usize,
}

impl MemRegion {
    pub const NRF52805: Self = Self {
        flash_origin: 0,
        flash_length: 192,
        ram_origin: 0x2 << 28,
        ram_length: 24,
    };
    pub const NRF52810: Self = Self::NRF52805;
    pub const NRF52811: Self = Self::NRF52805;

    pub const NRF52820: Self = Self {
        flash_origin: 0,
        flash_length: 256,
        ram_origin: 0x2 << 28,
        ram_length: 32,
    };

    pub const NRF52832_XXAA: Self = Self {
        flash_origin: 0,
        flash_length: 512,
        ram_origin: 0x2 << 28,
        ram_length: 64,
    };
    pub const NRF52832_XXAB: Self = Self::NRF52820;

    pub const NRF52833: Self = Self {
        flash_origin: 0,
        flash_length: 512,
        ram_origin: 0x2 << 28,
        ram_length: 128,
    };

    pub const NRF52840: Self = Self {
        flash_origin: 0,
        flash_length: 1024,
        ram_origin: 0x2 << 28,
        ram_length: 256,
    };
}
