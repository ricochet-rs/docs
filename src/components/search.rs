use leptos::{
    ev::keydown,
    html::{Input, Li},
    prelude::*,
};
use leptos_use::{on_click_outside, use_document, use_element_hover, use_event_listener};

use crate::{docs::DOC_PAGES, search_engine::query_engine};

#[component]
pub fn SearchIcon(#[prop(optional)] class: Option<String>) -> AnyView {
    let class = class.unwrap_or(String::from("h-5 w-5 stroke-current"));
    view! {
        <svg viewBox="0 0 20 20" fill="none" aria-hidden="true" class=class>
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M12.01 12a4.25 4.25 0 1 0-6.02-6 4.25 4.25 0 0 0 6.02 6Zm0 0 3.24 3.25"
            ></path>
        </svg>
    }
    .into_any()
}

#[component]
pub fn NoResultsIcon(#[prop(optional)] class: Option<String>) -> AnyView {
    view! {
        <svg viewBox="0 0 20 20" fill="none" aria-hidden="true" class=class>
            <path
                strokeLinecap="round"
                strokeLinejoin="round"
                d="M12.01 12a4.237 4.237 0 0 0 1.24-3c0-.62-.132-1.207-.37-1.738M12.01 12A4.237 4.237 0 0 1 9 13.25c-.635 0-1.237-.14-1.777-.388M12.01 12l3.24 3.25m-3.715-9.661a4.25 4.25 0 0 0-5.975 5.908M4.5 15.5l11-11"
            ></path>
        </svg>
    }.into_any()
}

#[component]
pub fn SearchResult(idx: usize, res_idx: usize) -> AnyView {
    let search_item = &DOC_PAGES[idx];
    let item = NodeRef::<Li>::new();
    let is_hovered = use_element_hover(item);
    let class = "group block cursor-default px-4 py-3 aria-selected:bg-zinc-50 dark:aria-selected:bg-zinc-800/50 cursor-pointer";
    let cls = if res_idx > 0 {
        format!("{class} border-t border-zinc-100 dark:border-zinc-800")
    } else {
        class.to_string()
    };
    view! {
        <a href=search_item.href>
            <li
                node_ref=item
                class=cls
                role="option"
                aria-selected=move || if is_hovered.get() { "true" } else { "false" }
            >

                <div
                    id=search_item.href.to_string()
                    aria-hidden="true"
                    class="text-sm font-medium text-zinc-900 group-aria-selected:text-emerald-500 dark:text-white"
                >
                    // TODO highlight
                    {search_item.title.to_string()}
                // <HighlightQuery text={result.title} query={query} />
                </div>

                <div
                    id=format!("{}-hierarchy", search_item.href)
                    aria-hidden="true"
                    class="flex text-xs mt-1 truncate whitespace-nowrap text-2xs text-zinc-500"
                >
                    // {hierarchy.map((item, itemIndex, items) => (
                    <p>{search_item.section.to_string()}</p>
                    // <HighlightQuery text={item} query={query} />
                    <span class="mx-2 text-zinc-300 dark:text-zinc-700">/</span>
                    <p>{search_item.title}</p>
                // ))}
                </div>
            // )}
            </li>
        </a>
    }.into_any()
}

#[component]
pub fn SearchResults(query: RwSignal<String>) -> AnyView {
    let res = move || query_engine(&query.get(), Some(5));
    view! {
        {move || {
            let items = res();
            let n = items.len();
            match n == 0 {
                true => {
                    view! {
                        <div class="p-6 text-center">
                            <NoResultsIcon class="mx-auto h-5 w-5 stroke-zinc-900 dark:stroke-zinc-600"
                                .into()/>
                            <p class="mt-2 text-xs text-zinc-700 dark:text-zinc-400">
                                "Nothing found for "
                                <strong class="break-words font-semibold text-zinc-900 dark:text-white">
                                    "\"" {move || query.get()} "\""
                                </strong> ". Please try again."
                            </p>
                        </div>
                    }
                        .into_any()
                }
                false => {
                    let v = items
                        .into_iter()
                        .enumerate()
                        .map(|(idx, i)| {
                            leptos::logging::log!("{i:?}");
                            view! { <SearchResult idx=i.document.id as usize res_idx=idx/> }
                        })
                        .collect_view();
                    view! { <ul role="list">{v}</ul> }.into_any()
                }
            }
        }}
    }
    .into_any()
}

#[component]
pub fn SearchButton(show_search: RwSignal<bool>) -> AnyView {
    view! {
        <button
            type="button"
            class="hover:cursor-pointer hidden h-8 w-full items-center gap-2  bg-zinc-50 pl-2 pr-3 text-sm text-zinc-500 ring-1 ring-zinc-900/10 transition hover:ring-zinc-900/20 dark:bg-white/5 dark:text-zinc-400 dark:ring-inset dark:ring-white/10 dark:hover:ring-white/20 lg:flex focus:[&amp;:not(:focus-visible)]:outline-none"
            on:click=move |_| {
                show_search.set(true);
            }
        >

            <SearchIcon/>
            "Start deploying..."
            <kbd class="ml-auto text-2xs text-zinc-400 dark:text-zinc-500">
                <kbd class="font-sans">"⌘"</kbd>
                <kbd class="font-sans">K</kbd>
            </kbd>
        </button>
    }.into_any()
}

#[component]
pub fn SearchInput(
    query: RwSignal<String>,
    show_search: RwSignal<bool>,
    node_ref: NodeRef<Input>,
) -> AnyView {
    let _ = on_click_outside(node_ref, move |_| {
        show_search.set(false);
    });

    view! {
        <div class="group relative flex h-12">
            <SearchIcon class="pointer-events-none absolute left-3 top-0 h-full w-5 stroke-zinc-500"
                .to_string()/>
            <input
                bind:value=query
                on:input=move |_| {
                    query_engine(&query.get(), Some(5));
                }

                id="modal-search-input"
                node_ref=node_ref
                class="flex-auto appearance-none bg-transparent pl-10 text-zinc-900 outline-none placeholder:text-zinc-500 focus:w-full focus:flex-none dark:text-white sm:text-sm [&::-webkit-search-cancel-button]:hidden [&::-webkit-search-decoration]:hidden [&::-webkit-search-results-button]:hidden [&::-webkit-search-results-decoration]:hidden"
                placeholder="Find something..."
            />
        // {autocompleteState.status === 'stalled' && (
        // <div className="absolute inset-y-0 right-3 flex items-center">
        // <LoadingIcon className="h-5 w-5 animate-spin stroke-zinc-200 text-zinc-900 dark:stroke-zinc-800 dark:text-emerald-400" />
        // </div>
        // )}
        </div>
    }.into_any()
}

#[component]
pub fn SearchDialog(show_search: RwSignal<bool>, node_ref: NodeRef<Input>) -> AnyView {
    let _ = use_event_listener(use_document(), keydown, move |evt| {
        if evt.key() == "Escape" {
            leptos::logging::log!("Close search");
            show_search.set(false);
        }
    });

    let search_query = RwSignal::new(String::new());

    view! {
        <dialog class="fixed inset-0 z-60 block" aria-modal="true">

            <div class="fixed inset-0 bg-zinc-400/25 backdrop-blur-sm dark:bg-black/40"></div>
            <div class="fixed inset-0 overflow-y-auto px-4 py-4 sm:px-6 sm:py-20 md:py-32 lg:px-8 lg:py-[15vh]">

                <div class="mx-auto overflow-hidden bg-zinc-50 shadow-xl ring-1 ring-zinc-900/7.5 dark:bg-zinc-900 dark:ring-zinc-800 sm:max-w-xl">
                    <form
                        on:submit=move |ev| {
                            ev.prevent_default();
                        }

                        role="search"
                    >

                        <SearchInput query=search_query show_search=show_search node_ref=node_ref/>
                        <div class=move || {
                            leptos::logging::log!("Search query is: {:?}", search_query.get());
                            if search_query.get().is_empty() { "hidden" } else { "" }
                        }>
                            <div class="border-t border-zinc-200 bg-white empty:hidden dark:border-zinc-100/5 dark:bg-white/2.5">
                                <SearchResults query=search_query/>
                            </div>
                        </div>
                    </form>
                </div>
            </div>
        </dialog>
    }
    .into_any()
}

#[component]
pub fn MobileSearch(show_search: RwSignal<bool>) -> AnyView {
    view! {
        <div class="contents lg:hidden">
            <button
                type="button"
                class="flex h-6 w-6 items-center justify-center transition hover:bg-zinc-900/5 dark:hover:bg-white/5 lg:hidden focus:[&:not(:focus-visible)]:outline-none"
                aria-label="Find something..."
                on:click=move |_| {
                    show_search.set(true);
                }
            >

                <SearchIcon class="h-5 w-5 stroke-zinc-900 dark:stroke-white".to_string()/>
            </button>
        </div>
    }.into_any()
}
