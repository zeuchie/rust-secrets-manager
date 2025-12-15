use crate::vault::Vault;
use crate::crypto::Encryptor;
use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use rand::Rng;

// Get the path to the vault file
fn vault_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| String::from("."));
    let mut path = PathBuf::from(home);
    path.push("vault.rsm");
    path
}

// Get the path to the vault file encryption key
fn key_path() -> PathBuf {
    let home = env::var("HOME").unwrap();
    let mut path = PathBuf::from(home);
    path.push(".ssh/rsm-key");
    path
}

// Create a new vault and save it to a JSON file
pub fn new_vault_file() -> anyhow::Result<Vault> {
    // Create a new encryption key
    let mut key = [0u8; 32];
    rand::rng().fill(&mut key);

    // Save the key to .ssh folder
    let key_path = key_path();
    let mut key_file = fs::File::create(key_path)?;
    key_file.write_all(&key)?;

    // Create new vault and save to vault path
    let vault = Vault::new_vault();
    save_vault_to_file(&vault, key)?;
    Ok(vault)
}

// Save vault as encrypted JSON to a file
pub fn save_vault_to_file(vault: &Vault, key: [u8; 32]) -> anyhow::Result<()> {
    let v = serde_json::to_vec(vault).unwrap();
    let encryptor = Encryptor::new(&key);
    let v = encryptor.encrypt(&v)?;
    let path: PathBuf = vault_path();
    let mut file = fs::File::create(path)?;
    file.write_all(&v)?;
    Ok(())
}

// Load the vault from a an encrpyted JSON file
pub fn load_vault_from_file() -> anyhow::Result<Vault> {
    let vault_key = load_vault_key()?;
    let encryptor = Encryptor::new(&vault_key);
    
    let path = vault_path();
    let mut file = fs::File::open(path)?;
    let mut v = vec![];
    file.read_to_end(&mut v)?;

    let v = encryptor.decrypt(&v)?;
    let vault = serde_json::from_slice(&v)?;
    Ok(vault)
}

// Load the key from path
pub fn load_vault_key() -> anyhow::Result<[u8; 32]> {
    let path = key_path();
    let mut file = fs::File::open(path)?;
    let mut key = [0; 32];
    file.read_exact(&mut key)?;
    Ok(key)
}
