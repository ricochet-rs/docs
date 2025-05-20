use leptos::prelude::*;

#[component]
pub fn GridPattern(
    width: f64,
    height: f64,
    x: f64,
    y: f64,
    squares: Option<Vec<(f64, f64)>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView {
    let pattern_id = String::from("grid-pattern-id");

    view! {
        <svg aria-hidden="true" class=class style=style>
            <defs>
                <pattern
                    id=pattern_id.clone()
                    width=width
                    height=height
                    patternUnits="userSpaceOnUse"
                    x=x
                    y=y
                >
                    <path d=format!("M.5 {}V.5H{}", height, width) fill="none"></path>
                </pattern>
            </defs>

            <rect
                width="100%"
                height="100%"
                stroke-width="0"
                fill=format!("url(#{})", pattern_id)
            ></rect>

            {squares
                .map(|sq| {
                    view! {
                        <svg x=x y=y class="overflow-visible">
                            {sq
                                .into_iter()
                                .map(|(sx, sy)| {
                                    view! {
                                        <rect
                                            stroke-width="0"
                                            width=width + 1.0
                                            height=height + 1.0
                                            x=sx * width
                                            y=sy * height
                                        ></rect>
                                    }
                                })
                                .collect_view()}
                        </svg>
                    }
                })}

        </svg>
    }
}

#[component]
pub fn HeroPattern() -> AnyView {
    view! {
        <div id="hero-pattern" class="absolute inset-0 mx-0 max-w-none overflow-hidden">
            <div class="absolute left-1/2 top-0 ml-[-38rem] h-[30rem] w-[81.25rem] dark:[mask-image:linear-gradient(white,transparent)]">
                <div class="absolute inset-0 bg-gradient-to-r from-[#ba8cbf] to-[#615391] opacity-40 [mask-image:radial-gradient(farthest-side_at_top,white,transparent)] dark:from-[#ba8cbf]/30 dark:to-[#615391]/30 dark:opacity-100">
                    <GridPattern
                        width=72.0
                        height=23.0
                        x=-30.0
                        y=-100.0
                        squares=Some(vec![(4.0, 3.0), (2.0, 1.0), (7.0, 3.0), (10.0, 6.0)])

                        class="absolute inset-x-0 inset-y-[-50%] h-[200%] w-full skew-y-[-18deg] fill-black/40 stroke-black/50 mix-blend-overlay dark:fill-white/2.5 dark:stroke-white/5"
                            .to_string()
                    />
                </div>
                <svg
                    viewBox="0 0 1113 440"
                    aria-hidden="true"
                    class="absolute left-1/2 top-0 ml-[-19rem] w-[69.5625rem] fill-white blur-[26px] dark:hidden opacity-20"
                >
                    <path d="M.016 439.5s-9.5-300 434-300S882.516 20 882.516 20V0h230.004v439.5H.016Z"></path>
                </svg>
            </div>
        </div>
    }.into_any()
}
