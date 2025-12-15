use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Vault {
    secrets: HashMap<Website, Secret>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Website(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Secret {
    pub username: String,
    pub password: String,
}

impl Vault {
    pub fn new_vault() -> Vault {
        Vault {
            secrets: HashMap::new(),
        }
    }

    pub fn add_to_vault(&mut self, website: Website, secret: Secret) {
        if !self.secrets.contains_key(&website) {
            println!("The secret for {} has been added.", &website.0);
            self.secrets.insert(website, secret);
        } else {
            println!(
                "There is already a secret for {w} in the vault. To edit the secret for {w}, use update.",
                w = website.0,
            )
        }
    }

    pub fn remove_from_vault(&mut self, website: &Website) {
        if self.secrets.contains_key(website) {
            println!("The secret for {} has been removed.", website.0);
            self.secrets.remove(website);
        } else {
            println!(
                "There is no secret for {} in the vault. To view all websites with a secret, use list.",
                website.0
            )
        }
    }

    pub fn update_secret(&mut self, website: Website, secret: Secret) {
        if self.secrets.contains_key(&website) {
            println!("The secret for {} has been updated.", &website.0);
            self.secrets.insert(website, secret);
        } else {
            println!(
                "There is no secret for {w} in the vault. To create a new secret for {w}, use add.",
                w = website.0
            )
        }
    }

    pub fn get_secret(&mut self, website: &Website) -> Option<&Secret> {
        self.secrets.get(website)
    }

    pub fn list_websites_with_secret(&self) {
        for website in self.secrets.keys() {
            println!("{}", website.0);
        }
    }
}
