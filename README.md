# rust-secrets-manager

A local, CLI-based secrets manager written in Rust.
The application stores secrets locally in a vault file.
Secrets are never intended to be stored in plaintext on disk long-term.

## Phase 1: User Input (CLI Only)

**Goal**
- Accept and parse user commands
- No real vault logic yet

**Commands**
```text
init
add <name>
get <name>
list