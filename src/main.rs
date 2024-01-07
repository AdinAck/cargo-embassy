mod cli;
mod init;

use std::time::Duration;

use clap::Parser;
use cli::{Cargo, EmbassyCommand};
use indicatif::ProgressBar;
use init::common::init;

fn main() {
    let Cargo::Embassy(embassy) = Cargo::parse();

    match embassy.command {
        EmbassyCommand::Init {
            name,
            chip,
            commit,
            panic_handler,
        } => {
            let pb = ProgressBar::new_spinner();
            pb.enable_steady_tick(Duration::from_millis(100));
            match init(&pb, name, chip, commit, panic_handler) {
                Ok(_) => pb.finish_with_message(format!("Finished in {}s", pb.elapsed().as_secs())),
                Err(e) => pb.abandon_with_message(format!("Failed with error: {:#?}.", e)),
            }
        }
        EmbassyCommand::Docs => open::that("https://embassy.dev/book/dev/index.html")
            .expect("Failed to open Embassy documentation page."),
    }
}
