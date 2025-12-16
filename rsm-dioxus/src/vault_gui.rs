use dioxus::prelude::*;
use lucide_dioxus::{ Lock, List, Plus, Search, SquarePen, Trash2, Eye, EyeOff, Copy, Check, X, Settings, Key };

#[derive(Clone, PartialEq)]
enum ActiveView {
    List,
    Add,
    Get,
    Update,
    Delete,
    Init,
}

#[derive(Clone, PartialEq)]
pub struct Secret {
    pub website: String,
    pub username: String,
    pub password: String,
}

#[component]
pub fn VaultGUI() -> Element {
    let mut active_view = use_signal(|| ActiveView::List);
    let _vault_initialized = use_signal(|| true);

    let secrets = use_signal(|| {
        vec![
            Secret {
                website: "github.com".into(),
                username: "user@example.com".into(),
                password: "example_password_123".into(),
            },
            Secret {
                website: "gmail.com".into(),
                username: "myemail@gmail.com".into(),
                password: "secure_pass_456".into(),
            },
        ]
    });

    let status_message = use_signal(|| String::new());

    rsx! {
        div { class: "flex h-screen",
            // Sidebar
            div { class: "w-64 bg-slate-800 border-r border-slate-700 flex flex-col",
                div { class: "p-6 border-b border-slate-700",
                    div { class: "flex items-center gap-3",
                        div { class: "w-10 h-10 bg-blue-600 rounded-lg flex items-center justify-center",
                            Lock {color: "white", size: 24,},
                        }
                        div {
                            h1 { class: "text-white", "RSM" }
                            p { class: "text-slate-400 text-sm", "Rust Secrets Manager" }
                        }
                    }
                }

                nav { class: "flex-1 p-4",
                    div { class: "space-y-1",
                        button {
                            onclick: move |_| active_view.set(ActiveView::List),
                            class: "w-full flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-slate-300 hover:bg-slate-700",
                            List {color: "white", size: 18,}, "All Secrets"
                        }
                        button {
                            onclick: move |_| active_view.set(ActiveView::Add),
                            class: "w-full flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-slate-300 hover:bg-slate-700",
                            Plus {color: "white", size: 18,}, "Add Secret"
                        }
                        button {
                            onclick: move |_| active_view.set(ActiveView::Get),
                            class: "w-full flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-slate-300 hover:bg-slate-700",
                            Search {color: "white", size: 18,}, "Get Secret"
                        }
                        button {
                            onclick: move |_| active_view.set(ActiveView::Update),
                            class: "w-full flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-slate-300 hover:bg-slate-700",
                            SquarePen {color: "white", size: 18,}, "Update Secret"
                        }
                        button {
                            onclick: move |_| active_view.set(ActiveView::Delete),
                            class: "w-full flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-slate-300 hover:bg-slate-700",
                            Trash2 {color: "white", size: 18,}, "Delete Secret"
                        }
                    }
                }

                div { class: "p-4 border-t border-slate-700",
                    button {
                        onclick: move |_| active_view.set(ActiveView::Init),
                        class: "w-full flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-slate-300 hover:bg-slate-700",
                        "Initialize Vault"
                    }
                    div { class: "mt-4 p-3 bg-slate-700/50 rounded-lg",
                        div { class: "flex items-center gap-2 text-slate-400 text-xs",
                            Lock {color: "white", size: 12,}, "~/vault.rsm"
                        }
                        div { class: "flex items-center gap-2 text-slate-400 text-xs",
                            Key {color: "white", size: 12,}, "~/.ssh/rsm-key"
                        }
                    }
                }
            }

            // Main
            div { class: "flex-1 flex flex-col bg-slate-900",
                { if !status_message().is_empty() {
                    if status_message().contains("No secret found") {
                        rsx!(
                            div { class: "bg-red-600 text-white px-6 py-3 flex items-center gap-2",
                                X {color: "white", size: 18,},
                                span { "{status_message}" }
                            }
                        )
                    } else {
                        rsx!(
                            div { class: "bg-green-600 text-white px-6 py-3 flex items-center gap-2",
                                Check {color: "white", size: 18,},
                                span { "{status_message}" }
                            }
                        )
                    }
                } else {
                    rsx!()
                } }

                div { class: "flex-1 overflow-auto",
                    match active_view().clone() {
                        ActiveView::List => rsx!( ListView { secrets: secrets.clone(), status: status_message.clone() } ),
                        ActiveView::Add => rsx!( AddView { secrets: secrets.clone(), status: status_message.clone() } ),
                        ActiveView::Get => rsx!( GetView { secrets: secrets.clone(), status: status_message.clone() } ),
                        ActiveView::Update => rsx!( UpdateView { secrets: secrets.clone(), status: status_message.clone() } ),
                        ActiveView::Delete => rsx!( DeleteView { secrets: secrets.clone(), status: status_message.clone() } ),
                        ActiveView::Init => rsx!( InitView { status: status_message.clone() } ),
                    }
                }
            }
        }
    }
}

#[component]
fn ListView(mut secrets: Signal<Vec<Secret>>, mut status: Signal<String>) -> Element {
    let list = secrets();

    rsx!{
        div { class: "p-8",
            div { class: "max-w-4xl",
                h2 { class: "text-white text-2xl mb-2", "All Secrets" }
                p { class: "text-slate-400 mb-6", "Manage all the secrets stored in your vault" }

                { if list.is_empty() {
                    rsx!( div { class: "bg-slate-800 rounded-lg p-12 text-center",
                        "No secrets stored in vault"
                    })
                } else {
                    rsx!( div { class: "space-y-3",
                        for secret in list.iter().cloned() {
                            {
                                let site = secret.website.clone();
                                    let mut show = use_signal(|| false);
                                rsx!( div { key: "{site}", class: "bg-slate-800 rounded-lg p-6 border border-slate-700",
                                    div { class: "flex items-start justify-between mb-4",
                                        div { h3 { class: "text-white text-lg mb-1", "{secret.website}" } p { class: "text-slate-400 text-sm", "{secret.username}" } }
                                        button { onclick: move |_| {
                                            let site = site.clone();
                                            secrets.with_mut(|v| v.retain(|s| s.website != site));
                                            status.set(format!("Secret for {} deleted", site));
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
                } }
            }
        }
    }
}

#[component]
fn AddView(mut secrets: Signal<Vec<Secret>>, mut status: Signal<String>) -> Element {
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
                    secrets.with_mut(|v| v.push(Secret { website: site.clone(), username: user, password: pass }));
                    status.set(format!("Secret for {} added successfully", site));
                    website.set(String::new()); username.set(String::new()); password.set(String::new());
                }, class: "bg-slate-800 rounded-lg p-6 border border-slate-700",
                    div { class: "space-y-4",
                        div { label { class: "text-white block mb-2", "Website" } input { r#type: "text", value: "{website}", oninput: move |e| website.set(e.value()), placeholder: "https://www.example.com", class: "w-full bg-slate-700 text-white px-4 py-3 rounded border border-slate-600" } }
                        div { label { class: "text-white block mb-2", "Username" } input { r#type: "text", value: "{username}", oninput: move |e| username.set(e.value()), placeholder: "", class: "w-full bg-slate-700 text-white px-4 py-3 rounded border border-slate-600" } }
                        div { label { class: "text-white block mb-2", "Password" } div { class: "relative", input { r#type: "password", value: "{password}", oninput: move |e| password.set(e.value()), placeholder: "", class: "w-full bg-slate-700 text-white px-4 py-3 rounded border border-slate-600 pr-12" } } }
                        button { r#type: "submit", class: "w-full bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded transition-colors flex items-center justify-center gap-2", Plus {color: "white", size: 18,}, "Add Secret" }
                    }
                }
            }
        }
    }
}

#[component]
fn GetView(secrets: Signal<Vec<Secret>>, mut status: Signal<String>) -> Element {
    let mut website = use_signal(|| String::new());
    let mut found = use_signal(|| None as Option<Secret>);
    let mut show_password = use_signal(|| false);

    let list = secrets();

    rsx!{
        div { class: "p-8",
            div { class: "max-w-2xl",
                h2 { class: "text-white text-2xl mb-2", "Get Secret" }
                p { class: "text-slate-400 mb-6", "Retrieve a secret for a website" }

                form { onsubmit: move |e| {
                    e.prevent_default();
                    let site = website();
                    let maybe = list.iter().find(|s| s.website == site).cloned();
                    found.set(maybe.clone());
                    if maybe.is_some() { status.set(format!("Secret for {} retrieved", site)); } else { status.set(format!("No secret found for {}", site)); }
                }, class: "bg-slate-800 rounded-lg p-6 border border-slate-700 mb-6",
                    div { class: "flex gap-3",
                        input { r#type: "text", value: "{website}", oninput: move |e| website.set(e.value()), placeholder: "Enter website (https://www.example.com)", class: "flex-1 bg-slate-700 text-white px-4 py-3 rounded border border-slate-600" }
                        button { r#type: "submit", class: "bg-blue-600 hover:bg-blue-700 text-white px-8 py-3 rounded flex items-center justify-center gap-2", Search {color: "white", size: 18,}, "Search" }
                    }
                }

                { if let Some(s) = found().clone() {
                    rsx!( div { class: "bg-slate-800 rounded-lg p-6 border border-slate-700",
                        h3 { class: "text-white text-lg mb-4", "{s.website}" }
                        div { class: "space-y-4",
                            div { label { class: "text-slate-400 text-sm block mb-2", "Username" } div { class: "flex items-center gap-2", input { r#type: "text", value: "{s.username}", readonly: true, class: "flex-1 bg-slate-700 text-white px-3 py-2 rounded border border-slate-600" } button { onclick: move |_| status.set("Username copied (demo)".to_string()), class: "p-2 text-slate-400", Copy {color: "white", size: 18,}, } } }
                            div { label { class: "text-slate-400 text-sm block mb-2", "Password" } div { class: "flex items-center gap-2", input { r#type: if show_password() { "text" } else { "password" }, value: "{s.password}", readonly: true, class: "flex-1 bg-slate-700 text-white px-3 py-2 rounded border border-slate-600" } button { onclick: move |_| show_password.set(!show_password()), class: "p-2 text-slate-400",
                                    { if show_password() {
                                        rsx!( Eye {color: "white", size: 18,} )
                                    } else {
                                        rsx!( EyeOff {color: "white", size: 18,} )
                                    } }
                                } button { onclick: move |_| status.set("Password copied (demo)".to_string()), class: "p-2 text-slate-400", Copy {color: "white", size: 18,}, } } }
                        }
                    })
                } else { rsx!() } }
            }
        }
    }
}

#[component]
fn UpdateView(mut secrets: Signal<Vec<Secret>>, mut status: Signal<String>) -> Element {
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
                    let site = website();
                    let mut found = false;
                    secrets.with_mut(|v| {
                        for s in v.iter_mut() {
                            if s.website == site {
                                s.username = username();
                                s.password = password();
                                found = true;
                                break;
                            }
                        }
                    });
                    if !found {
                        status.set(format!("No secret found for {}", site));
                        return;
                    }
                    status.set(format!("Secret for {} updated successfully", site));
                    website.set(String::new()); username.set(String::new()); password.set(String::new());
                }, class: "bg-slate-800 rounded-lg p-6 border border-slate-700",
                    div { class: "space-y-4",
                        div { label { class: "text-white block mb-2", "Website" } input { r#type: "text", value: "{website}", oninput: move |e| website.set(e.value()), placeholder: "https://www.example.com", class: "w-full bg-slate-700 text-white px-4 py-3 rounded border border-slate-600" } }
                        div { label { class: "text-white block mb-2", "New Username" } input { r#type: "text", value: "{username}", oninput: move |e| username.set(e.value()), placeholder: "", class: "w-full bg-slate-700 text-white px-4 py-3 rounded border border-slate-600" } }
                        div { label { class: "text-white block mb-2", "New Password" } div { class: "relative", input { r#type: "password", value: "{password}", oninput: move |e| password.set(e.value()), placeholder: "", class: "w-full bg-slate-700 text-white px-4 py-3 rounded border border-slate-600 pr-12" } } }
                        button { r#type: "submit", class: "w-full bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded transition-colors flex items-center justify-center gap-2", SquarePen {color: "white", size: 18,}, "Update Secret" }
                    }
                }
            }
        }
    }
}

#[component]
fn DeleteView(mut secrets: Signal<Vec<Secret>>, mut status: Signal<String>) -> Element {
    let mut website = use_signal(|| String::new());

    rsx!{
        div { class: "p-8",
            div { class: "max-w-2xl",
                h2 { class: "text-white text-2xl mb-2", "Delete Secret" }
                p { class: "text-slate-400 mb-6", "Remove a secret from your vault" }

                form { onsubmit: move |e| {
                    e.prevent_default();
                    let site = website();
                    let exists = secrets().iter().any(|s| s.website == site);
                    if !exists {
                        status.set(format!("No secret found for {}", site));
                        return;
                    }
                    secrets.with_mut(|v| v.retain(|s| s.website != site));
                    status.set(format!("Secret for {} deleted successfully", site));
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

#[component]
fn InitView(status: Signal<String>) -> Element {
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

                    button { onclick: move |_| status.set("Vault initialized successfully".to_string()), class: "w-full bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded transition-colors flex items-center justify-center gap-2", Settings {color: "white", size: 18,}, "Initialize New Vault" }
                }
            }
        }
    }
}
