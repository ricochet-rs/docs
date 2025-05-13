use super::search::{MobileSearch, SearchButton, SearchDialog};
use crate::{
    docs::{doc_sections, DocPage, DocSection},
    HomeButton,
};
use leptos::{
    ev::keydown,
    html::{Div, Input},
    logging,
    prelude::*,
};
use leptos_use::{on_click_outside, use_document, use_event_listener, ColorMode};

#[component]
pub fn SunIcon(#[prop(optional)] class: Option<String>) -> AnyView {
    view! {
        <svg viewBox="0 0 20 20" fill="none" aria-hidden="true" class=class>
            <path d="M12.5 10a2.5 2.5 0 1 1-5 0 2.5 2.5 0 0 1 5 0Z"></path>
            <path
                strokeLinecap="round"
                d="M10 5.5v-1M13.182 6.818l.707-.707M14.5 10h1M13.182 13.182l.707.707M10 15.5v-1M6.11 13.889l.708-.707M4.5 10h1M6.11 6.111l.708.707"
            ></path>
        </svg>
    }.into_any()
}

#[component]
pub fn MoonIcon(#[prop(optional)] class: Option<String>) -> AnyView {
    view! {
        <svg viewBox="0 0 20 20" fill="none" aria-hidden="true" class=class>
            <path d="M15.224 11.724a5.5 5.5 0 0 1-6.949-6.949 5.5 5.5 0 1 0 6.949 6.949Z"></path>
        </svg>
    }
    .into_any()
}

#[component]
pub fn ModeToggle(mode: Signal<ColorMode>, set_mode: WriteSignal<ColorMode>) -> AnyView {
    view! {
        <button
            type="button"
            class="flex h-6 w-6 items-center justify-center transition hover:bg-zinc-900/5 dark:hover:bg-white/5 cursor-pointer"
            aria-label="Toggle dark mode"
            on:click=move |_| {
                let which_mode = match mode.get() {
                    ColorMode::Dark => ColorMode::Light,
                    _ => ColorMode::Dark,
                };
                set_mode.set(which_mode);
            }
        >

            <SunIcon class="h-5 w-5 stroke-zinc-900 dark:hidden".to_string()/>
            <MoonIcon class="hidden h-5 w-5 stroke-white dark:block".to_string()/>
        </button>
    }.into_any()
}

#[component]
pub fn TopLevelNavItem(href: String, children: Children) -> AnyView {
    view! {
        <li>
            <a
                href=href
                class="text-sm leading-5 text-zinc-600 transition hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-white"
            >
                {children()}
            </a>
        </li>
    }.into_any()
}

#[component]
pub fn Header(mode: Signal<ColorMode>, set_mode: WriteSignal<ColorMode>) -> AnyView {
    let show_search = RwSignal::new(false);
    let nr = NodeRef::<Input>::new();
    let _ = use_event_listener(use_document(), keydown, move |evt| {
        let is_ctrl = evt.ctrl_key();
        let is_meta = evt.meta_key();
        let is_k = evt.key() == "k";
        if (is_ctrl | is_meta) & is_k {
            leptos::logging::log!("Open search");
            show_search.set(true);
        }
    });

    Effect::new(move |_| {
        if show_search.get() {
            if let Some(inner) = nr.get() {
                leptos::logging::log!("focusing search input");
                let _ = inner.focus();
            } else {
                leptos::logging::log!("Failed to find input node ref")
            }
        }
    });

    let noder = NodeRef::<Div>::new();
    let show_tray = RwSignal::new(false);

    view! {
        <div class=move || { if !show_search.get() { "hidden" } else { "" } }>
            <SearchDialog show_search=show_search node_ref=nr/>
        </div>

        <div class=move || { if show_tray.get() { "" } else { "hidden" } }>
            <MobileTray show_tray=show_tray node_ref=noder/>
        </div>

        <div class="fixed inset-x-0 top-0 z-50 flex h-14 items-center justify-between gap-12 px-4 transition sm:px-6 lg:left-72 lg:z-30 lg:px-8 xl:left-80 backdrop-blur-sm bg-zinc-50 dark:bg-zinc-900">
            // divider
            <div class="absolute inset-x-0 top-full h-px transition bg-zinc-900/7.5 dark:bg-white/7.5"></div>
            <div class="hidden lg:block lg:max-w-md lg:flex-auto">
                <SearchButton show_search=show_search/>
            </div>

            // Mobile Navigation
            <nav class="block lg:hidden">
                <MobileNavigation show_tray=show_tray/>
            </nav>

            <div class="flex items-center gap-5">
                // Main navigation
                <nav class="hidden md:block">
                    <ul role="list" class="flex items-center gap-8">
                        <TopLevelNavItem href="/".to_string()>"API"</TopLevelNavItem>
                        <TopLevelNavItem href="/".to_string()>"Documentation"</TopLevelNavItem>
                    </ul>
                </nav>

                // Divider
                <div class="hidden md:block md:h-5 md:w-px md:bg-zinc-900/10 md:dark:bg-white/15"></div>

                <MobileSearch show_search=show_search/>
                <ModeToggle mode=mode set_mode=set_mode/>
            // </div>

            // Sign in button
            // <div class="hidden min-[416px]:contents">
            // <Button href="#".to_string()>"Sign in"</Button>
            // </div>
            </div>
        </div>
    }.into_any()
}

#[component]
fn ArrowIcon(#[prop(optional)] class: Option<String>) -> impl IntoView {
    view! {
        <svg viewBox="0 0 20 20" fill="none" aria-hidden="true" class=class>
            <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                d="m11.5 6.5 3 3.5m0 0-3 3.5m3-3.5h-9"
            ></path>
        </svg>
    }
}

// Define variant styles as a constant
const VARIANT_STYLES: [(&str, &str); 5] = [
    ("primary", " bg-zinc-900 py-1 px-3 text-white hover:bg-zinc-700 dark:bg-emerald-400/10 dark:text-emerald-400 dark:ring-1 dark:ring-inset dark:ring-emerald-400/20 dark:hover:bg-emerald-400/10 dark:hover:text-emerald-300 dark:hover:ring-emerald-300"),
    ("secondary", " bg-zinc-100 py-1 px-3 text-zinc-900 hover:bg-zinc-200 dark:bg-zinc-800/40 dark:text-zinc-400 dark:ring-1 dark:ring-inset dark:ring-zinc-800 dark:hover:bg-zinc-800 dark:hover:text-zinc-300"),
    ("filled", " bg-zinc-900 py-1 px-3 text-white hover:bg-zinc-700 dark:bg-emerald-500 dark:text-white dark:hover:bg-emerald-400"),
    ("outline", " py-1 px-3 text-zinc-700 ring-1 ring-inset ring-zinc-900/10 hover:bg-zinc-900/2.5 hover:text-zinc-900 dark:text-zinc-400 dark:ring-white/10 dark:hover:bg-white/5 dark:hover:text-white"),
    ("text", "text-emerald-500 hover:text-emerald-600 dark:text-emerald-400 dark:hover:text-emerald-500"),
];

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Filled,
    Outline,
    Text,
}

impl ButtonVariant {
    fn to_class(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => VARIANT_STYLES[0].1,
            ButtonVariant::Secondary => VARIANT_STYLES[1].1,
            ButtonVariant::Filled => VARIANT_STYLES[2].1,
            ButtonVariant::Outline => VARIANT_STYLES[3].1,
            ButtonVariant::Text => VARIANT_STYLES[4].1,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum ArrowDirection {
    Left,
    Right,
    None,
}

#[component]
pub fn Button(
    #[prop(optional)] variant: Option<ButtonVariant>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] href: Option<String>,
    #[prop(optional)] arrow: Option<ArrowDirection>,
    children: Children,
) -> AnyView {
    let variant = variant.unwrap_or(ButtonVariant::Primary);
    let arrow = arrow.unwrap_or(ArrowDirection::None);

    // Combine classes
    let base_classes =
        "inline-flex gap-0.5 justify-center overflow-hidden text-sm font-medium transition";
    let variant_class = variant.to_class();
    let additional_class = class.unwrap_or("".to_string());
    let combined_class = format!("{} {} {}", base_classes, variant_class, additional_class);

    // Arrow icon classes
    let arrow_base_class = "mt-0.5 h-5 w-5";
    let arrow_variant_class = if matches!(variant, ButtonVariant::Text) {
        "relative top-px"
    } else {
        ""
    };
    let arrow_direction_class = match arrow {
        ArrowDirection::Left => "-ml-1 rotate-180",
        ArrowDirection::Right => "-mr-1",
        ArrowDirection::None => "",
    };
    let arrow_classes = format!(
        "{} {} {}",
        arrow_base_class, arrow_variant_class, arrow_direction_class
    );

    match href {
        Some(href_value) => view! {
            <a href=href_value class=combined_class>
                {matches!(arrow, ArrowDirection::Left)
                    .then(|| view! { <ArrowIcon class=arrow_classes.clone()/> })}
                {children()}
                {matches!(arrow, ArrowDirection::Right)
                    .then(|| view! { <ArrowIcon class=arrow_classes.clone()/> })}
            </a>
        }
        .into_any(),
        None => view! {
            <button class=combined_class>
                {matches!(arrow, ArrowDirection::Left)
                    .then(|| view! { <ArrowIcon class=arrow_classes.clone()/> })} {children()}
                {matches!(arrow, ArrowDirection::Right)
                    .then(|| view! { <ArrowIcon class=arrow_classes.clone()/> })}
            </button>
        }
        .into_any(),
    }
    .into_any()
}

#[derive(Default)]
pub enum TagVariant {
    #[default]
    Medium,
    Small,
}

#[derive(Default)]
pub enum TagColor {
    #[default]
    Emerald,
    // Add other colors as needed
}

impl TagVariant {
    fn to_class(&self) -> &'static str {
        match self {
            TagVariant::Medium => "px-1.5 ring-1 ring-inset",
            TagVariant::Small => "px-1 ring-1 ring-inset",
        }
    }
}

impl TagColor {
    fn to_class(&self, variant: &TagVariant) -> &'static str {
        match (self, variant) {
            (TagColor::Emerald, TagVariant::Medium) => {
                "ring-emerald-300 bg-emerald-400/10 text-emerald-500 dark:text-emerald-400"
            }
            (TagColor::Emerald, TagVariant::Small) => {
                "ring-emerald-300 bg-emerald-400/10 text-emerald-500 dark:text-emerald-400"
            } // Add other color/variant combinations
        }
    }
}

#[component]
pub fn Tag(
    #[prop(optional)] variant: Option<TagVariant>,
    #[prop(optional)] color: Option<TagColor>,
    children: Children,
) -> impl IntoView {
    let variant = variant.unwrap_or_default();
    let color = color.unwrap_or_default();

    let classes = format!(
        "font-mono text-[0.625rem] font-semibold leading-6 {} {}",
        variant.to_class(),
        color.to_class(&variant)
    );

    view! { <span class=classes>{children()}</span> }
}

#[component]
pub fn NavLink(
    #[prop(optional)] href: String,
    #[prop(optional)] tag: Option<String>,
    #[prop(optional)] active: bool,
    #[prop(optional)] is_anchor_link: bool,
    children: Children,
) -> impl IntoView {
    let padding_class = if is_anchor_link { "pl-7" } else { "pl-4" };
    let active_class = if active {
        "text-zinc-900 dark:text-zinc-100"
    } else {
        "text-zinc-600 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-white"
    };

    let classes = format!(
        "flex justify-between gap-2 py-1 pr-3 text-sm transition {} {}",
        padding_class, active_class
    );

    view! {
        <a href=href aria-current=if active { "page" } else { "false" } class=classes>
            <span class="truncate">{children()}</span>
            {tag.map(|t| view! { <Tag variant=TagVariant::Small>{t}</Tag> })}

        </a>
    }
}

#[component]
pub fn NavigationGroup(
    group: (DocSection, Vec<&'static DocPage>),
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let location = leptos_router::hooks::use_location();
    let (title, links) = group;

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
                                let href = di.href.to_string();
                                let hr = href.clone();
                                let title = di.title.to_string();
                                let is_active = location.pathname.get().split_off(1) == hr;
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
                                        <NavLink href=href.into() active=is_active>

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

#[component]
pub fn MobileNavigation(show_tray: RwSignal<bool>) -> AnyView {
    view! {
        <div class="flex items-center gap-5 lg:hidden">
            <button
                id="mobile-nav-toggle"
                type="button"
                class="flex h-6 w-6 items-center justify-center transition hover:bg-zinc-900/5 dark:hover:bg-white/5 cursor-pointer"
                aria-label="Toggle navigation"
                on:click=move |_| {
                    let cur = show_tray.get();
                    leptos::logging::log!("show_tray = {cur:?}");
                    show_tray.set(!cur);
                    leptos::logging::log!("show_tray updated to {}", show_tray.get());
                }
            >

                {move || {
                    if show_tray.get() {
                        view! {
                            <CloseIcon class="w-4 stroke-zinc-900 dark:stroke-white ".to_string()/>
                        }
                            .into_any()
                    } else {
                        view! {
                            <MenuIcon class="w-4 stroke-zinc-900 dark:stroke-white ".to_string()/>
                        }
                            .into_any()
                    }
                }}

            </button>
            <HomeButton/>
        </div>
    }.into_any()
}

#[component]
pub fn MobileTray(show_tray: RwSignal<bool>, node_ref: NodeRef<Div>) -> AnyView {
    // FIXME this is not compatible with the tray signal
    // let _ = on_click_outside(node_ref, move |evt| {
    //     let target = evt.target();
    //     if let Some(tar) = target {
    //         logging::log!("Event {tar:?}");
    //     }
    //     show_tray.set(false);
    // });

    let _ = use_event_listener(use_document(), keydown, move |evt| {
        if evt.key() == "Escape" {
            leptos::logging::log!("Close mobile modal");
            show_tray.set(false);
        }
    });

    view! {
        <dialog class="fixed inset-0 z-50 block lg:hidden" aria-modal="true">
            <div class="fixed inset-0 bg-zinc-400/25 backdrop-blur-sm dark:bg-black/40"></div>
            <div
                node_ref=node_ref
                class="fixed bottom-0 left-0 top-14 w-full overflow-y-auto bg-zinc-50 px-4 pb-4 pt-6 shadow-lg shadow-zinc-900/10 ring-1 ring-zinc-900/7.5 dark:bg-zinc-900 dark:ring-zinc-800 min-[416px]:max-w-sm sm:px-6 sm:pb-10"
            >
                <Navigation/>
            </div>
        </dialog>
    }
    .into_any()
}

#[component]
pub fn MenuIcon(#[prop(optional)] class: Option<String>) -> AnyView {
    let class = class.unwrap_or("size-5".to_string());
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class=class
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
            ></path>
        </svg>
    }
    .into_any()
}

#[component]
pub fn CloseIcon(#[prop(optional)] class: Option<String>) -> AnyView {
    let class = class.unwrap_or("size-5".to_string());
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class=class
        >
            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12"></path>
        </svg>
    }
    .into_any()
}

#[component]
pub fn Navigation(#[prop(optional)] class: Option<&'static str>) -> impl IntoView {
    view! {
        <nav class=class>
            <ul role="list">
                // Map through navigation groups
                {doc_sections()
                    .into_iter()
                    .enumerate()
                    .map(|(i, group)| {
                        view! {
                            <NavigationGroup group=group class=if i == 0 { "md:mt-0" } else { "" }/>
                        }
                    })
                    .collect::<Vec<_>>()} // <Button
                <li class="sticky bottom-0 z-10 mt-6 min-[416px]:hidden">// href="#".to_string()
                // variant=ButtonVariant::Filled
                // class="w-full".to_string()
                // >
                // "Sign in"
                // </Button>
                </li>
            </ul>
        </nav>
    }
}
