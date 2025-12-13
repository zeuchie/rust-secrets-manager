mod cli;
mod vault;
mod storage;
mod crypto;
mod error;

use clap::Parser;
use cli::{CLI, Commands};
use vault::Vault;
use storage::Storage;

fn main() {
    // To use rust secrets manager run 'cargo install --path . && rsm'

    let args = CLI::parse();

    // Load vault from file (or create a new file and vault if one doesn't exist)

    // Match on commands and execute appropriate vault operations
    match args.command {
        Commands::Init => {
            todo!()
        }
        Commands::Add(args) => {
            todo!()
        }
        Commands::Delete(args) => {
            todo!()
        }
        Commands::Update(args) => {
            todo!()
        }
        Commands::Get(args) => {
            todo!()
        }
        Commands::List(args) => {
            todo!()
        }
    }
}
