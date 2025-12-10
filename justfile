set dotenv-load:=true

build:
    bunx --bun astro build

preview:
    bunx --bun astro dev

links:
  lychee dist/ -t 40 --max-redirects 10 --exclude-loopback --insecure --exclude-path src/ --cache --max-cache-age 1d

# lint using prettier
lint:
    bunx --bun prettier --check .

# fix lints using prettier
fmt:
    bunx --bun prettier --write .
