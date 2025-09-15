use crate::versioning::{VERSIONS, use_version_context};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn ChevronDownIcon(#[prop(optional)] class: Option<String>) -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class=class
        >
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
        Some(ref ctx) => {
            // Sync context with URL only once on mount
            let location = leptos_router::hooks::use_location();
            let ctx_clone = ctx.clone();

            // Run once to sync URL with context
            let current_path = location.pathname.get_untracked();
            let url_version = if current_path.starts_with("/dev") {
                crate::versioning::get_version_by_path("dev")
            } else if current_path.starts_with("/v") {
                let trimmed = current_path.trim_start_matches('/');
                let parts: Vec<&str> = trimmed.splitn(2, '/').collect();
                if !parts.is_empty() && parts[0].starts_with("v") {
                    crate::versioning::get_version_by_path(parts[0])
                } else {
                    None
                }
            } else {
                None
            };

            // Update context if URL has a different version (only on mount)
            if let Some(url_ver) = url_version {
                let current_ver = ctx_clone.current.get_untracked();
                if current_ver.path != url_ver.path {
                    ctx_clone.set_version.set(url_ver.clone());
                }
            }

            ctx.current
        }
        None => {
            // Fallback if no version context (shouldn't happen since we provide it at app level)
            Signal::derive(move || crate::versioning::get_current_version().clone())
        }
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
                class="inline-flex items-center justify-between gap-x-1.5 px-3 h-8 text-sm font-medium dark:text-zinc-100 text-zinc-700 ring-1 ring-inset ring-zinc-300 hover:bg-zinc-50 dark:bg-zinc-800  dark:ring-zinc-700 dark:hover:bg-zinc-700"
                on:click=move |_| set_show_dropdown.update(|v| *v = !*v)
            >
                <span>{move || current_version.get().label}</span>
                <ChevronDownIcon class="h-5 w-5 text-zinc-400".to_string() />
            </button>

            <Show when=move || show_dropdown.get()>
                <div class="absolute right-0 z-10 mt-2 w-48 origin-top-right shadow-lg ring-1 ring-zinc-300 ring-opacity-5 focus:outline-none bg-white dark:bg-zinc-800 dark:ring-zinc-700">
                    <div role="menu">
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
                                            class=move || {
                                                format!(
                                                    "cursor-pointer block w-full px-4 py-2 text-left text-sm hover:bg-zinc-100 dark:hover:bg-zinc-700 {}",
                                                    if is_current() {
                                                        "bg-zinc-50 text-zinc-900 font-medium dark:bg-zinc-700/90 dark:text-zinc-100"
                                                    } else {
                                                        "text-zinc-700 dark:text-zinc-300"
                                                    },
                                                )
                                            }
                                            role="menuitem"
                                            on:click=move |_| {
                                                if let Some(ref ctx) = version_ctx
                                                    && let Some(version) = VERSIONS
                                                        .iter()
                                                        .find(|v| v.path == path)
                                                {
                                                    ctx.set_version.set(version.clone());
                                                    set_show_dropdown.set(false);
                                                    let current_path = leptos_router::hooks::use_location()
                                                        .pathname
                                                        .get();
                                                    let new_path = if current_path.starts_with("/v") || current_path.starts_with("/dev") {
                                                        let trimmed = current_path.trim_start_matches('/');
                                                        let path_parts: Vec<&str> = trimmed.splitn(2, '/').collect();
                                                        if path_parts.len() > 1 {
                                                            format!("/{}/{}", path, path_parts[1])
                                                        } else {
                                                            format!("/{}", path)
                                                        }
                                                    } else {
                                                        format!("/{}", path)
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
                                }
                            }
                        />
                    </div>
                </div>
            </Show>
        </div>
    }
}
