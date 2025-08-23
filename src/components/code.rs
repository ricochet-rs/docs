use leptos::prelude::*;

#[component]
pub fn CodePanel() -> impl IntoView {
    
}

#[component]
pub fn CodeGroupHeader() -> impl IntoView {
    view! {
        <div class="not-prose my-6 overflow-hidden bg-zinc-900 shadow-md dark:ring-1 dark:ring-white/10">
            <div class="flex min-h-[calc(theme(spacing.12)+1px)] flex-wrap items-start gap-x-4 border-b border-zinc-700 bg-zinc-800 px-4 dark:border-zinc-800 dark:bg-transparent">
                <div
                    class="-mb-px flex gap-4 text-xs font-medium"
                    role="tablist"
                    aria-orientation="horizontal"
                >
                    <button
                        class="border-b py-3 transition focus:[&amp;:not(:focus-visible)]:outline-none border-emerald-500 text-emerald-400"
                        id="headlessui-tabs-tab-:r3l:"
                        role="tab"
                        type="button"
                        aria-selected="true"
                        tabindex="0"
                        data-headlessui-state="selected"
                        aria-controls="headlessui-tabs-panel-:r3t:"
                    >
                        cURL
                    </button>
                    <button
                        class="border-b py-3 transition focus:[&amp;:not(:focus-visible)]:outline-none border-transparent text-zinc-400 hover:text-zinc-300"
                        id="headlessui-tabs-tab-:r3n:"
                        role="tab"
                        type="button"
                        aria-selected="false"
                        tabindex="-1"
                        data-headlessui-state=""
                        aria-controls="headlessui-tabs-panel-:r3v:"
                    >
                        JavaScript
                    </button>
                    <button
                        class="border-b py-3 transition focus:[&amp;:not(:focus-visible)]:outline-none border-transparent text-zinc-400 hover:text-zinc-300"
                        id="headlessui-tabs-tab-:r3p:"
                        role="tab"
                        type="button"
                        aria-selected="false"
                        tabindex="-1"
                        data-headlessui-state=""
                        aria-controls="headlessui-tabs-panel-:r41:"
                    >
                        Python
                    </button>
                    <button
                        class="border-b py-3 transition focus:[&amp;:not(:focus-visible)]:outline-none border-transparent text-zinc-400 hover:text-zinc-300"
                        id="headlessui-tabs-tab-:r3r:"
                        role="tab"
                        type="button"
                        aria-selected="false"
                        tabindex="-1"
                        data-headlessui-state=""
                        aria-controls="headlessui-tabs-panel-:r43:"
                    >
                        PHP
                    </button>
                </div>
            </div>
            <div>
                <div
                    id="headlessui-tabs-panel-:r3t:"
                    role="tabpanel"
                    tabindex="0"
                    data-headlessui-state="selected"
                    aria-labelledby="headlessui-tabs-tab-:r3l:"
                >
                    <div class="group dark:bg-white/2.5">
                        <div class="relative">
                            <pre class="overflow-x-auto p-4 text-xs text-white">
                                <code class="language-bash">
                                    <span>
                                        <span style="color: var(--shiki-token-comment)">
                                            # cURL is most likely already installed on your machine
                                        </span>
                                    </span>
                                    <span>
                                        <span style="color: var(--shiki-color-text)">
                                            curl --version
                                        </span>
                                    </span>
                                    <span></span>
                                </code>
                            </pre>
                            <button
                                type="button"
                                class="group/button absolute right-4 top-3.5 overflow-hidden py-1 pl-2 pr-3 text-2xs font-medium opacity-0 backdrop-blur transition focus:opacity-100 group-hover:opacity-100 bg-white/5 hover:bg-white/7.5 dark:bg-white/2.5 dark:hover:bg-white/5"
                            >
                                <span
                                    aria-hidden="false"
                                    class="pointer-events-none flex items-center gap-0.5 text-zinc-400 transition duration-300"
                                >
                                    <svg
                                        viewBox="0 0 20 20"
                                        aria-hidden="true"
                                        class="h-5 w-5 fill-zinc-500/20 stroke-zinc-500 transition-colors group-hover/button:stroke-zinc-400"
                                    >
                                        <path
                                            stroke-width="0"
                                            d="M5.5 13.5v-5a2 2 0 0 1 2-2l.447-.894A2 2 0 0 1 9.737 4.5h.527a2 2 0 0 1 1.789 1.106l.447.894a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-5a2 2 0 0 1-2-2Z"
                                        ></path>
                                        <path
                                            fill="none"
                                            stroke-linejoin="round"
                                            d="M12.5 6.5a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-5a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2m5 0-.447-.894a2 2 0 0 0-1.79-1.106h-.527a2 2 0 0 0-1.789 1.106L7.5 6.5m5 0-1 1h-3l-1-1"
                                        ></path>
                                    </svg>
                                    Copy
                                </span>
                                <span
                                    aria-hidden="true"
                                    class="pointer-events-none absolute inset-0 flex items-center justify-center text-emerald-400 transition duration-300 translate-y-1.5 opacity-0"
                                >
                                    Copied!
                                </span>
                            </button>
                        </div>
                    </div>
                </div>
                <span
                    id="headlessui-tabs-panel-:r3v:"
                    role="tabpanel"
                    tabindex="-1"
                    style="position: fixed; top: 1px; left: 1px; width: 1px; height: 0px; padding: 0px; margin: -1px; overflow: hidden; clip: rect(0px, 0px, 0px, 0px); white-space: nowrap; border-width: 0px;"
                    aria-labelledby="headlessui-tabs-tab-:r3n:"
                ></span>
                <span
                    id="headlessui-tabs-panel-:r41:"
                    role="tabpanel"
                    tabindex="-1"
                    style="position: fixed; top: 1px; left: 1px; width: 1px; height: 0px; padding: 0px; margin: -1px; overflow: hidden; clip: rect(0px, 0px, 0px, 0px); white-space: nowrap; border-width: 0px;"
                    aria-labelledby="headlessui-tabs-tab-:r3p:"
                ></span>
                <span
                    id="headlessui-tabs-panel-:r43:"
                    role="tabpanel"
                    tabindex="-1"
                    style="position: fixed; top: 1px; left: 1px; width: 1px; height: 0px; padding: 0px; margin: -1px; overflow: hidden; clip: rect(0px, 0px, 0px, 0px); white-space: nowrap; border-width: 0px;"
                    aria-labelledby="headlessui-tabs-tab-:r3r:"
                ></span>
            </div>
        </div>
    }
}
