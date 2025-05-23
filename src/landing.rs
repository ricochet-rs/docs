use crate::{
    components::{
        footer::{BlueSkyIcon, DiscordIcon, GitHubIcon, SmallPrint, SocialLink},
        hero_pattern::HeroPattern,
        navigation::{ModeToggle, TopLevelNavItem},
    },
    HomeButton,
};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_use::ColorMode;

#[server]
async fn insert_email(email: String, interview: bool) -> Result<(), ServerFnError> {
    use reqwest::Client;
    #[derive(serde::Serialize, Debug)]
    struct EntryBody {
        email: String,
        interview: bool,
    }

    let supabase_url = "https://zmgzxezvboqmvioqtrze.supabase.co/rest/v1/ricochet-waitlist";
    // FIXME lol
    let supabase_key = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InptZ3p4ZXp2Ym9xbXZpb3F0cnplIiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTc0NzYwODU5OCwiZXhwIjoyMDYzMTg0NTk4fQ.xaFozu3Y4o_ZV3YG2qpEPOPUX335XBKN724488CtrUw";

    let client = Client::new();

    let body = EntryBody { email, interview };

    // leptos::logging::log!("Body is {body:?}");
    let req = client
        .post(supabase_url)
        .json(&body)
        .header("apikey", supabase_key)
        .header("Authorization", format!("Bearer {}", supabase_key).as_str())
        .header("Prefer", "return=minimal")
        .build()
        .unwrap();

    // leptos::logging::log!("Request is {req:?}");
    let _ = &client.execute(req).await?;

    Ok(())
}

#[component]
pub fn LandingPage(mode: ReadSignal<ColorMode>, set_mode: WriteSignal<ColorMode>) -> AnyView {
    let dark_mode_class = move || match mode.get() {
        ColorMode::Dark => "dark w-full",
        _ => "w-full",
    };

    let email = RwSignal::new(String::new());
    let contact = RwSignal::new(false);
    let submitted = RwSignal::new(false);

    let icon_class = String::from(
        "h-5 w-5 text-zinc-700 hover:text-zinc-900 dark:hover:text-zinc-500 fill-zinc-700 transition group-hover:fill-zinc-900 dark:group-hover:fill-zinc-500",
    );


    view! {
        <Title text="ricochet 🐇"/>
        <div class=move || dark_mode_class()>
            <div class="flex-auto h-full w-full bg-zinc-100/50 antialiased dark:bg-zinc-900">
                <div class="h-full">
                    // Header section
                    <header class="lg:z-40 lg:flex">
                        <div class="fixed inset-x-0 top-0 z-50 flex h-14 items-center justify-between gap-12 px-4 transition sm:px-6 lg:z-30 lg:px-8 backdrop-blur-sm">
                            // divider
                            <div class="absolute inset-x-0 top-full h-px transition bg-zinc-900/7.5 dark:bg-white/7.5"></div>
                            <HomeButton/>

                            <div class="flex items-center gap-5">
                                // Main navigation
                                <nav>
                                    <ul role="list" class="flex items-center gap-8">
                                        <TopLevelNavItem href="/hello"
                                            .to_string()>"Documentation"</TopLevelNavItem>
                                    </ul>
                                </nav>

                                // Divider
                                <div class="hidden md:block md:h-5 md:w-px md:bg-zinc-900/10 md:dark:bg-white/15"></div>
                                <ModeToggle mode=mode.into() set_mode=set_mode/>

                            </div>
                        </div>
                    </header>

                    // Main content area
                    <div class="relative flex h-full flex-col  pt-14 z-10">
                        <main class="flex-auto py-16">
                            <div class="mx-auto w-full">
                                <div class="h-full w-full max-w-3xl dark:text-white lg:max-w-4xl mx-auto text-center relative space-y-12">
                                    <h1 class="max-w-3xl mx-auto text-title text-balance text-4xl font-bold sm:text-6xl text-zinc-800 dark:text-zinc-100">
                                        "Data Scientists are "
                                        <span class="dark:text-violet-500 text-violet-600">
                                            "Developers"
                                        </span>
                                    </h1>

                                    <p class="max-w-2xl mx-auto text-balance text-lg dark:text-zinc-300 text-zinc-500">
                                        "Elastic scaling of Shiny, Plumber, and Ambiorix, a serverless R runtime, scheduled tasks—they're all a line of code away."
                                    </p>
                                </div>
                            </div>
                            <div class="relative mx-auto max-w-3xl lg:max-w-5xl block lg:flex items-center justify-between w-full space-x-14 space-y-10 py-42 px-8 lg:px-0">

                                <div class="w-full space-y-6">
                                    <h2 class="dark:text-white text-4xl font-bold text-balance">
                                        "Help shape our roadmap 📍"
                                    </h2>
                                    <p class="text-balance text-lg dark:text-zinc-300 text-zinc-500">
                                        "We're looking for small teams to test early builds, run ricochet locally, and tell us what actually matters. If you're ready to dive in and give real feedback, check the box."
                                    </p>
                                </div>
                                <div class="w-full space-y-4 border border-zinc-900/7.5 p-6 shadow-md dark:border-zinc-100/10 dark:shadow-zinc-800/10">
                                    <form
                                        class="flex dark:text-white"
                                        on:submit=move |ev| {
                                            ev.prevent_default();
                                            let e = email.get();
                                            let c = contact.get();
                                            if !submitted.get() {
                                                leptos::task::spawn_local(async move {
                                                    leptos::logging::log!("Email is {e} and contact is {c}");
                                                    let _ = insert_email(e, c).await;
                                                });
                                                submitted.set(true);
                                            }
                                        }
                                    >

                                        <div class="w-full space-y-4 text-left">
                                            <div>
                                                <p class="text-center font-bold text-lg mb-2">
                                                    "Be the first to deploy 🚀"
                                                </p>
                                                <label for="email" class="font-mono text-sm font-semibold">
                                                    "Email"
                                                </label>
                                            </div>
                                            <input
                                                type="email"
                                                id="email"
                                                name="email"
                                                bind:value=email
                                                required=""
                                                aria-required="true"
                                                class="flex h-10 w-full border border-zinc-900/7.5 bg-zinc-100 py-2 px-2 text-sm focus-visible:ring-2 focus-visible:ring-black focus-visible:ring-offset-1 focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 dark:border-zinc-100/10 dark:bg-zinc-700/20"
                                                placeholder="you@domain.com"
                                            />
                                            <div class="flex items-center mb-4">
                                                <input
                                                    id="contact"
                                                    type="checkbox"
                                                    bind:value=contact
                                                    class="w-4 h-4  border-zinc-300 dark:border-zinc-600"
                                                />
                                                <label
                                                    for="contact"
                                                    class="ms-2 text-sm font-medium text-zinc-900 dark:text-zinc-300"
                                                >
                                                    "Want to help shape our roadmap?"
                                                </label>
                                            </div>

                                            {move || {
                                                match submitted.get() {
                                                    true => {
                                                        view! {
                                                            <button
                                                                disabled=true
                                                                class="w-full cursor-pointer py-2 mt-2 text-sm font-semibold ring dark:bg-emerald-500/10 dark:text-emerald-300 dark:ring-emerald-300 dark:hover:bg-emerald-500 dark:hover:text-white dark:hover:ring-emerald-400
                                                                bg-emerald-500 text-white hover:bg-zinc-700
                                                                "
                                                            >
                                                                "Talk to you soon!"
                                                            </button>
                                                        }
                                                    }
                                                    false => {
                                                        view! {
                                                            <button
                                                                disabled=false
                                                                class="w-full cursor-pointer py-2 mt-2 text-sm font-semibold ring dark:bg-violet-500/10 dark:text-violet-300 dark:ring-violet-300 dark:hover:bg-violet-500 dark:hover:text-white dark:hover:ring-violet-400
                                                                bg-black text-white hover:bg-zinc-700
                                                                "
                                                            >
                                                                "Join the waitlist"
                                                            </button>
                                                        }
                                                    }
                                                }
                                            }}

                                            <p class="text-xs dark:text-zinc-300 text-zinc-500">
                                                "Or message us directly at "
                                                <a
                                                    class="dark:hover:text-white hover:text-black font-bold underline-1 dark:decoration-violet-600 decoration-violet-500 underline-offset-1 underline"
                                                    href="mailto:contact@ricochet.rs"
                                                >
                                                    "contact@ricochet.rs"
                                                </a> "."
                                            </p>
                                        </div>
                                    </form>
                                </div>
                            </div>
                            <div class="relative mx-auto max-w-3xl lg:max-w-5xl justify-between w-full space-x-8 pb-42 px-8 lg:px-0">
                                <div class="relative z-10 mx-auto max-w-xl space-y-6 text-center md:space-y-12 dark:text-white mb-18">
                                    <h2 class="dark:text-white text-balance text-4xl font-medium lg:text-5xl">
                                        "Built for data scientists deploying real code"
                                    </h2>
                                    <p class="text-body dark:text-zinc-300 text-zinc-500">
                                        "Built from the ground up in Rust for speed and efficiency—deploy and scale R and Julia code, schedule tasks, and run long-lived services with confidence."
                                    </p>
                                </div>
                                <div class="w-full grid grid-col-2 lg:grid-cols-3 divide-x divide-y divide-zinc-900/7.5 dark:divide-zinc-100/10 border border-zinc-900/7.5 dark:border-zinc-100/10">
                                    <FeatureCard
                                        title=String::from("Deploy Anything")
                                        desc=String::from(
                                            "Shiny, Ambiorix, Plumber, or any long-running service—if it speaks HTTP, it runs on ricochet. No artificial limits. No container boilerplate.",
                                        )

                                        icon=view! { <RocketIcon/> }.into_any()
                                    />

                                    <FeatureCard
                                        title="Security".to_string()
                                        desc="Bring your own identity provider with OpenID Connect. Control access to every deployed app, API, and task with fine-grained permissions."
                                            .to_string()
                                        icon=view! { <ShieldIcon/> }.into_any()
                                    />
                                    <FeatureCard
                                        title="Serverless R Functions".to_string()
                                        desc="Deploy R functions as RESTful services—instantly callable, autoscaling, and production-ready. The first serverless runtime built for R."
                                            .to_string()
                                        icon=view! { <CloudCheck/> }.into_any()
                                    />

                                    <FeatureCard
                                        title="Elastic Scaling".to_string()
                                        desc="Deploy once and scale automatically based on demand."
                                            .to_string()
                                        icon=view! { <LightningIcon/> }.into_any()
                                    />
                                    <FeatureCard
                                        title="Scheduled Tasks".to_string()
                                        desc="Deploy R and Julia scripts or Quarto documents as tasks. Run them on a schedule, or invoke them on demand via our REST API or client SDKs."
                                            .to_string()
                                        icon=view! { <ClockUserIcon/> }.into_any()
                                    />
                                    <FeatureCard
                                        title="Persistent Storage".to_string()
                                        desc="
                                        Each app, API, or task has access to persistent storage. Data written there is preserved across versions—no extra setup required."
                                            .to_string()
                                        icon=view! { <HardDrivesIcon/> }.into_any()
                                    />
                                </div>
                            </div>
                            <div class="relative mx-auto max-w-3xl lg:max-w-5xl justify-between w-full space-x-8 pb-20 px-8 lg:px-0">
                                <div class="relative z-10 mx-auto max-w-xl space-y-6 text-center md:space-y-6 dark:text-white mb-18">
                                    <h2 class="dark:text-white text-balance text-4xl font-medium lg:text-5xl mb-6">
                                        "Join our community"
                                    </h2>
                                    <p class="text-body dark:text-zinc-300 text-zinc-500">
                                        "Chat with us on "
                                        <a
                                            class="dark:hover:text-white hover:text-black font-bold underline-1 dark:decoration-violet-600 decoration-violet-500 underline-offset-1 underline"
                                            href="https://discord.gg/hgdXEm8xuT"
                                        >
                                            "Discord"
                                        </a> " to share ideas, ask questions, or follow updates."
                                    </p>

                                    <p class="text-body dark:text-zinc-300 text-zinc-500">
                                        "Want to help shape or build the product? Email us at "
                                        <a
                                            class="dark:hover:text-white hover:text-black font-bold underline-1 dark:decoration-violet-600 decoration-violet-500 underline-offset-1 underline"
                                            href="mailto:contact@ricochet.rs"
                                        >
                                            "contact@ricochet.rs"
                                        </a> ", we'd love to hear from you."
                                    </p>
                                    <div class="flex justify-center gap-4 ">
                                        <SocialLink
                                            href="https://github.com/ricochet-rs/"
                                            icon=view! { <GitHubIcon class=icon_class.clone()/> }
                                        >
                                            "Follow us on GitHub"
                                        </SocialLink>
                                        <SocialLink
                                            href="https://discord.gg/hgdXEm8xuT"
                                            icon=view! { <DiscordIcon class=icon_class.clone()/> }
                                        >
                                            "Join our Discord server"
                                        </SocialLink>
                                        <SocialLink
                                            href="https://bsky.app/profile/ricochet.rs"
                                            icon=view! { <BlueSkyIcon class=icon_class/> }
                                        >
                                            "Follow us on Bluesky"
                                        </SocialLink>
                                    </div>
                                </div>
                            </div>
                        </main>
                        <footer class="mx-auto w-full max-w-2xl space-y-10 pb-16 lg:max-w-5xl">
                            <SmallPrint/>
                        </footer>
                    </div>
                    <HeroPattern/>
                </div>
            </div>
        </div>
    }.into_any()
}

#[component]
fn FeatureCard(title: String, desc: String, #[prop(optional)] icon: Option<AnyView>) -> AnyView {
    view! {
        <div class="w-full space-y-2 p-8 lg:p-10">
            // Add SVG icon here
            // The title
            <div class="flex dark:text-white items-center">
                <span class="me-2">{icon}</span>
                <p class="font-bold">{title}</p>
            </div>
            // The section description
            <p class="dark:text-zinc-300 text-zinc-500 text-sm">{desc}</p>
        </div>
    }
    .into_any()
}


#[component]
fn LightningIcon(#[prop(optional)] class: Option<String>) -> AnyView {
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
            <path d="M215.79,118.17a8,8,0,0,0-5-5.66L153.18,90.9l14.66-73.33a8,8,0,0,0-13.69-7l-112,120a8,8,0,0,0,3,13l57.63,21.61L88.16,238.43a8,8,0,0,0,13.69,7l112-120A8,8,0,0,0,215.79,118.17ZM109.37,214l10.47-52.38a8,8,0,0,0-5-9.06L62,132.71l84.62-90.66L136.16,94.43a8,8,0,0,0,5,9.06l52.8,19.8Z"></path>
        </svg>
    }.into_any()
}

#[component]
fn HardDrivesIcon(#[prop(optional)] class: Option<String>) -> AnyView {
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
            <path d="M208,136H48a16,16,0,0,0-16,16v48a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V152A16,16,0,0,0,208,136Zm0,64H48V152H208v48Zm0-160H48A16,16,0,0,0,32,56v48a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V56A16,16,0,0,0,208,40Zm0,64H48V56H208v48ZM192,80a12,12,0,1,1-12-12A12,12,0,0,1,192,80Zm0,96a12,12,0,1,1-12-12A12,12,0,0,1,192,176Z"></path>
        </svg>
    }.into_any()
}

#[component]
fn ShieldIcon(#[prop(optional)] class: Option<String>) -> AnyView {
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
            <path d="M72,128a134.63,134.63,0,0,1-14.16,60.47,8,8,0,1,1-14.32-7.12A118.8,118.8,0,0,0,56,128,71.73,71.73,0,0,1,83,71.8,8,8,0,1,1,93,84.29,55.76,55.76,0,0,0,72,128Zm56-8a8,8,0,0,0-8,8,184.12,184.12,0,0,1-23,89.1,8,8,0,0,0,14,7.76A200.19,200.19,0,0,0,136,128,8,8,0,0,0,128,120Zm0-32a40,40,0,0,0-40,40,8,8,0,0,0,16,0,24,24,0,0,1,48,0,214.09,214.09,0,0,1-20.51,92A8,8,0,1,0,146,226.83,230,230,0,0,0,168,128,40,40,0,0,0,128,88Zm0-64A104.11,104.11,0,0,0,24,128a87.76,87.76,0,0,1-5,29.33,8,8,0,0,0,15.09,5.33A103.9,103.9,0,0,0,40,128a88,88,0,0,1,176,0,282.24,282.24,0,0,1-5.29,54.45,8,8,0,0,0,6.3,9.4,8.22,8.22,0,0,0,1.55.15,8,8,0,0,0,7.84-6.45A298.37,298.37,0,0,0,232,128,104.12,104.12,0,0,0,128,24ZM94.4,152.17A8,8,0,0,0,85,158.42a151,151,0,0,1-17.21,45.44,8,8,0,0,0,13.86,8,166.67,166.67,0,0,0,19-50.25A8,8,0,0,0,94.4,152.17ZM128,56a72.85,72.85,0,0,0-9,.56,8,8,0,0,0,2,15.87A56.08,56.08,0,0,1,184,128a252.12,252.12,0,0,1-1.92,31A8,8,0,0,0,189,168a8.39,8.39,0,0,0,1,.06,8,8,0,0,0,7.92-7,266.48,266.48,0,0,0,2-33A72.08,72.08,0,0,0,128,56Zm57.93,128.25a8,8,0,0,0-9.75,5.75c-1.46,5.69-3.15,11.4-5,17a8,8,0,0,0,5,10.13,7.88,7.88,0,0,0,2.55.42,8,8,0,0,0,7.58-5.46c2-5.92,3.79-12,5.35-18.05A8,8,0,0,0,185.94,184.26Z"></path>
        </svg>
    }.into_any()
}

#[component]
fn CloudCheck(#[prop(optional)] class: Option<String>) -> AnyView {
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
            <path d="M160,40A88.09,88.09,0,0,0,81.29,88.67,64,64,0,1,0,72,216h88a88,88,0,0,0,0-176Zm0,160H72a48,48,0,0,1,0-96c1.1,0,2.2,0,3.29.11A88,88,0,0,0,72,128a8,8,0,0,0,16,0,72,72,0,1,1,72,72Zm37.66-93.66a8,8,0,0,1,0,11.32l-48,48a8,8,0,0,1-11.32,0l-24-24a8,8,0,0,1,11.32-11.32L144,148.69l42.34-42.35A8,8,0,0,1,197.66,106.34Z"></path>
        </svg>
    }.into_any()
}

#[component]
fn ClockUserIcon(#[prop(optional)] class: Option<String>) -> AnyView {
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
            <path d="M136,72v43.05l36.42-18.21a8,8,0,0,1,7.16,14.31l-48,24A8,8,0,0,1,120,128V72a8,8,0,0,1,16,0Zm-8,144a88,88,0,1,1,88-88,8,8,0,0,0,16,0A104,104,0,1,0,128,232a8,8,0,0,0,0-16Zm103.73,5.94a8,8,0,1,1-15.46,4.11C213.44,215.42,203.46,208,192,208s-21.44,7.42-24.27,18.05A8,8,0,0,1,160,232a8.15,8.15,0,0,1-2.06-.27,8,8,0,0,1-5.67-9.79,40,40,0,0,1,17.11-23.32,32,32,0,1,1,45.23,0A40,40,0,0,1,231.73,221.94ZM176,176a16,16,0,1,0,16-16A16,16,0,0,0,176,176Z"></path>
        </svg>
    }.into_any()
}

#[component]
fn RocketIcon(#[prop(optional)] class: Option<String>) -> AnyView {
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
            <path d="M223.85,47.12a16,16,0,0,0-15-15c-12.58-.75-44.73.4-71.41,27.07L132.69,64H74.36A15.91,15.91,0,0,0,63,68.68L28.7,103a16,16,0,0,0,9.07,27.16l38.47,5.37,44.21,44.21,5.37,38.49a15.94,15.94,0,0,0,10.78,12.92,16.11,16.11,0,0,0,5.1.83A15.91,15.91,0,0,0,153,227.3L187.32,193A15.91,15.91,0,0,0,192,181.64V123.31l4.77-4.77C223.45,91.86,224.6,59.71,223.85,47.12ZM74.36,80h42.33L77.16,119.52,40,114.34Zm74.41-9.45a76.65,76.65,0,0,1,59.11-22.47,76.46,76.46,0,0,1-22.42,59.16L128,164.68,91.32,128ZM176,181.64,141.67,216l-5.19-37.17L176,139.31Zm-74.16,9.5C97.34,201,82.29,224,40,224a8,8,0,0,1-8-8c0-42.29,23-57.34,32.86-61.85a8,8,0,0,1,6.64,14.56c-6.43,2.93-20.62,12.36-23.12,38.91,26.55-2.5,36-16.69,38.91-23.12a8,8,0,1,1,14.56,6.64Z"></path>
        </svg>
    }.into_any()
}
