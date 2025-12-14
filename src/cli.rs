use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CLI {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initialize a new vault
    Init,
    /// Add a secret to the vault
    Add(AddArgs),
    /// Delete a secret from the vault
    Delete(DeleteArgs),
    /// Update a secret in the vault
    Update(UpdateArgs),
    /// Get a secret from the vault
    Get(GetArgs),
    /// See all websites with a secret in the vault
    List,
}

#[derive(Debug, Args)]
pub struct AddArgs {
    pub website: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Args)]
pub struct DeleteArgs {
    pub website: String,
}

#[derive(Debug, Args)]
pub struct UpdateArgs{
    pub website: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Args)]
pub struct GetArgs {
    pub website: String,
}