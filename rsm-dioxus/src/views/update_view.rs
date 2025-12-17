use dioxus::prelude::*;
use lucide_dioxus::{ SquarePen };
use rsm_core::*;

#[component]
pub fn UpdateView(mut status: Signal<String>) -> Element {
    let mut website = use_signal(|| String::new());
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());

    rsx!{
        div { class: "p-8",
            div { class: "max-w-2xl",
                h2 { class: "text-white text-2xl mb-2", "Update Secret" }
                p { class: "text-slate-400 mb-6", "Update an existing vault secret" }

                form { onsubmit: move |e| {
                    e.prevent_default();
                    let w = website();

                    match storage::load_vault_from_file() {
                        Ok(mut vault) => {
                            vault.update_secret(vault::Website(w.clone()), vault::Secret { username: username(), password: password() });
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

                    status.set(format!("Secret for {} updated successfully", w));
                    website.set(String::new()); username.set(String::new()); password.set(String::new());
                }, class: "bg-slate-800 rounded-lg p-6 border border-slate-700",
                    div { class: "space-y-4",
                        div { label { class: "text-white block mb-2", "Website" } input { r#type: "text", value: "{website}", oninput: move |e| website.set(e.value()), placeholder: "https://www.example.com", class: "w-full bg-slate-700 text-white px-4 py-3 rounded border border-slate-600", autocorrect: "off", autocapitalize: "off", spellcheck: "false", autocomplete: "off" } }
                        div { label { class: "text-white block mb-2", "New Username" } input { r#type: "text", value: "{username}", oninput: move |e| username.set(e.value()), placeholder: "", class: "w-full bg-slate-700 text-white px-4 py-3 rounded border border-slate-600", autocorrect: "off", autocapitalize: "off", spellcheck: "false", autocomplete: "off" } }
                        div { label { class: "text-white block mb-2", "New Password" } div { class: "relative", input { r#type: "password", value: "{password}", oninput: move |e| password.set(e.value()), placeholder: "", class: "w-full bg-slate-700 text-white px-4 py-3 rounded border border-slate-600 pr-12" } } }
                        button { r#type: "submit", class: "w-full bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded transition-colors flex items-center justify-center gap-2", SquarePen {color: "white", size: 18,}, "Update Secret" }
                    }
                }
            }
        }
    }
}
