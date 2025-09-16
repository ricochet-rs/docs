set dotenv-load:=true

preview:
    bunx --bun astro dev

links:
  lychee dist/ -t 40 --max-redirects 10 --exclude-loopback --insecure --exclude-path src/ --cache --max-cache-age 1d
