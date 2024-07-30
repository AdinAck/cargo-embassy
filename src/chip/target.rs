use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Target {
    Thumbv6,
    Thumbv7,
    Thumbv7e,
    Thumbv7f,
    Thumbv8,
    XTensaS3,
    Risc32Imc,
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Thumbv6 => "thumbv6m-none-eabi",
            Self::Thumbv7 => "thumbv7m-none-eabi",
            Self::Thumbv7e => "thumbv7em-none-eabi",
            Self::Thumbv7f => "thumbv7em-none-eabihf",
            Self::Thumbv8 => "thumbv8m.main-none-eabihf",
            Self::XTensaS3 => "xtensa-esp32s3-none-elf",
            Self::Risc32Imc => "riscv32imc-unknown-none-elf",
        })
    }
}
