set dotenv-load:=true

build:
    bun astro build

preview:
    bun astro dev

links:
  lychee dist/ -t 40 --max-redirects 10 --exclude-loopback --insecure --exclude-path src/ --cache --max-cache-age 1d

# lint using prettier
lint:
    bun prettier --check .

# fix lints using prettier
fmt:
    bun prettier --write .
