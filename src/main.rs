mod cli;
mod init;

use clap::Parser;
use cli::{Cargo, EmbassyCommand};
use init::common::init;

fn main() {
    let Cargo::Embassy(embassy) = Cargo::parse();

    match embassy.command {
        EmbassyCommand::Init {
            name,
            family,
            target,
            chip,
            commit,
        } => init(name, family, chip, target, commit),
        EmbassyCommand::Docs => open::that("https://embassy.dev/book/dev/index.html")
            .expect("Failed to open Embassy documentation page."),
    }
}
