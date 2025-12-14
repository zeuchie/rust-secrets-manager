mod cli;
mod vault;
mod storage;
mod crypto;
mod error;

use clap::Parser;
use cli::{CLI, Commands};
use crate::storage::*;
use crate::vault::*;
// use std::env;


fn main() {
    // To use rust secrets manager run 'cargo install --path . && rsm'

    let args = CLI::parse();
    let mut vault = storage::load_or_create_vault();

    // Match on commands and execute appropriate vault operations
    match args.command {
        Commands::Init => {
            storage::load_or_create_vault();
        }
        Commands::Add(args) => {
            let website = Website(args.website);
            let secret = Secret {
                username: args.username,
                password: args.password,
            };
            vault.add_to_vault(website, secret);
            save_vault_to_file(vault);
        }
        Commands::Delete(args) => {
            let website = Website(args.website);
            vault.remove_from_vault(&website);
            save_vault_to_file(vault);
        }
        Commands::Update(args) => {
            let website = Website(args.website);
            let secret = Secret {
                username: args.username,
                password: args.password,
            };
            vault.update_secret(website, secret);
            save_vault_to_file(vault);
        }
        Commands::Get(args) => {
            let website = Website(args.website);
            vault.get_secret(&website);
        }
        Commands::List => {
            vault.list_websites_with_secret();
        }
    }
}
