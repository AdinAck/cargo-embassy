#[derive(Debug)]
pub enum Error {
    CargoAdd(String),
    ChangeDir,
    CreateCargo,
    CreateFile(&'static str),
    CreateFolder(&'static str),
    ErroneousSoftdevice,
    InvalidChip(InvalidChip),
}

#[derive(Debug)]
pub enum InvalidChip {
    Unknown,
    Ambiguous,
}
