#[derive(Debug)]
pub enum Error {
    CargoAdd(String),
    ChangeDir,
    CreateCargo,
    CreateFile(String),
    CreateFolder(String),
    ErroneousSoftdevice,
    ErroneousPanicHandler,
    InvalidChip(InvalidChip),
}

#[derive(Debug)]
pub enum InvalidChip {
    Unknown,
    Ambiguous,
}
