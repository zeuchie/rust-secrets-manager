mod cli;

use clap::Parser;
use cli::{CLI, Commands};
use rsm_core::storage;
use rsm_core::vault::*;
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
            storage::save_vault_to_file(&vault, storage::load_vault_key()?)?;
        }
        Commands::Delete(args) => {
            let mut vault = storage::load_vault_from_file()?;

            let website = Website(args.website);
            vault.remove_from_vault(&website);
            storage::save_vault_to_file(&vault, storage::load_vault_key()?)?;
        }
        Commands::Update(args) => {
            let mut vault = storage::load_vault_from_file()?;
            let website = Website(args.website);
            let secret = Secret {
                username: args.username,
                password: args.password,
            };
            vault.update_secret(website, secret);
            storage::save_vault_to_file(&vault, storage::load_vault_key()?)?;
        }
        Commands::Get(args) => {
            let mut vault = storage::load_vault_from_file()?;
            let website = Website(args.website);
            let secret = vault.get_secret(&website);
            if let Some(secret) = secret {
                println!(
                    "The username for {} is {} and the password has been copied to clipboard.",
                    website.0,
                    secret.username
                );
                // Copy password to OS clipboard
                cli_clipboard::set_contents(secret.password.to_owned())
                    .unwrap();
            } else {
                println!(
                    "There is no secret for {w} in the vault. To create a new secret for {w}, use add.",
                    w = website.0
                )
            }
        }
        Commands::List => {
            let vault = storage::load_vault_from_file()?;
            vault.list_websites_with_secret();
        }
    }
    Ok(())
}
