use dioxus::prelude::*;
use lucide_dioxus::{ Plus };
use rsm_core::*;

#[component]
pub fn AddView(mut status: Signal<String>) -> Element {
    let mut website = use_signal(|| String::new());
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());

    rsx!{
        div { class: "p-8",
            div { class: "max-w-2xl",
                h2 { class: "text-white text-2xl mb-2", "Add New Secret" }
                p { class: "text-slate-400 mb-6", "Store a new secret for a website" }

                form { onsubmit: move |e| {
                    e.prevent_default();
                    if website().is_empty() || username().is_empty() || password().is_empty() {
                        status.set("All fields are required".to_string());
                        return;
                    }
                    let site = website();
                    let user = username();
                    let pass = password();

                    match storage::load_vault_from_file() {
                        Ok(mut vault) => {
                            vault.add_to_vault(vault::Website(site.clone()), vault::Secret { username: user.clone(), password: pass.clone() });
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
                        Err(_) => {
                            status.set("Vault not initialized. Please initialize vault first.".to_string());
                            return;
                        }
                    }

                    status.set(format!("Secret for {} added successfully", site));
                    website.set(String::new()); username.set(String::new()); password.set(String::new());
                }, class: "bg-slate-800 rounded-lg p-6 border border-slate-700",
                    div { class: "space-y-4",
                        div { label { class: "text-white block mb-2", "Website" } input { r#type: "text", value: "{website}", oninput: move |e| website.set(e.value()), placeholder: "https://www.example.com", class: "w-full bg-slate-700 text-white px-4 py-3 rounded border border-slate-600", autocorrect: "off", autocapitalize: "off", spellcheck: "false", autocomplete: "off" } }
                        div { label { class: "text-white block mb-2", "Username" } input { r#type: "text", value: "{username}", oninput: move |e| username.set(e.value()), placeholder: "", class: "w-full bg-slate-700 text-white px-4 py-3 rounded border border-slate-600", autocorrect: "off", autocapitalize: "off", spellcheck: "false", autocomplete: "off" } }
                        div { label { class: "text-white block mb-2", "Password" } div { class: "relative", input { r#type: "password", value: "{password}", oninput: move |e| password.set(e.value()), placeholder: "", class: "w-full bg-slate-700 text-white px-4 py-3 rounded border border-slate-600 pr-12" } } }

                        button { r#type: "submit", class: "w-full bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded transition-colors flex items-center justify-center gap-2", Plus {color: "white", size: 18,}, "Add Secret" }
                    }
                }
            }
        }
    }
}
