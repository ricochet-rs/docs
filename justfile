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

# Add a new version to the docs
add-version VERSION:
    #!/usr/bin/env bash
    set -e
    echo "🆕 Adding version {{VERSION}}"

    # Find the latest version by checking the versioning.rs file
    LATEST_VERSION=$(awk '/is_latest: true/{found=1} found && /path:/{gsub(/[",]/, "", $2); print $2; exit}' src/versioning.rs)
    if [ -z "$LATEST_VERSION" ]; then
        LATEST_VERSION="v0.1"
        echo "⚠️  Could not detect latest version, defaulting to v0.1"
    else
        echo "📋 Using $LATEST_VERSION as template (marked as latest)"
    fi

    # Create content directory
    mkdir -p "src/content/{{VERSION}}"
    echo "📁 Created src/content/{{VERSION}}"

    # Create generated directory
    mkdir -p "src/generated/{{VERSION}}"
    echo "📁 Created src/generated/{{VERSION}}"

    # Copy existing content as template (from latest version)
    if [ -d "src/content/$LATEST_VERSION" ]; then
        find "src/content/$LATEST_VERSION" -type f -name "*.qmd" -exec cp {} "src/content/{{VERSION}}/" \;
        COPIED_COUNT=$(find "src/content/$LATEST_VERSION" -name "*.qmd" | wc -l)
        echo "📋 Copied $COPIED_COUNT files from $LATEST_VERSION"
    else
        echo "⚠️  Latest version directory not found, creating empty structure"
    fi

    # Add version to versioning.rs and make it the new latest
    echo "🔧 Adding version to src/versioning.rs..."

    # First, set all existing versions to is_latest: false
    sed -i.bak 's/is_latest: true/is_latest: false/g' src/versioning.rs

    # Update the array size
    CURRENT_SIZE=$(grep -o "VERSIONS: \[Version; [0-9]\+\]" src/versioning.rs | grep -o "[0-9]\+")
    NEW_SIZE=$((CURRENT_SIZE + 1))
    # Use a more specific pattern to avoid multiple replacements
    sed -i.bak "s/pub const VERSIONS: \[Version; $CURRENT_SIZE\]/pub const VERSIONS: [Version; $NEW_SIZE]/" src/versioning.rs

    # Add the new version entry with proper indentation before the closing bracket
    sed -i.bak "/^];$/i\\
    Version {\\
        label: \"{{VERSION}}\",\\
        path: \"{{VERSION}}\",\\
        is_latest: true,\\
    }," src/versioning.rs

    # Clean up backup file
    rm -f src/versioning.rs.bak

    echo "🔧 Updating docs module..."

    # Create DOC_PAGES array for the new version
    LATEST_PAGES_NAME="DOC_PAGES_$(echo "$LATEST_VERSION" | tr '.' '_' | tr '[:lower:]' '[:upper:]')"
    NEW_PAGES_NAME="DOC_PAGES_$(echo "{{VERSION}}" | tr '.' '_' | tr '[:lower:]' '[:upper:]')"

    # Find and copy the latest version's DOC_PAGES array
    sed -n "/^pub const $LATEST_PAGES_NAME:/,/^];$/p" src/docs/mod.rs > /tmp/latest_pages.txt

    # Replace the array name and update paths in the copied content
    sed "s/$LATEST_PAGES_NAME/$NEW_PAGES_NAME/g" /tmp/latest_pages.txt | \
    sed "s|include_str!(\"../generated/$LATEST_VERSION/|include_str!(\"../generated/{{VERSION}}/|g" > /tmp/new_pages.txt

    # Insert the new DOC_PAGES array before the first pub fn get_doc (only if it doesn't already exist)
    if ! grep -q "$NEW_PAGES_NAME" src/docs/mod.rs; then
        awk '/^pub fn get_doc\(/ && !inserted {system("cat /tmp/new_pages.txt"); inserted=1} {print}' src/docs/mod.rs > /tmp/docs_updated.rs
        mv /tmp/docs_updated.rs src/docs/mod.rs
    fi

    # Add match arm to get_doc_for_version function before the default case
    sed -i.bak "/_ => &DOC_PAGES_V0_1/i\\
        \"{{VERSION}}\" => &$NEW_PAGES_NAME," src/docs/mod.rs

    # Update the default case to point to the new latest version
    sed -i.bak "s/_ => &DOC_PAGES_V0_1, \/\/ Default to latest stable/_ => \&$NEW_PAGES_NAME, \/\/ Default to latest stable/" src/docs/mod.rs

    # Clean up temp files and backup file
    rm -f /tmp/latest_pages.txt /tmp/new_pages.txt src/docs/mod.rs.bak

    echo "✅ Version {{VERSION}} added to versioning.rs!"
    echo "✅ Version {{VERSION}} added to docs module!"
    echo "🎨 Running formatter..."
    just fmt >/dev/null 2>&1

    echo "📄 Generating documentation files..."
    just doc >/dev/null 2>&1

    echo "✅ Version {{VERSION}} structure created and ready!"
    echo "🎉 New version is now available and set as latest"

# Remove a version from the docs
remove-version VERSION:
    #!/usr/bin/env bash
    set -e
    echo "🗑️  Removing version {{VERSION}}"

    # Safety check
    if [ "{{VERSION}}" = "v0.1" ] || [ "{{VERSION}}" = "dev" ]; then
        echo "❌ Cannot remove core version {{VERSION}}"
        exit 1
    fi

    # Remove directories
    rm -rf "src/content/{{VERSION}}"
    rm -rf "src/generated/{{VERSION}}"

    echo "✅ Version {{VERSION}} directories removed!"
    echo "📝 Manual cleanup needed:"
    echo "  1. Remove version from VERSIONS array in src/versioning.rs"
    echo "  2. Remove DOC_PAGES_{{VERSION}} array in src/docs/mod.rs"
    echo "  3. Update get_doc_for_version() match in src/docs/mod.rs"

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

build TARGET="":
    #!/usr/bin/env bash
    if [ -n "{{TARGET}}" ]; then
        CARGO_BUILD_TARGET={{TARGET}} cargo leptos build --release
    else
        cargo leptos build --release
    fi

# Build binary in Docker and extract to local filesystem
build-docker PLATFORM="linux/amd64":
    #!/usr/bin/env bash
    mkdir -p target/docker-build
    docker run --rm \
        --platform {{PLATFORM}} \
        -v "$(pwd):/workspace" \
        -v "$(pwd)/target/docker-build:/output" \
        -w /workspace \
        reg.devxy.io/docker.io/library/rust:1.89 \
        bash -c "
            rustup target add wasm32-unknown-unknown && \
            cargo install cargo-leptos && \
            curl -fsSL https://deb.nodesource.com/setup_22.x | bash - && \
            apt-get install -y nodejs && \
            npm install -g pnpm && \
            CI=true pnpm install --frozen-lockfile && \
            cargo leptos build --release && \
            cp target/release/ricochet-docs /output/
        "

docker-build PLATFORM="linux/amd64":
    docker build --platform {{PLATFORM}} -t ricochet-docs .

docker-build-multi:
    docker buildx build --platform linux/amd64,linux/arm64 -t ricochet-docs .

prod:
    cargo leptos serve --precompress --release

serve-prod:
    nohup trunk serve --release >/dev/null 2>&1
