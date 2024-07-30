use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum Variant {
    C3,
    S3,
}

impl Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::C3 => "esp32c3",
            Self::S3 => "esp32s3",
        })
    }
}
