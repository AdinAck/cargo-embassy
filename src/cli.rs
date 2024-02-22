pub mod init_args;

use clap::{Parser, Subcommand};
use init_args::InitArgs;

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
pub enum Cargo {
    #[command(subcommand)]
    Embassy(Embassy),
}

#[derive(Debug, Clone, Subcommand)]
pub enum Embassy {
    #[command(about = "Initializes an Embassy project in the current workspace")]
    Init(InitArgs),
    #[command(about = "Opens the Embassy documentation page in your web browser")]
    Docs,
    #[command(
        subcommand,
        about = "Tools related to features in the Embassy ecosystem"
    )]
    Feature(Feature),
}

#[derive(Debug, Clone, Subcommand)]
pub enum Feature {
    #[command(about = "View a list of the available features")]
    List,
    #[command(about = "Add a feature to your project")]
    Add,
}
