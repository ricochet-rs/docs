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

# Development mode: watch both content and Rust files
serve:
    #!/usr/bin/env bash
    # Kill any existing background processes when script exits
    trap 'kill $(jobs -p) 2>/dev/null' EXIT

    echo "🚀 Starting development mode..."
    echo "📝 Content watcher will regenerate docs when .qmd files change"
    echo "⚡ Leptos will hot-reload when Rust files or generated docs change"
    echo ""

    # Start content watcher in background
    node watch-content.mjs &

    # Start leptos watch (this blocks)
    cargo leptos watch --hot-reload --precompress

prod:
    cargo leptos serve --precompress --release

serve-prod:
    nohup trunk serve --release >/dev/null 2>&1
