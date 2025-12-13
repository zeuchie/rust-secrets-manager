use std::collections::HashMap;

pub struct Vault {
    secrets: HashMap<String, (String, String)>,
}

impl Vault {
    pub fn new_vault() -> Vault {
        Vault {
            secrets: HashMap::new(),
        }
    }

    pub fn add_to_vault(&mut self, website: String, username: String, password: String) {
        if !self.secrets.contains_key(&website) {
            println!("The secret for {} has been added.", &website);
            self.secrets.insert(website, (username, password));
        } else {
            println!(
                "There is already a secret for {website} in the vault. To edit the secret for {website}, use update."
            )
        }
    }

    pub fn remove_from_vault(&mut self, website: &String) {
        if self.secrets.contains_key(website) {
            println!("The secret for {} has been removed.", website);
            self.secrets.remove(website);
        } else {
            println!(
                "There is no secret for {website} in the vault. To view all websites with a secret, use list."
            )
        }
    }

    pub fn update_secret(&mut self, website: String, username: String, password: String) {
        if self.secrets.contains_key(&website) {
            println!("The secret for {} has been updated.", &website);
            self.secrets.insert(website, (username, password));
        } else {
            println!(
                "There is no secret for {website} in the vault. To create a new secret for {website}, use add."
            )
        }
    }

    pub fn get_secret(&mut self, website: &String) {
        if self.secrets.contains_key(website) {
            println!("The username for {website} is {} and the password has been copied to clipboard.", self.secrets.get(website).unwrap().0);
            // TODO: copy password to clipboard
        } else {
            println!(
                "There is no secret for {website} in the vault. To create a new secret for {website}, use add."
            )
        }
    }

    pub fn list_websites_with_secret(&self) {
        for website in self.secrets.keys() {
            println!("{website}");
        }
    }
}
