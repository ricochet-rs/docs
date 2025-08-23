use crate::{
    DocPage, Index, Layout,
    api::{ApiLandingPage, ApiRefPage},
    landing::LandingPage,
};
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
    path,
};
use leptos_use::ColorMode;

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
    let (mode, set_mode) = signal(ColorMode::Dark);
    view! {
        <Stylesheet id="leptos" href="/pkg/ricochet-docs.css"/>

        // sets the document title
        <Title text="📓 Docs ricochet"/>

        // content for this welcome page
        <Router>
            <Routes fallback=move || view! { <Layout mode=mode set_mode=set_mode/> }>
                <Route
                    path=path!("/api")
                    view=move || {
                        view! { <ApiLandingPage mode=mode set_mode=set_mode/> }
                    }
                />

                <Route
                    path=StaticSegment("/hello")
                    view=move || view! { <Index mode=mode set_mode=set_mode/> }
                />
                <Route
                    path=path!("/")
                    view=move || view! { <LandingPage mode=mode set_mode=set_mode/> }
                />

                <Route
                    path=path!("/:path")
                    view=move || view! { <DocPage mode=mode set_mode=set_mode/> }
                />
                <Route
                    path=path!("/docs/:path")
                    view=move || view! { <DocPage mode=mode set_mode=set_mode/> }
                />
                <Route
                    path=path!("/api/:path")
                    view=move || {
                        view! { <ApiRefPage mode=mode set_mode=set_mode/> }
                    }
                />

            </Routes>
        </Router>
    }
}
