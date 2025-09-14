use crate::{
    DocPage, Layout,
    api::{ApiLandingPage, ApiRefPage},
    versioning::{get_current_version, provide_version_context},
};
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    components::{Redirect, Route, Router, Routes},
    path,
};
use leptos_use::{ColorMode, use_preferred_dark};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="flex min-h-screen">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Provide version context
    provide_version_context();

    // Create auto mode state - None means auto, Some means manual override
    let (theme_override, set_theme_override) = signal::<Option<ColorMode>>(None);

    // Get system preference
    let prefers_dark = use_preferred_dark();

    // Compute the actual mode based on override or system preference
    let mode = Signal::derive(move || {
        theme_override.get().unwrap_or_else(|| {
            if prefers_dark.get() {
                ColorMode::Dark
            } else {
                ColorMode::Light
            }
        })
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/ricochet-docs.css"/>

        // sets the document title
        <Title text="📓 Docs ricochet"/>

        // content for this welcome page
        <Router>
            <Routes fallback=move || view! { <Layout mode=mode theme_override=theme_override set_theme_override=set_theme_override/> }>
                <Route
                    path=path!("/api")
                    view=move || {
                        view! { <ApiLandingPage mode=mode theme_override=theme_override set_theme_override=set_theme_override/> }
                    }
                />


                <Route
                    path=path!("/")
                    view=move || {
                        let default_path = format!("/{}", get_current_version().path);
                        view! { <Redirect path=default_path/> }
                    }
                />
                <Route
                    path=path!("/:path")
                    view=move || view! { <DocPage mode=mode theme_override=theme_override set_theme_override=set_theme_override/> }
                />
                <Route
                    path=path!("/v0.1")
                    view=move || view! { <DocPage mode=mode theme_override=theme_override set_theme_override=set_theme_override/> }
                />
                <Route
                    path=path!("/v0.1/:path")
                    view=move || view! { <DocPage mode=mode theme_override=theme_override set_theme_override=set_theme_override/> }
                />
                <Route
                    path=path!("/dev")
                    view=move || view! { <DocPage mode=mode theme_override=theme_override set_theme_override=set_theme_override/> }
                />
                <Route
                    path=path!("/dev/:path")
                    view=move || view! { <DocPage mode=mode theme_override=theme_override set_theme_override=set_theme_override/> }
                />
                <Route
                    path=path!("/api/:path")
                    view=move || {
                        view! { <ApiRefPage mode=mode theme_override=theme_override set_theme_override=set_theme_override/> }
                    }
                />

            </Routes>
        </Router>
    }
}
