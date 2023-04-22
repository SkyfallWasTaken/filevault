use clap::{Parser, Subcommand};

pub mod new;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: VaultCommand,
}

#[derive(Subcommand)]
pub enum VaultCommand {
    /// Creates a new vault.
    New { name: String },

    /// Unlocks a vault.
    Unlock { name: String },
}
