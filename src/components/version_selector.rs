use crate::versioning::{VERSIONS, use_version_context};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn ChevronDownIcon(#[prop(optional)] class: Option<String>) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class>
            <path d="m6 9 6 6 6-6"></path>
        </svg>
    }
}

#[component]
pub fn VersionSelector() -> impl IntoView {
    let version_ctx = use_version_context();
    let (show_dropdown, set_show_dropdown) = signal(false);
    let navigate = use_navigate();

    let current_version = match version_ctx {
        Some(ref ctx) => ctx.current,
        None => Signal::derive(|| crate::versioning::get_current_version().clone()),
    };

    // Close dropdown when clicking outside
    let dropdown_ref = NodeRef::<leptos::html::Div>::new();
    let _ = leptos_use::on_click_outside(dropdown_ref, move |_| {
        set_show_dropdown.set(false);
    });

    view! {
        <div class="relative inline-block text-left" node_ref=dropdown_ref>
            <button
                type="button"
                class="inline-flex items-center justify-between gap-x-1.5 rounded-md bg-white px-3 py-2 text-sm font-medium text-zinc-900 ring-1 ring-inset ring-zinc-300 hover:bg-zinc-50 dark:bg-zinc-800 dark:text-zinc-100 dark:ring-zinc-700 dark:hover:bg-zinc-700"
                on:click=move |_| set_show_dropdown.update(|v| *v = !*v)
            >
                <span>
                    {move || current_version.get().label}
                </span>
                <ChevronDownIcon class="h-5 w-5 text-zinc-400".to_string()/>
            </button>

            <Show when=move || show_dropdown.get()>
                <div class="absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none dark:bg-zinc-800 dark:ring-zinc-700">
                    <div class="py-1" role="menu">
                        <For
                            each=|| VERSIONS.iter()
                            key=|version| version.path
                            children={
                                let navigate = navigate.clone();
                                let version_ctx = version_ctx.clone();
                                move |version| {
                                    let path = version.path;
                                    let label = version.label;
                                    let is_latest = version.is_latest;
                                    let is_current = move || current_version.get().path == path;
                                    let navigate = navigate.clone();
                                    let version_ctx = version_ctx.clone();

                                view! {
                                    <button
                                        class=move || format!(
                                            "block w-full px-4 py-2 text-left text-sm hover:bg-zinc-100 dark:hover:bg-zinc-700 {}",
                                            if is_current() {
                                                "bg-zinc-50 text-zinc-900 font-medium dark:bg-zinc-700 dark:text-zinc-100"
                                            } else {
                                                "text-zinc-700 dark:text-zinc-300"
                                            }
                                        )
                                        role="menuitem"
                                        on:click=move |_| {
                                            if let Some(ref ctx) = version_ctx
                                                && let Some(version) = VERSIONS.iter().find(|v| v.path == path) {
                                                    ctx.set_version.set(version.clone());
                                                    set_show_dropdown.set(false);

                                                    // Navigate to the new version's docs
                                                    let current_path = leptos_router::hooks::use_location().pathname.get();
                                                    let new_path = if current_path.contains("/docs/") {
                                                        let path_parts: Vec<&str> = current_path.split('/').collect();
                                                        if path_parts.len() > 3 {
                                                            // Has a page after version: /docs/v0.1/quickstart
                                                            format!("/docs/{}/{}", path, path_parts[3..].join("/"))
                                                        } else {
                                                            // Just version root: /docs/v0.1
                                                            format!("/docs/{}", path)
                                                        }
                                                    } else {
                                                        format!("/docs/{}", path)
                                                    };
                                                    navigate(&new_path, Default::default());
                                                }
                                        }
                                    >
                                        <span class="flex items-center justify-between">
                                            <span>{label}</span>
                                            <Show when=move || is_latest>
                                                <span class="ml-2 inline-flex items-center rounded px-1.5 py-0.5 text-xs font-medium bg-emerald-50 text-emerald-700 ring-1 ring-inset ring-emerald-600/20 dark:bg-emerald-400/10 dark:text-emerald-400 dark:ring-emerald-400/30">
                                                    "latest"
                                                </span>
                                            </Show>
                                        </span>
                                    </button>
                                }
                            }}
                        />
                    </div>
                </div>
            </Show>
        </div>
    }
}
