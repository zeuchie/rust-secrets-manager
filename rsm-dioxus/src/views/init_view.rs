use dioxus::prelude::*;
use lucide_dioxus::{ Settings };
use rsm_core::storage::new_vault_file;

#[component]
pub fn InitView(status: Signal<String>) -> Element {
    rsx!{
        div { class: "p-8",
            div { class: "max-w-2xl",
                h2 { class: "text-white text-2xl mb-2", "Initialize Vault" }
                p { class: "text-slate-400 mb-6", "Create a new encrypted vault" }

                div { class: "bg-slate-800 rounded-lg p-6 border border-slate-700 space-y-6",
                    div { h3 { class: "text-white mb-3", "Vault Configuration" }
                        div { class: "space-y-3",
                            div { class: "flex items-center justify-between p-3 bg-slate-700/50 rounded", span { class: "text-slate-400", "Vault Location" } code { class: "text-blue-400", "~/vault.rsm" } }
                            div { class: "flex items-center justify-between p-3 bg-slate-700/50 rounded", span { class: "text-slate-400", "Encryption Key" } code { class: "text-blue-400", "~/.ssh/rsm-key" } }
                            div { class: "flex items-center justify-between p-3 bg-slate-700/50 rounded", span { class: "text-slate-400", "Encryption" } code { class: "text-green-400", "AES-256" } }
                        }
                    }

                    div { class: "bg-blue-900/20 border border-blue-800/50 rounded p-4", p { class: "text-blue-400 text-sm", "Note: Initializing the vault will create a new encrypted file at ~/vault.rsm and generate an encryption key at ~/.ssh/rsm-key. Keep your key secure!" } }

                    button {
                        onclick: move |_| {
                            status.set("Initializing vault...".to_string());
                            match new_vault_file() {
                                Ok(_) => status.set("Vault initialized successfully".to_string()),
                                Err(e) => status.set(format!("Failed to initialize vault: {}", e)),
                            }
                        },
                        class: "w-full bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded transition-colors flex items-center justify-center gap-2",
                        Settings {color: "white", size: 18,},
                        "Initialize New Vault"
                    }
                }
            }
        }
    }
}
