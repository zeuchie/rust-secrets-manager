mod cli;
mod crypto;
mod storage;
mod vault;

use crate::storage::*;
use crate::vault::*;
use clap::Parser;
use cli::{CLI, Commands};
// use std::env;

fn main() -> anyhow::Result<()> {
    // To use rust secrets manager run 'cargo install --path . && rsm'

    let args = CLI::parse();

    // Match on commands and execute appropriate vault operations
    match args.command {
        Commands::Init => {
            storage::new_vault_file()?;
            println!("Created new vault.");
        }
        Commands::Add(args) => {
            let mut vault = storage::load_vault_from_file()?;
            let website = Website(args.website);
            let secret = Secret {
                username: args.username,
                password: args.password,
            };
            vault.add_to_vault(website, secret);
            save_vault_to_file(&vault, storage::load_vault_key()?)?;
        }
        Commands::Delete(args) => {
            let mut vault = storage::load_vault_from_file()?;

            let website = Website(args.website);
            vault.remove_from_vault(&website);
            save_vault_to_file(&vault, storage::load_vault_key()?)?;
        }
        Commands::Update(args) => {
            let mut vault = storage::load_vault_from_file()?;
            let website = Website(args.website);
            let secret = Secret {
                username: args.username,
                password: args.password,
            };
            vault.update_secret(website, secret);
            save_vault_to_file(&vault, storage::load_vault_key()?)?;
        }
        Commands::Get(args) => {
            let mut vault = storage::load_vault_from_file()?;
            let website = Website(args.website);
            vault.get_secret(&website);
        }
        Commands::List => {
            let mut vault = storage::load_vault_from_file()?;
            vault.list_websites_with_secret();
        }
    }
    Ok(())
}
