use dioxus::prelude::*;
use lucide_dioxus::{ Trash2 };
use rsm_core::*;

#[component]
pub fn DeleteView(mut status: Signal<String>) -> Element {
    let mut website = use_signal(|| String::new());

    rsx!{
        div { class: "p-8",
            div { class: "max-w-2xl",
                h2 { class: "text-white text-2xl mb-2", "Delete Secret" }
                p { class: "text-slate-400 mb-6", "Remove a secret from your vault" }

                form { onsubmit: move |e| {
                    e.prevent_default();
                    let w = website();
                    match storage::load_vault_from_file() {
                        Ok(mut vault) => {
                            vault.remove_from_vault(&vault::Website(w.clone()));
                            match storage::load_vault_key() {
                                Ok(key) => {
                                    if let Err(e) = storage::save_vault_to_file(&vault, key) {
                                        status.set(format!("Failed to save vault: {}", e));
                                        return;
                                    }
                                }
                                Err(e) => { status.set(format!("Failed to load vault key: {}", e)); return; }
                            }
                        }
                        Err(_) => { status.set("Vault not initialized. Please initialize vault first.".to_string()); return; }
                    }
                    status.set(format!("Secret for {} deleted successfully", w));
                    website.set(String::new());
                }, class: "bg-slate-800 rounded-lg p-6 border border-slate-700",
                    div { class: "space-y-4",
                        div { label { class: "text-white block mb-2", "Website" } input { r#type: "text", value: "{website}", oninput: move |e| website.set(e.value()), placeholder: "https://www.example.com", class: "w-full bg-slate-700 text-white px-4 py-3 rounded border border-slate-600" } }
                        div { class: "bg-red-900/20 border border-red-800/50 rounded p-4", p { class: "text-red-400 text-sm", "Note: This action cannot be undone. The secret will be permanently removed from the vault." } }
                        button { r#type: "submit", class: "w-full bg-red-600 hover:bg-red-700 text-white px-6 py-3 rounded transition-colors flex items-center justify-center gap-2", Trash2 {color: "white", size: 18,}, "Delete Secret" }
                    }
                }
            }
        }
    }
}
