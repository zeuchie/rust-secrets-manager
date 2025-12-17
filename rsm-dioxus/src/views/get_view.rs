use dioxus::prelude::*;
use lucide_dioxus::{Copy, Eye, EyeOff, Search};
use rsm_core::*;

#[component]
pub fn GetView(mut status: Signal<String>) -> Element {
    let mut website = use_signal(|| String::new());
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut show_password = use_signal(|| false);

    rsx! {
        div { class: "p-8",
            div { class: "max-w-2xl",
                h2 { class: "text-white text-2xl mb-2", "Get Secret" }
                p { class: "text-slate-400 mb-6", "Retrieve a secret for a website" }

                form { onsubmit: move |e| {
                    e.prevent_default();
                    let w = website();
                    match storage::load_vault_from_file() {
                        Ok(mut v) => {
                            let maybe = v.get_secret(&vault::Website(w.clone()));
                            if let Some(secret) = maybe.cloned() {
                                username.set(secret.username);
                                password.set(secret.password);
                                status.set(format!("Secret for {} retrieved", w));
                            } else {
                                // Clear previous displayed values
                                username.set(String::new());
                                password.set(String::new());
                                status.set(format!("No secret found for {}", w));
                            }
                        }
                        Err(_) => {
                            status.set("Vault not initialized. Please initialize vault first.".to_string());
                        }
                    }
                }, class: "bg-slate-800 rounded-lg p-6 border border-slate-700 mb-6",
                    div { class: "flex gap-3",
                        input { r#type: "text", value: "{website}", oninput: move |e| website.set(e.value()), placeholder: "Enter website (https://www.example.com)", class: "flex-1 bg-slate-700 text-white px-4 py-3 rounded border border-slate-600", autocorrect: "off", autocapitalize: "off", spellcheck: "false", autocomplete: "off" }
                        button { r#type: "submit", class: "bg-blue-600 hover:bg-blue-700 text-white px-8 py-3 rounded flex items-center justify-center gap-2", Search {color: "white", size: 18,}, "Search" }
                    }
                }

                { if !username().is_empty() {
                    rsx!( div { class: "bg-slate-800 rounded-lg p-6 border border-slate-700",
                        h3 { class: "text-white text-lg mb-4", "{website}" }
                        div { class: "space-y-4",
                            div { label { class: "text-slate-400 text-sm block mb-2", "Username" } div { class: "flex items-center gap-2", input { r#type: "text", value: "{username}", readonly: true, class: "flex-1 bg-slate-700 text-white px-3 py-2 rounded border border-slate-600" } button { onclick: move |_| {
                                        let u = username();
                                        if !u.is_empty() {
                                            let _ = cli_clipboard::set_contents(u);
                                            status.set("Username copied to clipboard".to_string());
                                        }
                                    }, class: "p-2 text-slate-400", Copy {color: "white", size: 18,}, } } }
                            div { label { class: "text-slate-400 text-sm block mb-2", "Password" } div { class: "flex items-center gap-2", input { r#type: if show_password() { "text" } else { "password" }, value: "{password}", readonly: true, class: "flex-1 bg-slate-700 text-white px-3 py-2 rounded border border-slate-600" } button { onclick: move |_| show_password.set(!show_password()), class: "p-2 text-slate-400",
                                    { if show_password() {
                                        rsx!( Eye {color: "white", size: 18,} )
                                    } else {
                                        rsx!( EyeOff {color: "white", size: 18,} )
                                    } }
                                } button { onclick: move |_| {
                                        let password = password();
                                        if !password.is_empty() {
                                            let _ = cli_clipboard::set_contents(password);
                                            status.set("Password copied to clipboard".to_string());
                                        }
                                    }, class: "p-2 text-slate-400", Copy {color: "white", size: 18,}, }
                            } }
                        }
                    })
                } else { rsx!() } }
            }
        }
    }
}
