use indicatif::ProgressBar;

use super::{common::Error, utils::init_file};

pub fn init_memory_x(pb: &ProgressBar) -> Result<(), Error> {
    init_file(
        pb,
        "memory.x",
        include_str!("../templates/memory.x.template"),
    )
}
