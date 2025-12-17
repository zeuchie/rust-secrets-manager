use dioxus::prelude::*;
use lucide_dioxus::{Copy, Eye, EyeOff, RotateCw, Trash2};
use rsm_core::*;

#[component]
pub fn ListView(mut status: Signal<String>) -> Element {
    let mut vault = use_signal(|| match storage::load_vault_from_file() {
        Ok(v) => v,
        Err(_) => vault::Vault::new_vault(),
    });

    let list: Vec<(vault::Website, vault::Secret)> =
        vault.read().secrets.clone().into_iter().collect();

    rsx! {
        div { class: "p-8",
            div { class: "max-w-4xl",
                div { class: "flex items-center justify-between mb-2",
                    h2 { class: "text-white text-2xl", "All Secrets" }
                    button { onclick: move |_| {
                        match storage::load_vault_from_file() {
                            Ok(v) => { vault.set(v); status.set("Vault refreshed".to_string()); },
                            Err(e) => { status.set(format!("Failed to refresh vault: {}", e)); }
                        } }, 
                        class: "flex items-center gap-2 bg-slate-700 px-3 py-2 rounded hover:bg-slate-600 text-slate-200", RotateCw {color: "white", size: 16,}, "Refresh" 
                    }
                }
                p { class: "text-slate-400 mb-6", "Manage all the secrets stored in your vault" }

                { if vault.read().secrets.is_empty() {
                    rsx!( div { class: "bg-slate-800 rounded-lg p-12 text-center",
                        "No secrets stored in vault"
                    })
                    } else {
                        rsx!( 
                            div { class: "space-y-3",
                            for (website, secret) in list.into_iter() {
                                {
                                    let site_str = website.0.clone();
                                    let mut show = use_signal(|| false);
                                    rsx!( 
                                        div { key: "{site_str}", class: "bg-slate-800 rounded-lg p-6 border border-slate-700",
                                        div { class: "flex items-start justify-between mb-4",
                                            div { h3 { class: "text-white text-lg mb-1", "{site_str}" } p { class: "text-slate-400 text-sm", "{secret.username}" } }
                                            button { onclick: move |_| {
                                                let site = vault::Website(site_str.clone());
                                                match storage::load_vault_from_file() {
                                                    Ok(mut v) => {
                                                        v.remove_from_vault(&site);
                                                        match storage::load_vault_key() {
                                                            Ok(key) => {
                                                                if let Err(e) = storage::save_vault_to_file(&v, key) {
                                                                    status.set(format!("Failed to save vault: {}", e));
                                                                    return;
                                                                }
                                                                vault.set(v);
                                                            }
                                                            Err(e) => { status.set(format!("Failed to load vault key: {}", e)); return; }
                                                        }
                                                    }
                                                    Err(_) => { status.set("Vault not initialized. Please initialize vault first.".to_string()); return; }
                                                }
                                                status.set(format!("Secret for {} deleted", site.0));
                                            }, class: "text-red-400 hover:text-red-300 p-2 hover:bg-slate-700 rounded transition-colors", Trash2 {color: "red", size: 18,}, }
                                        }

                                        div { class: "grid grid-cols-2 gap-4",
                                            div {
                                                label { class: "text-slate-400 text-sm block mb-2", "Username" }
                                                div { class: "flex items-center gap-2",
                                                    input { r#type: "text", value: "{secret.username}", readonly: true, class: "flex-1 bg-slate-700 text-white px-3 py-2 rounded border border-slate-600" }
                                                    button { onclick: move |_| status.set("Username copied (demo)".to_string()), class: "p-2 text-slate-400 hover:text-white hover:bg-slate-700 rounded transition-colors", Copy {color: "white", size: 18,}, }
                                                }
                                            }
                                            div {
                                                label { class: "text-slate-400 text-sm block mb-2", "Password" }
                                                div { class: "flex items-center gap-2",
                                                    input { r#type: if show() { "text" } else { "password" }, value: "{secret.password}", readonly: true, class: "flex-1 bg-slate-700 text-white px-3 py-2 rounded border border-slate-600" }
                                                    button { onclick: move |_| show.set(!show()), class: "p-2 text-slate-400 hover:text-white hover:bg-slate-700 rounded transition-colors",
                                                        { if show() {
                                                            rsx!( Eye {color: "white", size: 18,} )
                                                        } else {
                                                            rsx!( EyeOff {color: "white", size: 18,} )
                                                        } }
                                                    }
                                                    button { onclick: move |_| status.set("Password copied (demo)".to_string()), class: "p-2 text-slate-400 hover:text-white hover:bg-slate-700 rounded transition-colors", Copy {color: "white", size: 18,}, }
                                                }
                                            }
                                        }
                                    })
                                }
                            }
                        })
                    }
                }
            }
        }
    }
}
