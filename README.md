# rust-secrets-manager
**Overview:**
- A local CLI secrets manager written in Rust. The application stores and retrieves secrets (usernames/passwords) in a single AES-256 encrypted vault file on your machine.

**Features**
- **AES-256 encryption:** Vault contents are encrypted at rest using AES-256.
- **Local-only storage:** All data lives on the local filesystem.

**Install & Run**
- **Install:** `cargo install --path .`
- **Run:** `rsm` (or `cargo run --release -- [COMMAND]` for development runs).

**Available Commands**
- `rsm init` — initialize a new vault.
- `rsm add <website> <username> <password>` — add a secret for a website.
- `rsm get <website>` — retrieve the secret for a website.
- `rsm update <website> <username> <password>` — update an existing secret.
- `rsm delete <website>` — delete a secret.
- `rsm list` — list all websites stored in the vault.

**Files & Storage**
- **Vault file:** The encrypted vault file is stored at `~/vault.rsm`.
- **Encryption key:** The key is placed in the user's .ssh folder as `~/.ssh/rsm-key`. 

****Note about path configurability*:** The CLI currently does **not** prompt for a custom vault or key path; the paths above reflect the current defaults on macOS.*