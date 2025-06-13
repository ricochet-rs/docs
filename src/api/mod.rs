use leptos::prelude::*;
use leptos_meta::Title;
use leptos_use::ColorMode;

use crate::{
    components::{footer::Footer, navigation::Header},
    HomeButton,
};

#[derive(Debug, Clone, Copy)]
pub enum HttpMethod {
    Get,
    Post,
    Patch,
    Delete,
}

impl HttpMethod {
    pub fn as_badge(&self) -> AnyView {
        let badge_class = match self {
            HttpMethod::Get => "font-mono text-sm font-medium  px-1.5 py-1 ring-1 ring-inset ring-emerald-300 dark:ring-emerald-400/30 bg-emerald-400/10 text-emerald-500 dark:text-emerald-400",
            HttpMethod::Post => "font-mono text-sm font-medium  px-1.5 py-1 ring-1 ring-inset ring-indigo-300 bg-indigo-400/10 text-indigo-500 dark:ring-indigo-400/30 dark:bg-indigo-400/10 dark:text-indigo-400",
            HttpMethod::Patch => "font-mono text-sm font-medium  px-1.5 py-1 ring-1 ring-inset ring-violet-300 bg-violet-400/10 text-violet-500 dark:ring-violet-400/30 dark:bg-violet-400/10 dark:text-violet-400",
            HttpMethod::Delete => "font-mono text-sm font-medium  px-1.5 py-1 ring-1 ring-inset ring-rose-200 bg-rose-50 text-red-500 dark:ring-rose-500/20 dark:bg-rose-400/10 dark:text-rose-400",
        };

        let method = match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Delete => "DELETE",
        };

        view! { <p class=format!("m-0! {badge_class}")>{method}</p> }.into_any()
    }
}

#[derive(Debug, Clone)]
pub struct ParamInfo {
    pub name: String,
    pub description: String,
}

impl ParamInfo {
    pub fn as_view(&self) -> AnyView {
        view! {
            <div class="not-prose space-y-2 mb-4">
                <p class="font-mono text-base font-semibold dark:text-white text-black">
                    {self.name.clone()}
                </p>
                <p class="text-sm mb-4">{self.description.clone()}</p>
            </div>
        }
        .into_any()
    }
}

#[derive(Debug, Clone)]
pub struct ApiEndpoint {
    pub title: String,
    pub description: String,
    pub method: HttpMethod,
    pub endpoint: String,
    pub path_params: Option<Vec<ParamInfo>>,
    pub body_params: Option<Vec<ParamInfo>>,
}

const SCROLLBAR_Y: &str = "  [&::-webkit-scrollbar]:w-2
[&::-webkit-scrollbar-track]:bg-zinc-100
[&::-webkit-scrollbar-thumb]:bg-zinc-300
dark:[&::-webkit-scrollbar-track]:bg-zinc-700
dark:[&::-webkit-scrollbar-thumb]:bg-zinc-500";

const SCROLLBAR_X: &str = "  [&::-webkit-scrollbar]:h-2
[&::-webkit-scrollbar-track]:bg-zinc-100
[&::-webkit-scrollbar-thumb]:bg-zinc-300
dark:[&::-webkit-scrollbar-track]:bg-zinc-700
dark:[&::-webkit-scrollbar-thumb]:bg-zinc-500";

impl ApiEndpoint {
    pub fn example() -> Self {
        Self {
            title: "Deploy an item".to_string(),
            method: HttpMethod::Post,
            description: "Deploy new tasks and services or update an existing one.".to_string(),
            endpoint: "/api/v0/content/upload".to_string(),
            path_params: None,
            body_params: Some(vec![
                ParamInfo {
                    name: "bundle".to_string(),
                    description: "a tar.gz file with content-type of application/x-tar".to_string(),
                },
                ParamInfo {
                    name: "config".to_string(),
                    description: "The _ricochet.toml file with content-type application/toml"
                        .to_string(),
                },
            ]),
        }
    }
}

#[component]
pub fn ApiRefLayout(
    mode: ReadSignal<ColorMode>,
    set_mode: WriteSignal<ColorMode>,
    endpoint: ApiEndpoint,
) -> impl IntoView {
    let ApiEndpoint {
        title,
        method,
        endpoint,
        description,
        path_params,
        body_params,
    } = endpoint;
    let next_slug = Some(0);
    let prev_slug = Some(0);

    // let UseColorModeReturn { mode, set_mode, .. } = use_color_mode();

    let dark_mode_class = move || match mode.get() {
        ColorMode::Dark => "dark w-full",
        _ => "w-full",
    };

    let example_code = r#"deployment <- ricochet::deploy()
url <- sprintf(
    "%s/content/%s/deployment/%s",
    ricochet::ricochet_host(),
    deployment$id,
    deployment$deployment_id
)
    "#;

    let title = format!("{title} | ricochet 🐇");
    view! {
        <Title text=title/>
        <div class=move || dark_mode_class()>
            <div class="flex-auto h-full w-full bg-zinc-100/50 antialiased dark:bg-zinc-900">
                <div class="h-full lg:ml-72 xl:ml-80">
                    // Header section
                    <header class="contents lg:pointer-events-none lg:fixed lg:inset-0 lg:z-40 lg:flex">
                        <div class="contents lg:pointer-events-auto lg:block lg:w-72 lg:overflow-y-auto lg:border-r lg:border-zinc-900/10 lg:px-6 lg:pb-8 lg:pt-4 lg:dark:border-white/10 xl:w-80">
                            <div class="hidden lg:flex">
                                <HomeButton/>
                            </div>
                            <Header mode=mode.into() set_mode=set_mode/>
                        // <Navigation class="hidden lg:mt-10 lg:block".into()/>
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
                                // <article
                                // inner_html=body
                                <div class="h-full w-full max-w-2xl lg:max-w-5xl mx-auto">
                                    <h2 class="mt-0! mb-4!">"Deploy an item"</h2>
                                    <p>{description}</p>
                                    <div class="inline-flex items-center p-2 border border-zinc-900/10 dark:border-white/10 m-0!">
                                        {method.as_badge()}
                                        <p class="ms-2 font-mono text-base my-0!">{endpoint}</p>
                                    </div>

                                    <div class="grid grid-cols-1 md:grid-cols-2 gap-8 mt-8 border-t border-zinc-900/5 pt-8 dark:border-white/5">
                                        {if let Some(body) = body_params {
                                            let v = body
                                                .into_iter()
                                                .map(|i| i.as_view())
                                                .collect_view()
                                                .into_any();
                                            view! {
                                                <div id="body-param-section" class="">
                                                    <div id="body-params" class="inline-flex items-center mb-4">
                                                        <h3 class="my-0!">"Body Parameters"</h3>
                                                        <ChainLink class="size-5 ms-2 dark:hover:text-white hover:cursor-pointer"
                                                            .to_string()/>
                                                    </div>
                                                    <div class="divide-y divide-zinc-900/10 dark:divide-white/10">
                                                        {v}
                                                    </div>
                                                </div>
                                            }
                                                .into_any()
                                        } else {
                                            view! {}.into_any()
                                        }}
                                        <div class="not-prose">
                                            <CodeTab/>
                                        </div>
                                    </div>
                                // class="w-full h-full max-w-3xl"
                                // ></article>
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
pub fn ApiRefPage(
    mode: ReadSignal<ColorMode>,
    set_mode: WriteSignal<ColorMode>,
    endpoint: ApiEndpoint,
) -> AnyView {
    view! { <ApiRefLayout set_mode=set_mode mode=mode endpoint=endpoint/> }.into_any()
}

#[component]
pub fn CodeTab() -> AnyView {
    let active_class ="text-violet-600 hover:text-violet-600 dark:text-violet-500 dark:hover:text-violet-500 border-violet-600 dark:border-violet-500";

    let curl_code = r#"curl -X
    POST \
    "https://ricochet.rs/api/v0/content/01JSZAXZ3TSTAYXP56ARDVFJCJ/invoke" \
    -H "Authorization: Key rico_AJFFXAaFVcw_LjrcKuB10gJ34cL9mS9mQu4oGjrafG56k"
    "#;
    view! {
        <div class="border border-zinc-900/10 dark:border-white/10 dark:bg-zinc-800/50 not-prose shadow-sm">
            <div class="mb-2 dark:bg-zinc-900">
                <ul
                    class="flex flex-wrap -mb-px text-sm font-medium text-center marker-none list-none px-4"
                    id="default-styled-tab"
                    data-tabs-toggle="#default-styled-tab-content"

                    role="tablist"
                >
                    <li class="me-2" role="presentation">
                        <button
                            class=format!(
                                "font-mono inline-block p-2 border-b-2 rounded-t-lg {active_class} cursor-pointer",
                            )

                            id="profile-styled-tab"
                            data-tabs-target="#styled-profile"
                            type="button"
                            role="tab"
                            aria-controls="profile"
                            aria-selected="false"
                        >
                            "cURL"
                        </button>
                    </li>
                    <li class="me-2" role="presentation">
                        <button
                            class="font-mono inline-block p-2 border-b-2 dark:border-zinc-500 dark:hover:border-white
                            text-zinc-500
                            dark:hover:text-white
                            hover:text-zinc-600 hover:border-zinc-300 cursor-pointer"

                            id="dashboard-styled-tab"
                            type="button"
                            role="tab"
                            aria-selected="false"
                        >
                            "R"
                        </button>
                    </li>
                </ul>
            </div>
            <div id="default-styled-tab-content not-prose">
                <div class="leading-[1.35rem] not-prose" role="tabpanel">
                    <pre class=format!("not-prose overflow-x-scroll px-2 {SCROLLBAR_X}")>
                        <code class="not-prose overflow-x-scroll text-xs">{curl_code}</code>
                    </pre>
                </div>
                <div
                    class="hidden p-4 rounded-lg bg-gray-50 dark:bg-gray-800"
                    id="styled-dashboard"
                    role="tabpanel"
                    aria-labelledby="dashboard-tab"
                >
                    <p class="text-sm text-gray-500 dark:text-gray-400">
                        This is some placeholder content the
                        <strong class="font-medium text-gray-800 dark:text-white">
                            "Dashboard tab's associated content"
                        </strong>
                        . Clicking another tab will toggle the visibility of this one for the next. The tab JavaScript swaps classes to control the content visibility and styling.
                    </p>
                </div>
            </div>
        </div>
    }.into_any()
}

#[component]
pub fn ChainLink(#[prop(optional)] class: Option<String>) -> AnyView {
    let class = class.unwrap_or("size-4".to_string());
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="32"
            height="32"
            fill="currentColor"
            viewBox="0 0 256 256"
            class=class
        >
            <path d="M240,88.23a54.43,54.43,0,0,1-16,37L189.25,160a54.27,54.27,0,0,1-38.63,16h-.05A54.63,54.63,0,0,1,96,119.84a8,8,0,0,1,16,.45A38.62,38.62,0,0,0,150.58,160h0a38.39,38.39,0,0,0,27.31-11.31l34.75-34.75a38.63,38.63,0,0,0-54.63-54.63l-11,11A8,8,0,0,1,135.7,59l11-11A54.65,54.65,0,0,1,224,48,54.86,54.86,0,0,1,240,88.23ZM109,185.66l-11,11A38.41,38.41,0,0,1,70.6,208h0a38.63,38.63,0,0,1-27.29-65.94L78,107.31A38.63,38.63,0,0,1,144,135.71a8,8,0,0,0,16,.45A54.86,54.86,0,0,0,144,96a54.65,54.65,0,0,0-77.27,0L32,130.75A54.62,54.62,0,0,0,70.56,224h0a54.28,54.28,0,0,0,38.64-16l11-11A8,8,0,0,0,109,185.66Z"></path>
        </svg>
    }.into_any()
}
