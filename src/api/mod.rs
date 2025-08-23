use crate::{
    HomeButton,
    api::parse::{read_api_ref, read_md},
    components::{
        footer::{PageLink, PageNav, SmallPrint},
        navigation::{ArrowDirection, Header, NavLink},
    },
};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_use::{ColorMode, UseClipboardReturn, use_clipboard};
use std::sync::OnceLock;

pub mod parse;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HttpMethod {
    Get,
    Post,
    Patch,
    Delete,
}

impl HttpMethod {
    pub fn as_badge(&self) -> AnyView {
        let badge_class = match self {
            HttpMethod::Get => {
                "font-mono text-sm font-medium  px-1.5 py-1 ring-1 ring-inset ring-emerald-300 dark:ring-emerald-400/30 bg-emerald-400/10 text-emerald-500 dark:text-emerald-400"
            }
            HttpMethod::Post => {
                "font-mono text-sm font-medium  px-1.5 py-1 ring-1 ring-inset ring-indigo-300 bg-indigo-400/10 text-indigo-500 dark:ring-indigo-400/30 dark:bg-indigo-400/10 dark:text-indigo-400"
            }
            HttpMethod::Patch => {
                "font-mono text-sm font-medium  px-1.5 py-1 ring-1 ring-inset ring-violet-300 bg-violet-400/10 text-violet-500 dark:ring-violet-400/30 dark:bg-violet-400/10 dark:text-violet-400"
            }
            HttpMethod::Delete => {
                "font-mono text-sm font-medium  px-1.5 py-1 ring-1 ring-inset ring-rose-200 bg-rose-50 text-red-500 dark:ring-rose-500/20 dark:bg-rose-400/10 dark:text-rose-400"
            }
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

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ParamInfo {
    pub name: String,
    pub desc: String,
}

impl ParamInfo {
    pub fn as_view(&self) -> AnyView {
        view! {
            <div class="space-y-0 mb-4">
                <p class="font-mono text-base font-semibold dark:text-white text-black mb-0!">
                    {self.name.clone()}
                </p>
                <p class="text-sm mt-2! mb-4" inner_html=self.desc.clone()></p>
            </div>
        }
        .into_any()
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Examples {
    r: String,
    curl: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ApiEndpoint {
    pub title: String,
    pub description: String,
    pub method: HttpMethod,
    pub path: String,
    pub response: String,
    pub path_params: Option<Vec<ParamInfo>>,
    pub body_params: Option<Vec<ParamInfo>>,
    pub examples: Examples,
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

#[component]
pub fn ApiRefLayout(
    mode: ReadSignal<ColorMode>,
    set_mode: WriteSignal<ColorMode>,
    endpoint: ApiEndpoint,
) -> impl IntoView {
    let ApiEndpoint {
        title,
        method,
        path,
        description,
        path_params,
        body_params,
        examples,
        response,
    } = endpoint;
    let location = leptos_router::hooks::use_location();
    let dark_mode_class = move || match mode.get() {
        ColorMode::Dark => "dark w-full",
        _ => "w-full",
    };

    let UseClipboardReturn { copied, copy, .. } = use_clipboard();
    let slg = location.pathname.get_untracked();
    let (prev_slug, next_slug) = find_prev_next(&slg);
    let clip_url = move |id: &'static str| {
        let loc = location.pathname.get();
        let url = leptos_router::hooks::use_url().get();
        let url = url.origin();
        format!("{url}{loc}#{id}")
    };

    let meta_title = format!("{title} | ricochet 🐇");

    view! {
        <Title text=meta_title/>
        <div class=dark_mode_class>
            <div class="flex-auto h-full w-full bg-zinc-100/50 antialiased dark:bg-zinc-900">
                <div class="h-full lg:ml-72 xl:ml-80">
                    // Header section
                    <header class="contents lg:pointer-events-none lg:fixed lg:inset-0 lg:z-40 lg:flex">
                        <div class="contents lg:pointer-events-auto lg:block lg:w-72 lg:overflow-y-auto lg:border-r lg:border-zinc-900/10 lg:px-6 lg:pb-8 lg:pt-4 lg:dark:border-white/10 xl:w-80">
                            <div class="hidden lg:flex">
                                <HomeButton/>
                            </div>
                            <Header mode=mode set_mode=set_mode/>
                            <ApiNavigation class="hidden lg:mt-10 lg:block"/>
                        </div>
                    </header>
                    // Main content area
                    <div class="relative flex h-full flex-col px-4 pt-14 sm:px-6 lg:px-8">

                        // this gets the location of the page we're on
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

                                    <div id="endpoint-header">
                                        <h2 class="mt-0! mb-4!">{title}</h2>
                                        <div class="inline-flex items-center p-2 border border-zinc-900/10 dark:border-white/10 m-0!">
                                            {method.as_badge()}
                                            <p class="ms-2 font-mono text-base my-0!">{path}</p>
                                        </div>
                                        <div class="flex-row gap-4">
                                            <div class="w-full" inner_html=description></div>
                                            <div id="response" class="mb-6">
                                                <h3 class="mt-0!">"Response"</h3>
                                                <div inner_html=response></div>
                                            </div>
                                            <div class="w-full overflow-x-auto">
                                                <CodeTab examples=examples.clone()/>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="mt-8 border-t border-zinc-900/5 pt-8 dark:border-white/5">
                                        <div id="params" class="">
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
                                                            <button on:click=move |_| {
                                                                let copy = copy.clone();
                                                                copy(&clip_url("body-params"));
                                                            }>
                                                                {move || {
                                                                    if copied.get() {
                                                                        view! {
                                                                            <CheckMark class="size-5 ms-2 dark:text-emerald-400 text-emerald-500"
                                                                                .to_string()/>
                                                                        }
                                                                            .into_any()
                                                                    } else {
                                                                        view! {
                                                                            <ChainLink class="size-5 ms-2 dark:text-zinc-400 dark:hover:text-white hover:cursor-pointer"
                                                                                .to_string()/>
                                                                        }
                                                                            .into_any()
                                                                    }
                                                                }}

                                                            </button>
                                                        </div>
                                                        <div class="divide-y divide-zinc-900/10 dark:divide-white/10">
                                                            {v}
                                                        </div>
                                                    </div>
                                                }
                                                    .into_any()
                                            } else {
                                                ().into_any()
                                            }}

                                            {if let Some(body) = path_params {
                                                let v = body
                                                    .into_iter()
                                                    .map(|i| i.as_view())
                                                    .collect_view()
                                                    .into_any();
                                                view! {
                                                    <div id="path-param-section" class="">
                                                        <div id="path-params" class="inline-flex items-center mb-4">
                                                            <h3 class="my-0!">"Path Parameters"</h3>
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
                                                ().into_any()
                                            }}

                                        </div>

                                    </div>
                                // class="w-full h-full max-w-3xl"
                                // ></article>
                                </div>
                            </div>
                        </main>
                        <ApiFooter prev=prev_slug next=next_slug/>
                    </div>
                </div>
            </div>
        </div>
    }
}

const OV: &str = include_str!("overview.md");
#[component]
pub fn ApiLandingPage(mode: ReadSignal<ColorMode>, set_mode: WriteSignal<ColorMode>) -> AnyView {
    let dark_mode_class = move || match mode.get() {
        ColorMode::Dark => "dark w-full",
        _ => "w-full",
    };

    let contents = OV.to_string();
    let overview = OnceResource::new(read_md(contents));

    view! {
        <Title text="Introduction"/>
        <div class=dark_mode_class>
            <div class="flex-auto h-full w-full bg-zinc-100/50 antialiased dark:bg-zinc-900">
                <div class="h-full lg:ml-72 xl:ml-80">
                    // Header section
                    <header class="contents lg:pointer-events-none lg:fixed lg:inset-0 lg:z-40 lg:flex">
                        <div class="contents lg:pointer-events-auto lg:block lg:w-72 lg:overflow-y-auto lg:border-r lg:border-zinc-900/10 lg:px-6 lg:pb-8 lg:pt-4 lg:dark:border-white/10 xl:w-80">
                            <div class="hidden lg:flex">
                                <HomeButton/>
                            </div>
                            <Header mode=mode set_mode=set_mode/>
                            <ApiNavigation class="hidden lg:mt-10 lg:block"/>
                        </div>
                    </header>
                    // Main content area
                    <div class="relative flex h-full flex-col px-4 pt-14 sm:px-6 lg:px-8">

                        // this gets the location of the page we're on
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

                                    <h2 class="mt-0! mb-4!">"Introduction"</h2>
                                    <Transition>
                                        {move || Suspend::new(async move {
                                            if let Some(Ok(ov)) = overview.get() {
                                                view! { <div inner_html=ov></div> }.into_any()
                                            } else {
                                                leptos::logging::log!("{:?}", overview.get());
                                                ().into_any()
                                            }
                                        })}

                                    </Transition>

                                </div>
                            </div>
                        </main>
                        <ApiFooter prev=None next=Some((1, 0))/>
                    </div>
                </div>
            </div>
        </div>
    }.into_any()
}

#[component]
pub fn ApiRefPage(mode: ReadSignal<ColorMode>, set_mode: WriteSignal<ColorMode>) -> AnyView {
    let location = leptos_router::hooks::use_location();

    let doc = Resource::new(
        move || location.pathname.get(),
        |pp| {
            let path = format!("./src/{}.toml", pp);
            read_api_ref(path)
        },
    );

    let dark_mode_class = move || match mode.get() {
        ColorMode::Dark => "dark w-full",
        _ => "w-full",
    };

    view! {
        <Transition>
            {move || Suspend::new(async move {
                if let Some(Ok(page)) = doc.get() {
                    view! { <ApiRefLayout mode=mode set_mode=set_mode endpoint=page/> }.into_any()
                } else {
                    view! {
                        <Title text="Not found!"/>
                        <div class=dark_mode_class>
                            <div class="flex-auto h-full w-full bg-zinc-100/50 antialiased dark:bg-zinc-900">
                                <div class="h-full lg:ml-72 xl:ml-80">
                                    // Header section
                                    <header class="contents lg:pointer-events-none lg:fixed lg:inset-0 lg:z-40 lg:flex">
                                        <div class="contents lg:pointer-events-auto lg:block lg:w-72 lg:overflow-y-auto lg:border-r lg:border-zinc-900/10 lg:px-6 lg:pb-8 lg:pt-4 lg:dark:border-white/10 xl:w-80">
                                            <div class="hidden lg:flex">
                                                <HomeButton/>
                                            </div>
                                            <Header mode=mode set_mode=set_mode/>
                                            <ApiNavigation class="hidden lg:mt-10 lg:block"/>
                                        </div>
                                    </header>
                                    <div class="not-prose mx-auto flex h-full max-w-xl flex-col items-center justify-center text-center">
                                        <p class="text-sm font-semibold text-zinc-900 dark:text-white">
                                            "404"
                                        </p>
                                        <h1 class="mt-2 text-2xl font-bold text-zinc-900 dark:text-white">
                                            "Page not found"
                                        </h1>
                                        <p class="mt-2 text-base text-zinc-600 dark:text-zinc-400">
                                            "These are not the docs you're looking for."
                                        </p>
                                        <a
                                            class="inline-flex gap-0.5 justify-center overflow-hidden text-sm font-medium transition bg-zinc-900 py-1 px-3 text-white hover:bg-zinc-700 dark:bg-emerald-400/10 dark:text-emerald-400 dark:ring-1 dark:ring-inset dark:ring-emerald-400/20 dark:hover:bg-emerald-400/10 dark:hover:text-emerald-300 dark:hover:ring-emerald-300 mt-8"
                                            href="/api/overview"
                                        >
                                            "Back to docs"
                                            <svg
                                                viewBox="0 0 20 20"
                                                fill="none"
                                                aria-hidden="true"
                                                class="mt-0.5 h-5 w-5 -mr-1"
                                            >
                                                <path
                                                    stroke="currentColor"
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    d="m11.5 6.5 3 3.5m0 0-3 3.5m3-3.5h-9"
                                                ></path>
                                            </svg>
                                        </a>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                        .into_any()
                }
            })}

        </Transition>
    }
    .into_any()
}

#[component]
pub fn CodeTab(examples: Examples) -> AnyView {
    let active_class = "text-violet-600 hover:text-violet-600 dark:text-violet-500 dark:hover:text-violet-500 border-violet-600 dark:border-violet-500 font-medium text-base";
    let inactive_class = "font-mono inline-block p-2 border-b-2 dark:border-zinc-400 dark:hover:border-white text-zinc-500 dark:text-zinc-400 dark:hover:text-white hover:text-zinc-900 hover:border-zinc-900 dark:hover:border-zinc-300 cursor-pointer text-base";

    let Examples { r, curl } = examples;

    #[derive(Clone, Copy)]
    enum CodeTab {
        Curl,
        R,
    }

    let code_tab = RwSignal::new(CodeTab::R);

    view! {
        <div class="not-prose">
            <h4 id="code-example" class="mb-2">
                "Example"
            </h4>
            <div class="border border-zinc-900/10 dark:border-white/10 dark:bg-zinc-800/50 not-prose shadow-sm">
                <div class="mb-2 bg-zinc-100 dark:bg-zinc-900 border-b border-zinc-900/10 dark:border-white/10">
                    <ul
                        class="flex flex-wrap -mb-px text-sm font-medium text-center marker-none list-none px-4"
                        id="default-styled-tab"
                        data-tabs-toggle="#default-styled-tab-content"

                        role="tablist"
                    >
                        <li class="me-2" role="presentation">
                            <button
                                class=move || {
                                    if let CodeTab::R = code_tab.get() {
                                        format!(
                                            "font-mono inline-block p-2 border-b-2 rounded-t-lg {active_class} cursor-pointer",
                                        )
                                    } else {
                                        inactive_class.to_string()
                                    }
                                }

                                type="button"
                                role="tab"
                                on:click=move |_| {
                                    code_tab.set(CodeTab::R);
                                }

                                aria-selected=move || {
                                    matches!(code_tab.get(), CodeTab::R)
                                }
                            >

                                "R"
                            </button>
                        </li>
                        <li class="me-2" role="presentation">
                            <button
                                class=move || {
                                    if let CodeTab::Curl = code_tab.get() {
                                        format!(
                                            "font-mono inline-block p-2 border-b-2 {active_class} cursor-pointer",
                                        )
                                    } else {
                                        inactive_class.to_string()
                                    }
                                }

                                id="curl-example-code"
                                type="button"
                                role="tab"
                                aria-selected=move || {
                                    matches!(code_tab.get(), CodeTab::Curl)
                                }

                                on:click=move |_| {
                                    code_tab.set(CodeTab::Curl);
                                }
                            >

                                "cURL"
                            </button>
                        </li>

                    </ul>
                </div>
                <div class="not-prose">
                    <div class="leading-[1.35rem] not-prose" role="tabpanel">
                        {move || {
                            match code_tab.get() {
                                CodeTab::R => {
                                    view! {
                                        <pre class=format!(
                                            "not-prose overflow-x-scroll px-4 {SCROLLBAR_X} max-h-[500px] {SCROLLBAR_Y}",
                                        )>
                                            <code
                                                class="not-prose overflow-x-scroll text-base/7"
                                                inner_html=r.clone()
                                            ></code>
                                        </pre>
                                    }
                                        .into_any()
                                }
                                CodeTab::Curl => {
                                    view! {
                                        <pre class=format!(
                                            "not-prose overflow-x-scroll px-4 {SCROLLBAR_X} max-h-[500px] {SCROLLBAR_Y}",
                                        )>
                                            <code
                                                class="not-prose overflow-x-scroll text-base/7"
                                                inner_html=curl.clone()
                                            ></code>
                                        </pre>
                                    }
                                        .into_any()
                                }
                            }
                        }}

                    </div>

                </div>
            </div>
        </div>
    }.into_any()
}

#[component]
pub fn CheckMark(#[prop(optional)] class: Option<String>) -> AnyView {
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
            <path d="M229.66,77.66l-128,128a8,8,0,0,1-11.32,0l-56-56a8,8,0,0,1,11.32-11.32L96,188.69,218.34,66.34a8,8,0,0,1,11.32,11.32Z"></path>
        </svg>
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

#[derive(Clone)]
pub struct ApiRefNavLink {
    pub title: String,
    pub slug: String,
}

#[derive(Copy, Clone)]
pub enum ApiRefSection {
    Overview,
    Content,
    Task,
    Service,
    User,
}

#[derive(Clone)]
pub struct ApiRefGroup {
    pub section: ApiRefSection,
    pub links: Vec<ApiRefNavLink>,
}

impl std::fmt::Display for ApiRefSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ApiRefSection::Overview => "Overview",
            ApiRefSection::Content => "Content",
            ApiRefSection::Task => "Tasks",
            ApiRefSection::Service => "Services",
            ApiRefSection::User => "User",
        };
        write!(f, "{}", s)
    }
}

#[component]
pub fn ApiNavGroup(
    group: ApiRefSection,
    links: Vec<ApiRefNavLink>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let location = leptos_router::hooks::use_location();
    let title = group.to_string();

    view! {
        <li class=format!("relative mt-6 {}", class.unwrap_or(""))>
            <h2 class="text-sm font-mono font-semibold text-zinc-900 dark:text-zinc-100">
                {title.to_string()}
            </h2>

            <div class="relative mt-3 pl-2">

                <div class="absolute inset-y-0 left-2 w-px bg-zinc-900/10 dark:bg-white/5"></div>

                // FIXME this is where we add nested navigation
                <ul role="list" class="border-l border-transparent">
                    {move || {
                        links
                            .iter()
                            .map(|di| {
                                let href = format!("/api/{}", di.slug);
                                let is_active = location.pathname.get().eq(&href);
                                let title = di.title.to_string();
                                view! {
                                    {move || {
                                        if is_active {
                                            view! {
                                                <div class="absolute left-2 h-6 w-px bg-violet-500"></div>
                                            }
                                                .into_any()
                                        } else {
                                            ().into_any()
                                        }
                                    }}

                                    <li class="relative">
                                        <NavLink href=href active=is_active>
                                            {title}
                                        </NavLink>
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}

                </ul>
            </div>
        </li>
    }
}

pub static API_REF_PAGES: OnceLock<[ApiRefGroup; 5]> = OnceLock::new();

pub fn api_ref_navs() -> &'static [ApiRefGroup; 5] {
    (API_REF_PAGES.get_or_init(|| {
        [
            ApiRefGroup {
                section: ApiRefSection::Overview,
                links: vec![ApiRefNavLink {
                    title: String::from("Introduction"),
                    slug: "".to_string(),
                }],
            },
            ApiRefGroup {
                section: ApiRefSection::Content,
                links: vec![
                    ApiRefNavLink {
                        title: "Deploy an item".to_string(),
                        slug: "post-deploy".to_string(),
                    },
                    ApiRefNavLink {
                        title: "List item deployments".to_string(),
                        slug: "get-deployments".to_string(),
                    },
                    ApiRefNavLink {
                        title: "Get current settings".to_string(),
                        slug: "get-toml".to_string(),
                    },
                    ApiRefNavLink {
                        title: "Update settings".to_string(),
                        slug: "patch-settings".to_string(),
                    },
                    // ApiRefNavLink {
                    //     title: "Delete an item".to_string(),
                    //     slug: "delete".to_string(),
                    // },
                ],
            },
            ApiRefGroup {
                section: ApiRefSection::Task,
                links: vec![
                    ApiRefNavLink {
                        title: "Schedule a task".to_string(),
                        slug: "patch-schedule".to_string(),
                    },
                    ApiRefNavLink {
                        title: "Invoke a task".to_string(),
                        slug: "post-invoke".to_string(),
                    },
                    ApiRefNavLink {
                        title: "List active invocations".to_string(),
                        slug: "get-invocations".to_string(),
                    },
                    ApiRefNavLink {
                        title: "Stop an invocation".to_string(),
                        slug: "post-stop-invocation".to_string(),
                    },
                ],
            },
            ApiRefGroup {
                section: ApiRefSection::Service,
                links: vec![
                    ApiRefNavLink {
                        title: "List active instances".to_string(),
                        slug: "get-instances".to_string(),
                    },
                    ApiRefNavLink {
                        title: "Stop an instance".to_string(),
                        slug: "post-stop-instance".to_string(),
                    },
                ],
            },
            ApiRefGroup {
                section: ApiRefSection::User,
                links: vec![ApiRefNavLink {
                    title: "List user items".to_string(),
                    slug: "user-items".to_string(),
                }],
            },
        ]
    })) as _
}

type PageNavigation = (Option<(usize, usize)>, Option<(usize, usize)>);

fn find_prev_next(path: &str) -> PageNavigation {
    let slug = path.trim_start_matches("/api/");

    let navs = api_ref_navs();

    // Flattened vector of (group_index, link_index, slug)
    let mut flat: Vec<(usize, usize, &str)> = vec![];

    for (gi, group) in navs.iter().enumerate() {
        for (li, link) in group.links.iter().enumerate() {
            flat.push((gi, li, link.slug.as_str()));
        }
    }

    // Find current index in flat list
    let pos = flat.iter().position(|&(_, _, s)| s == slug);

    match pos {
        Some(i) => {
            let prev = if i > 0 {
                Some((flat[i - 1].0, flat[i - 1].1))
            } else {
                None
            };

            let next = flat.get(i + 1).map(|&(g, l, _)| (g, l));

            (prev, next)
        }
        None => (None, None),
    }
}

#[component]
pub fn ApiNavigation(#[prop(optional)] class: Option<&'static str>) -> impl IntoView {
    view! {
        <nav class=class>
            <ul role="list">
                // Map through navigation groups
                {api_ref_navs()
                    .iter()
                    .enumerate()
                    .map(|(i, group)| {
                        view! {
                            <ApiNavGroup
                                group=group.section
                                links=group.links.clone()
                                class=if i == 0 { "md:mt-0" } else { "" }
                            />
                        }
                    })
                    .collect::<Vec<_>>()}
                <li class="sticky bottom-0 z-10 mt-6 min-[416px]:hidden"></li>
            </ul>
        </nav>
    }
}

#[component]
pub fn ApiFooter(next: Option<(usize, usize)>, prev: Option<(usize, usize)>) -> AnyView {
    view! {
        <footer class="mx-auto w-full max-w-2xl space-y-10 pb-16 lg:max-w-5xl">
            <PageNavigation prev=prev next=next/>
            <SmallPrint/>
        </footer>
    }
    .into_any()
}

#[component]
pub fn PageNavigation(prev: Option<(usize, usize)>, next: Option<(usize, usize)>) -> AnyView {
    // FIXME figure out how to get previous and next pages
    // into this
    let navs = api_ref_navs();
    let prev_page = prev.map(|(i, j)| {
        let ApiRefNavLink { title, slug } = navs[i].links[j].clone();
        PageNav {
            title,
            href: format!("/api/{slug}"),
        }
    });

    let next_page = next.map(|(i, j)| {
        let ApiRefNavLink { title, slug } = navs[i].links[j].clone();
        PageNav {
            title,
            href: format!("/api/{slug}"),
        }
    });

    view! {
        <div class="flex">
            <div class="flex flex-col items-start gap-3">

                {match prev_page {
                    Some(pp) => {
                        view! { <PageLink label="Previous" page=pp arrow=ArrowDirection::Left/> }
                            .into_any()
                    }
                    None => ().into_any(),
                }}

            </div>
            <div class="ml-auto flex flex-col items-end gap-3">

                {match next_page {
                    Some(np) => {
                        view! { <PageLink label="Next" page=np arrow=ArrowDirection::Right/> }
                            .into_any()
                    }
                    None => ().into_any(),
                }}

            </div>
        </div>
    }
    .into_any()
}
