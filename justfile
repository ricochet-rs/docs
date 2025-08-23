set dotenv-load:=true

deps:
    cargo install cargo-leptos

fmt:
    leptosfmt ricochet-ui/src/**/*.rs && leptosfmt ricochet-ui/src/*.rs && cargo fmt --all

lint:
    cargo clippy --all-targets --all-features -- -D warnings

lint-fix:
    cargo clippy --fix --all-targets --all-features -- -D warnings

doc:
    ./render-doc.R && node highlight-partials.mjs

serve:
    cargo leptos watch --hot-reload --precompress

prod:
    cargo leptos serve --precompress --release

serve-prod:
    nohup trunk serve --release >/dev/null 2>&1
