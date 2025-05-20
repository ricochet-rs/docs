use crate::{
    components::navigation::{ArrowDirection, Button, ButtonVariant},
    docs::{DocPage, DOC_PAGES},
};
use leptos::prelude::*;

#[derive(Debug, Clone)]
pub struct PageNav {
    title: String,
    href: String,
}

#[component]
pub fn PageNavigation(prev: Option<usize>, next: Option<usize>) -> AnyView {
    // FIXME figure out how to get previous and next pages
    // into this

    let prev_page = prev.map(|idx| {
        let DocPage { title, href, .. } = DOC_PAGES[idx];
        PageNav {
            title: title.into(),
            href: href.into(),
        }
    });

    let next_page = next.map(|idx| {
        let DocPage { title, href, .. } = DOC_PAGES[idx];
        PageNav {
            title: title.into(),
            href: href.into(),
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

#[component]
pub fn PageLink(label: &'static str, page: PageNav, arrow: ArrowDirection) -> AnyView {
    view! {
        <Button href=page.href.clone() variant=ButtonVariant::Secondary arrow=arrow>
            {label}
        </Button>
        <a
            href=page.href.clone()
            class="text-base font-semibold text-zinc-900 transition hover:text-zinc-600 dark:text-zinc-100 dark:hover:text-zinc-300"
        >
            {page.title}
        </a>
    }.into_any()
}

#[component]
pub fn GitHubIcon(#[prop(optional)] class: Option<String>) -> impl IntoView {
    view! {
        <svg viewBox="0 0 20 20" aria-hidden="true" class=class>
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M10 1.667c-4.605 0-8.334 3.823-8.334 8.544 0 3.78 2.385 6.974 5.698 8.106.417.075.573-.182.573-.406 0-.203-.011-.875-.011-1.592-2.093.397-2.635-.522-2.802-1.002-.094-.246-.5-1.005-.854-1.207-.291-.16-.708-.556-.01-.567.656-.01 1.124.62 1.281.876.75 1.292 1.948.93 2.427.705.073-.555.291-.93.531-1.143-1.854-.213-3.791-.95-3.791-4.218 0-.929.322-1.698.854-2.296-.083-.214-.375-1.09.083-2.265 0 0 .698-.224 2.292.876a7.576 7.576 0 0 1 2.083-.288c.709 0 1.417.096 2.084.288 1.593-1.11 2.291-.875 2.291-.875.459 1.174.167 2.05.084 2.263.53.599.854 1.357.854 2.297 0 3.278-1.948 4.005-3.802 4.219.302.266.563.78.563 1.58 0 1.143-.011 2.061-.011 2.35 0 .224.156.491.573.405a8.365 8.365 0 0 0 4.11-3.116 8.707 8.707 0 0 0 1.567-4.99c0-4.721-3.73-8.545-8.334-8.545Z"
            ></path>
        </svg>
    }
}

#[component]
pub fn DiscordIcon(#[prop(optional)] class: Option<String>) -> impl IntoView {
    view! {
        <svg viewBox="0 0 20 20" aria-hidden="true" class=class>
            <path d="M16.238 4.515a14.842 14.842 0 0 0-3.664-1.136.055.055 0 0 0-.059.027 10.35 10.35 0 0 0-.456.938 13.702 13.702 0 0 0-4.115 0 9.479 9.479 0 0 0-.464-.938.058.058 0 0 0-.058-.027c-1.266.218-2.497.6-3.664 1.136a.052.052 0 0 0-.024.02C1.4 8.023.76 11.424 1.074 14.782a.062.062 0 0 0 .024.042 14.923 14.923 0 0 0 4.494 2.272.058.058 0 0 0 .064-.02c.346-.473.654-.972.92-1.496a.057.057 0 0 0-.032-.08 9.83 9.83 0 0 1-1.404-.669.058.058 0 0 1-.029-.046.058.058 0 0 1 .023-.05c.094-.07.189-.144.279-.218a.056.056 0 0 1 .058-.008c2.946 1.345 6.135 1.345 9.046 0a.056.056 0 0 1 .059.007c.09.074.184.149.28.22a.058.058 0 0 1 .023.049.059.059 0 0 1-.028.046 9.224 9.224 0 0 1-1.405.669.058.058 0 0 0-.033.033.056.056 0 0 0 .002.047c.27.523.58 1.022.92 1.495a.056.056 0 0 0 .062.021 14.878 14.878 0 0 0 4.502-2.272.055.055 0 0 0 .016-.018.056.056 0 0 0 .008-.023c.375-3.883-.63-7.256-2.662-10.246a.046.046 0 0 0-.023-.021Zm-9.223 8.221c-.887 0-1.618-.814-1.618-1.814s.717-1.814 1.618-1.814c.908 0 1.632.821 1.618 1.814 0 1-.717 1.814-1.618 1.814Zm5.981 0c-.887 0-1.618-.814-1.618-1.814s.717-1.814 1.618-1.814c.908 0 1.632.821 1.618 1.814 0 1-.71 1.814-1.618 1.814Z"></path>
        </svg>
    }
}

#[component]
pub fn BlueSkyIcon(#[prop(optional)] class: Option<String>) -> AnyView {
    let class = class.unwrap_or("size-4".to_string());
    view! {
        <svg
            width="100%"
            height="100%"
            viewBox="0 0 600 530"
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            class=class
        >
            <path
                d="m135.72 44.03c66.496 49.921 138.02 151.14 164.28 205.46 26.262-54.316 97.782-155.54 164.28-205.46 47.98-36.021 125.72-63.892 125.72 24.795 0 17.712-10.155 148.79-16.111 170.07-20.703 73.984-96.144 92.854-163.25 81.433 117.3 19.964 147.14 86.092 82.697 152.22-122.39 125.59-175.91-31.511-189.63-71.766-2.514-7.3797-3.6904-10.832-3.7077-7.8964-0.0174-2.9357-1.1937 0.51669-3.7077 7.8964-13.714 40.255-67.233 197.36-189.63 71.766-64.444-66.128-34.605-132.26 82.697-152.22-67.108 11.421-142.55-7.4491-163.25-81.433-5.9562-21.282-16.111-152.36-16.111-170.07 0-88.687 77.742-60.816 125.72-24.795z"
                fill="currentColor"
            ></path>
        </svg>
    }.into_any()
}

#[component]
pub fn SocialLink(href: &'static str, icon: impl IntoView, children: Children) -> AnyView {
    view! {
        <a href=href class="group">
            <span class="sr-only">{children()}</span>
            <span class="h-5 w-5 fill-zinc-700 transition group-hover:fill-zinc-900 dark:group-hover:fill-zinc-500">
                {icon}
            </span>
        </a>
    }.into_any()
}

#[component]
pub fn SmallPrint() -> AnyView {
    let icon_class = String::from(
        "h-5 w-5 text-zinc-700 hover:text-zinc-900 dark:hover:text-zinc-500 fill-zinc-700 transition group-hover:fill-zinc-900 dark:group-hover:fill-zinc-500",
    );

    view! {
        <div class="flex flex-col items-center justify-between gap-5 border-t border-zinc-900/5 pt-8 dark:border-white/5 sm:flex-row">
            <p class="text-xs text-zinc-600 dark:text-zinc-400">
                "© Copyright 2025. All rights reserved."
            </p>
            <div class="flex gap-4">
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
        <div class="flex w-full gap-5 mx-auto">
            <p class="mx-auto items-center text-xs text-zinc-600 dark:text-zinc-400">
                "Made with 🤍 by "
                <a href="https://ricochet.rs" class="font-semibold dark:text-white">
                    "ricochet.rs"
                </a> " 🐇"
            </p>
        </div>
    }.into_any()
}

#[component]
pub fn Footer(next: Option<usize>, prev: Option<usize>) -> AnyView {
    view! {
        <footer class="mx-auto w-full max-w-2xl space-y-10 pb-16 lg:max-w-5xl">
            <PageNavigation prev=prev next=next/>
            <SmallPrint/>
        </footer>
    }
    .into_any()
}
