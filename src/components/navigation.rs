use super::search::{MobileSearch, SearchButton, SearchDialog};
use super::version_selector::VersionSelector;
use crate::{
    HomeButton,
    docs::{DocPage, DocSection, doc_sections},
    versioning::get_current_version,
};
use leptos::{
    ev::keydown,
    html::{Div, Input},
    prelude::*,
};
use leptos_use::{ColorMode, use_document, use_event_listener};

// Lucide Sun icon (24x24 scaled to 20x20)
#[component]
pub fn SunIcon(#[prop(optional)] class: Option<String>) -> AnyView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class>
            <circle cx="12" cy="12" r="4"></circle>
            <path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M4.93 19.07l1.41-1.41M17.66 6.34l1.41-1.41"></path>
        </svg>
    }.into_any()
}

// Lucide Moon icon (24x24 scaled to 20x20)
#[component]
pub fn MoonIcon(#[prop(optional)] class: Option<String>) -> AnyView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class>
            <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"></path>
        </svg>
    }.into_any()
}

// Lucide SunMoon icon (24x24 scaled to 20x20)
#[component]
pub fn SunMoonIcon(#[prop(optional)] class: Option<String>) -> AnyView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class>
            <path d="M12 8a2.83 2.83 0 0 0 4 4 4 4 0 1 1-4-4"></path>
            <path d="M12 2v2M12 20v2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M2 12h2M20 12h2M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4"></path>
        </svg>
    }.into_any()
}

#[component]
pub fn ModeToggle(
    mode: Signal<ColorMode>,
    theme_override: ReadSignal<Option<ColorMode>>,
    set_theme_override: WriteSignal<Option<ColorMode>>,
) -> AnyView {
    let is_auto = move || theme_override.get().is_none();

    // Determine current theme state for cycling
    let current_state = move || {
        if is_auto() {
            "auto"
        } else if mode.get() == ColorMode::Light {
            "light"
        } else {
            "dark"
        }
    };

    // Cycle through states: auto -> light -> dark -> auto
    let cycle_theme = move |_| match current_state() {
        "auto" => set_theme_override.set(Some(ColorMode::Light)),
        "light" => set_theme_override.set(Some(ColorMode::Dark)),
        _ => set_theme_override.set(None),
    };

    // Get the appropriate icon and label
    let icon_and_label = move || match current_state() {
        "auto" => (
            "Auto",
            view! { <SunMoonIcon class="h-5 w-5 stroke-zinc-900 dark:stroke-white".to_string()/> }
                .into_any(),
        ),
        "light" => (
            "Light",
            view! { <SunIcon class="h-5 w-5 stroke-zinc-900 dark:stroke-zinc-400".to_string()/> }
                .into_any(),
        ),
        _ => (
            "Dark",
            view! { <MoonIcon class="h-5 w-5 stroke-zinc-400 dark:stroke-white".to_string()/> }
                .into_any(),
        ),
    };

    view! {
        <button
            type="button"
            class="flex h-6 w-6 items-center justify-center rounded transition hover:bg-zinc-900/5 dark:hover:bg-white/5 cursor-pointer"
            aria-label=move || format!("{} theme (click to cycle)", icon_and_label().0)
            title=move || format!("{} (click to change)", icon_and_label().0)
            on:click=cycle_theme
        >
            {move || icon_and_label().1}
        </button>
    }.into_any()
}

#[component]
pub fn TopLevelNavItem(href: String, children: Children) -> AnyView {
    let default_class = "text-sm leading-5 text-zinc-600 transition hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-white";
    view! {
        <li>
            <a href=href class=default_class>
                {children()}
            </a>
        </li>
    }
    .into_any()
}

#[component]
pub fn Header(
    mode: Signal<ColorMode>,
    theme_override: ReadSignal<Option<ColorMode>>,
    set_theme_override: WriteSignal<Option<ColorMode>>,
) -> AnyView {
    let show_search = RwSignal::new(false);
    let nr = NodeRef::<Input>::new();
    let _ = use_event_listener(use_document(), keydown, move |evt| {
        let is_ctrl = evt.ctrl_key();
        let is_meta = evt.meta_key();
        let is_k = evt.key() == "k";
        if (is_ctrl | is_meta) & is_k {
            show_search.set(true);
        }
    });

    Effect::new(move |_| {
        if show_search.get()
            && let Some(inner) = nr.get()
        {
            let _ = inner.focus();
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

        <div class="fixed inset-x-0 top-0 z-50 flex h-14 items-center justify-between gap-12 px-4 transition sm:px-6 lg:left-72 lg:z-30 lg:px-8 xl:left-80 backdrop-blur-sm ">
            // divider
            <div class="absolute inset-x-0 top-full h-px transition bg-zinc-900/7.5 dark:bg-white/7.5"></div>
            <div class="hidden lg:flex lg:items-center lg:gap-4 lg:max-w-md lg:flex-auto">
                <SearchButton show_search=show_search/>
                <VersionSelector/>
                <span class="inline-flex items-center rounded-md bg-amber-50 px-2 py-1 text-xs font-medium text-amber-800 ring-1 ring-inset ring-amber-600/20 dark:bg-amber-400/10 dark:text-amber-400 dark:ring-amber-400/30">
                    "BETA"
                </span>
            </div>

            // Mobile Navigation
            <nav class="block lg:hidden">
                <MobileNavigation show_tray=show_tray/>
            </nav>

            <div class="flex items-center gap-5">
                // Main navigation
                <nav class="hidden md:block">
                    <ul role="list" class="flex items-center gap-8">
                        <TopLevelNavItem href="/api".to_string()>"API"</TopLevelNavItem>
                        <TopLevelNavItem href=format!("/docs/{}", get_current_version().path)>"Documentation"</TopLevelNavItem>
                    </ul>
                </nav>

                // Divider
                <div class="hidden md:block md:h-5 md:w-px md:bg-zinc-900/10 md:dark:bg-white/15"></div>

                <MobileSearch show_search=show_search/>
                <ModeToggle mode=mode theme_override=theme_override set_theme_override=set_theme_override/>
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
    (
        "primary",
        " bg-zinc-900 py-1 px-3 text-white hover:bg-zinc-700 dark:bg-emerald-400/10 dark:text-emerald-400 dark:ring-1 dark:ring-inset dark:ring-emerald-400/20 dark:hover:bg-emerald-400/10 dark:hover:text-emerald-300 dark:hover:ring-emerald-300",
    ),
    (
        "secondary",
        " bg-zinc-100 py-1 px-3 text-zinc-900 hover:bg-zinc-200 dark:bg-zinc-800/40 dark:text-zinc-400 dark:ring-1 dark:ring-inset dark:ring-zinc-800 dark:hover:bg-zinc-800 dark:hover:text-zinc-300",
    ),
    (
        "filled",
        " bg-zinc-900 py-1 px-3 text-white hover:bg-zinc-700 dark:bg-emerald-500 dark:text-white dark:hover:bg-emerald-400",
    ),
    (
        "outline",
        " py-1 px-3 text-zinc-700 ring-1 ring-inset ring-zinc-900/10 hover:bg-zinc-900/2.5 hover:text-zinc-900 dark:text-zinc-400 dark:ring-white/10 dark:hover:bg-white/5 dark:hover:text-white",
    ),
    (
        "text",
        "text-emerald-500 hover:text-emerald-600 dark:text-emerald-400 dark:hover:text-emerald-500",
    ),
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
                                let base_href = di.href.to_string();
                                let title = di.title.to_string();
                                let current_path = location.pathname.get();

                                // Extract version from current path if present
                                let (current_version, _) = if current_path.contains("/docs/v") || current_path.contains("/docs/dev") {
                                    let parts: Vec<&str> = current_path.split('/').collect();
                                    if parts.len() >= 3 && (parts[2].starts_with("v") || parts[2] == "dev") {
                                        (Some(parts[2]), parts[3..].join("/"))
                                    } else {
                                        (None, String::new())
                                    }
                                } else {
                                    (None, String::new())
                                };

                                // Generate version-aware href
                                let href = match current_version {
                                    Some(version) => format!("/docs/{}{}", version, base_href),
                                    None => format!("/docs{}", base_href),
                                };

                                // Check if this link is active
                                let mut p = current_path.clone();
                                if p.starts_with("/docs") {
                                    p = p.split_off(5);
                                    // Remove version prefix if present
                                    if p.starts_with("/v") || p.starts_with("/dev") {
                                        let parts: Vec<&str> = p.splitn(3, '/').collect();
                                        if parts.len() > 2 {
                                            p = format!("/{}", parts[2]);
                                        }
                                    }
                                }
                                let is_active = p == base_href;
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
                    show_tray.set(!cur);
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

    let location = leptos_router::hooks::use_location();

    let _ = use_event_listener(use_document(), keydown, move |evt| {
        if evt.key() == "Escape" {
            show_tray.set(false);
        }
    });

    view! {
        <dialog class="fixed inset-0 z-50 block lg:hidden" aria-modal="true">
            <div class="fixed inset-0 backdrop-blur-sm dark:bg-black/40"></div>
            <div
                node_ref=node_ref
                class="fixed bottom-0 left-0 top-14 w-full overflow-y-auto bg-zinc-50 px-4 pb-4 pt-6 shadow-lg shadow-zinc-900/10 ring-1 ring-zinc-900/7.5 dark:bg-zinc-900 dark:ring-zinc-800 min-[416px]:max-w-sm sm:px-6 sm:pb-10"
            >
                {move || {
                    let is_api = location.pathname.get().starts_with("/api");
                    if is_api {
                        view! { <crate::api::ApiNavigation></crate::api::ApiNavigation> }.into_any()
                    } else {
                        view! { <Navigation/> }.into_any()
                    }
                }}

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
                    .collect::<Vec<_>>()}
                <li class="sticky bottom-0 z-10 mt-6 min-[416px]:hidden"></li>
            </ul>
        </nav>
    }
}
