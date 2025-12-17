use dioxus::prelude::*;
use lucide_dioxus::{ Lock, List, Plus, Search, SquarePen, Trash2, Key };
use crate::views::{ListView, AddView, GetView, UpdateView, DeleteView, InitView};

#[derive(Clone, PartialEq)]
enum View {
    List,
    Add,
    Get,
    Update,
    Delete,
    Init,
}

#[component]
pub fn VaultGUI() -> Element {
    let mut active_view = use_signal(|| View::List);
    let mut status_message = use_signal(|| String::new());

    rsx! {
        // Sidebar
        div { class: "flex h-screen",
            div { class: "w-64 bg-slate-800 border-r border-slate-700 flex flex-col",
                div { class: "p-6 border-b border-slate-700",
                    div { class: "flex items-center gap-3",
                        div { class: "w-10 h-10 bg-blue-600 rounded-lg flex items-center justify-center",
                            Lock {color: "white", size: 24,},
                        }
                        div {
                            h1 { class: "text-white", "RSM" }
                            p { class: "text-slate-400 text-sm", "Secrets Manager" }
                        }
                    }
                }

                // List of view buttons
                nav { class: "flex-1 p-4",
                    div { class: "space-y-1",
                        button {
                            onclick: move |_| active_view.set(View::List),
                            class: "w-full flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-slate-300 hover:bg-slate-700",
                            List {color: "white", size: 18,}, "All Secrets"
                        }
                        button {
                            onclick: move |_| active_view.set(View::Add),
                            class: "w-full flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-slate-300 hover:bg-slate-700",
                            Plus {color: "white", size: 18,}, "Add Secret"
                        }
                        button {
                            onclick: move |_| active_view.set(View::Get),
                            class: "w-full flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-slate-300 hover:bg-slate-700",
                            Search {color: "white", size: 18,}, "Get Secret"
                        }
                        button {
                            onclick: move |_| active_view.set(View::Update),
                            class: "w-full flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-slate-300 hover:bg-slate-700",
                            SquarePen {color: "white", size: 18,}, "Update Secret"
                        }
                        button {
                            onclick: move |_| active_view.set(View::Delete),
                            class: "w-full flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-slate-300 hover:bg-slate-700",
                            Trash2 {color: "white", size: 18,}, "Delete Secret"
                        }
                    }
                }

                // Bottom Navigation - Initialize Button and Vault and Key Paths
                div { class: "p-4 border-t border-slate-700",
                    button {
                        onclick: move |_| active_view.set(View::Init),
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
                    rsx!(
                        div { class: "bg-blue-600 text-white px-6 py-3 flex items-center gap-2",
                            span { class: "flex-1", "{status_message}" }
                            button {
                                onclick: move |_| status_message.set(String::new()),
                                class: "ml-auto text-slate-200 hover:text-white bg-transparent p-1 rounded",
                                aria_label: "Dismiss",
                                "Dismiss"
                            }
                        }
                    )
                } else {
                    rsx!()
                } }

                div { class: "flex-1 overflow-auto",
                    match active_view(){
                        View::List => rsx!( ListView { status: status_message.clone()} ),
                        View::Add => rsx!( AddView { status: status_message.clone()} ),
                        View::Get => rsx!( GetView { status: status_message.clone()} ),
                        View::Update => rsx!( UpdateView { status: status_message.clone()} ),
                        View::Delete => rsx!( DeleteView { status: status_message.clone()} ),
                        View::Init => rsx!( InitView { status: status_message.clone()} ),
                    }
                }
            }
        }
    }
}