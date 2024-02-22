pub mod chip;
pub mod cli;
pub mod error;
mod init;

use clap::Parser;
use cli::{Cargo, Embassy, Feature};
use init::Init;

fn main() {
    let Cargo::Embassy(embassy) = Cargo::parse();

    match embassy {
        Embassy::Init(args) => {
            let init = Init::new();
            init.run(args);
        }
        Embassy::Docs => open::that("https://embassy.dev/book/dev/index.html")
            .expect("Failed to open Embassy documentation page."),
        Embassy::Feature(cmd) => match cmd {
            Feature::List => termimad::print_text(include_str!("docs/features.md")),
            Feature::Add => todo!(),
        },
    }
}
