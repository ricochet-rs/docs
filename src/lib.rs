#![recursion_limit = "256"]
pub mod app;

use components::{
    // code::CodeGroupHeader,
    footer::Footer,
    // hero_pattern::HeroPattern,
    navigation::{Header, Navigation},
};
use docs::{DocNavItem, get_doc, get_doc_for_version};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_location;
use leptos_use::ColorMode;
use versioning::get_version_by_path;

// Modules
pub mod api;
pub mod components;
pub mod docs;
pub mod landing;
pub mod search_engine;
pub mod versioning;

#[component]
pub fn HomeButton() -> AnyView {
    use leptos_router::hooks::use_location;
    
    let location = use_location();
    let href = Signal::derive(move || {
        let current_path = location.pathname.get();
        if current_path.starts_with("/v") || current_path.starts_with("/dev") {
            let trimmed = current_path.trim_start_matches('/');
            let parts: Vec<&str> = trimmed.splitn(2, '/').collect();
            if !parts.is_empty() && (parts[0].starts_with("v") || parts[0] == "dev") {
                format!("/{}/", parts[0])
            } else {
                format!("/{}/", versioning::get_current_version().path)
            }
        } else {
            format!("/{}/", versioning::get_current_version().path)
        }
    });
    
    view! {
        <a href=move || href.get() aria-label="Home">
            <p class="font-mono font-bold text-zinc-900 dark:text-zinc-100">ricochet</p>
        </a>
    }
    .into_any()
}

#[component]
pub fn Layout(
    mode: Signal<ColorMode>,
    theme_override: ReadSignal<Option<ColorMode>>,
    set_theme_override: WriteSignal<Option<ColorMode>>,
    #[prop(optional)] doc: Option<DocNavItem>,
) -> impl IntoView {
    let DocNavItem {
        title,
        body,
        prev_slug,
        next_slug,
    } = doc.unwrap_or_default();

    // let UseColorModeReturn { mode, set_mode, .. } = use_color_mode();

    let dark_mode_class = move || match mode.get() {
        ColorMode::Dark => "dark w-full",
        _ => "w-full",
    };

    let title = format!("{title} | ricochet 🐇");
    view! {
        <Title text=title/>
        <div class=dark_mode_class>
            <div class="flex-auto h-full w-full bg-zinc-100/50 antialiased dark:bg-zinc-900">
                <div class="h-full lg:ml-72 xl:ml-80">
                    // Header section
                    <header class="contents lg:pointer-events-none lg:fixed lg:inset-0 lg:z-40 lg:flex">
                        <div class="contents lg:pointer-events-auto lg:block lg:w-72 lg:overflow-y-auto lg:border-r lg:border-zinc-900/10 lg:px-6 lg:pb-8 lg:pt-4 lg:dark:border-white/10 xl:w-80">
                            <div class="hidden lg:flex">
                                <HomeButton/>
                            </div>
                            <Header mode=mode theme_override=theme_override set_theme_override=set_theme_override/>
                            <Navigation class="hidden lg:mt-10 lg:block"/>
                        </div>
                    </header>

                    // <HeroPattern/>
                    // Main content area
                    <div class="relative flex h-full flex-col px-4 pt-14 sm:px-6 lg:px-8">
                        <main class="flex-auto py-16">
                            <div class="h-full mx-auto !max-w-none prose lg:prose-lg prose-zinc dark:prose-invert w-full
                            prose-code:before:hidden prose-code:after:hidden prose-code:rounded-none
                            prose-h1:text-3xl lg:prose-h1:text-4xl
                            prose-pre:rounded-none
                            prose-li:my-0 prose-ul:my-1
                            prose-pre:dark:ring-1 prose-pre:dark:ring-white/10 prose-pre:dark:ring-inset prose-pre:shadow-md
                            prose-a:decoration-violet-500 prose-a:decoration-dotted prose-a:dark:hover:bg-violet-500 prose-a:hover:bg-violet-600 prose-a:hover:text-white prose-a:hover:decoration-violet-600 prose-a:dark:hover:decoration-violet-500
                            ">
                                <div class="h-full w-full max-w-2xl lg:max-w-5xl mx-auto">
                                    <article
                                        inner_html=body
                                        class="w-full h-full max-w-3xl"
                                    ></article>
                                </div>
                            </div>
                        </main>
                        <Footer prev=prev_slug next=next_slug/>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn DocPage(
    mode: Signal<ColorMode>,
    theme_override: ReadSignal<Option<ColorMode>>,
    set_theme_override: WriteSignal<Option<ColorMode>>,
) -> AnyView {
    let loc = use_location();
    let path = move || loc.pathname.get();

    view! {
        {move || {
            let p = path();

            // Extract version from path if present
            let (version, doc_path) = if p.starts_with("/v") || p.starts_with("/dev") {
                // Remove leading slash for easier parsing
                let trimmed = p.trim_start_matches('/');
                let parts: Vec<&str> = trimmed.splitn(2, '/').collect();
                
                if !parts.is_empty() {
                    let version_str = parts[0]; // e.g., "v0.1" or "dev"
                    let doc_path = if parts.len() > 1 {
                        format!("/{}", parts[1])
                    } else {
                        "/".to_string()
                    };
                    (get_version_by_path(version_str), doc_path)
                } else {
                    (None, p.to_string())
                }
            } else {
                (None, p.to_string())
            };

            let item = match version {
                Some(v) => get_doc_for_version(&doc_path, v),
                None => get_doc(&doc_path),
            };

            match item {
                Some(doc) => view! { <Layout doc=doc mode=mode theme_override=theme_override set_theme_override=set_theme_override/> }.into_any(),
                None => view! { <Layout mode=mode theme_override=theme_override set_theme_override=set_theme_override/> }.into_any(),
            }
        }}
    }
    .into_any()
}


#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
