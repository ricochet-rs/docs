set dotenv-load := true

# renovate: datasource=docker depName=lycheeverse/lychee
lychee_version := "0.22-alpine"

build:
    bun astro build

preview:
    bun astro dev

# check for broken links (builds first, runs lychee via docker)
links: build
    docker run --rm -v {{justfile_directory()}}:/app -w /app lycheeverse/lychee:{{lychee_version}} dist/ --root-dir dist/ -t 40 --max-redirects 10 --exclude-loopback --insecure --cache --max-cache-age 1d

install:
    bun install

# lint using prettier
lint:
    bun prettier --check .

# fix lints using prettier and format prose (semantic line breaks)
fmt:
    bun run scripts/semantic-breaks.mjs
    prettier --write .
