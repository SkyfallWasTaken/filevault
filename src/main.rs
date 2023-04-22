use clap::Parser;
use color_eyre::{eyre::eyre, Result};

mod consts;
mod vault;

mod commands;
use commands::VaultCommand;
use once_cell::sync::OnceCell;
use rand::{rngs::StdRng, SeedableRng};

// TODO: Switch to `ThreadRng`
static THREAD_RNG: OnceCell<StdRng> = OnceCell::new();

fn main() -> Result<()> {
    color_eyre::install()?;
    THREAD_RNG
        .set(StdRng::from_entropy())
        .map_err(|_| eyre!("Failed to set THREAD_RNG"))?;

    let cli = commands::Cli::parse();
    match &cli.command {
        VaultCommand::New { name } => commands::new::run(name),
        _ => unimplemented!(),
    }?;

    Ok(())
}
