doc:
    ./render-doc.R && node highlight-partials.mjs

serve:
    LEPTOS_TAILWIND_VERSION=v4.1.6 cargo leptos watch --hot-reload --precompress

prod:
    LEPTOS_TAILWIND_VERSION=v4.1.6 cargo leptos serve --precompress --release

serve-prod:
    nohup trunk serve --release >/dev/null 2>&1
