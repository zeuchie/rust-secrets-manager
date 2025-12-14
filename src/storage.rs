use crate::vault::Vault;
use std::fs;
use std::path::PathBuf;
use std::env;

// Get the path to the vault file
fn vault_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| String::from("."));
    // let cwd = env::current_dir(); // For future reference. Use to get current directory path.
    let mut path = PathBuf::from(home);
    path.push("vault.rsm");
    path
}

// Create a new vault and save it to a JSON file
pub fn new_vault_file() -> Vault {
    let path = vault_path();
    let vault = Vault::new_vault();
    let mut file = fs::File::create(&path).unwrap();
    serde_json::to_writer(&mut file, &vault).unwrap();
    vault
}

// Save vault to a JSON file
pub fn save_vault_to_file(vault: Vault) {
    let path = vault_path();
    let file = fs::File::create(&path).expect("failed to create vault file");
    serde_json::to_writer(&file, &vault).expect("failed to write vault");
}

// Load the vault from a JSON file
pub fn load_vault_from_file() -> anyhow::Result<Vault> {
    let path = vault_path();
    let mut serialized = fs::File::open(path)?;
    let deserialized: Vault = serde_json::from_reader(&mut serialized)?;
    Ok(deserialized)
}

pub fn load_or_create_vault() -> Vault {
    let path = vault_path();
    if path.exists() {
        match load_vault_from_file() {
            Ok(vault) => vault,
            Err(_) => new_vault_file(),
        }
    } else {
        new_vault_file()
    }
}