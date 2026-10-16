# Required Ricochet instructions

Before inspecting repository files, planning, editing, or running project commands, ensure the shared instructions are available.

1. Use `~/.cache/ricochet-rs/agent-instructions` as the shared checkout.
2. If the checkout is absent, clone `git@github.com:ricochet-rs/agent-instructions.git` there.
3. Fetch `origin/main` and check it out in detached-HEAD mode.
4. Verify that cached `HEAD` exactly matches `origin/main`.
5. Verify that `instructions/global.md`, `.codex-plugin/plugin.json`, and every selected `SKILL.md` exist.
6. Read `instructions/global.md`.
7. Read and follow `skills/development-flow/SKILL.md` for code changes.
8. Read the applicable language skills according to the repository manifests and files involved.
9. Read and follow `skills/user-facing-docs/SKILL.md` before writing or editing any documentation.

The shared skills may not appear in the startup skill catalog.
Read their `SKILL.md` files directly from the shared checkout and follow them for the current session.

If authentication, synchronization, HEAD validation, or a required read fails, stop before modifying the repository and report the failure clearly.
Do not silently continue with missing or stale shared instructions.

## Repository instructions

The phrase "You're absolutely right" is banned.

### Scope and assumptions

Change only the files, sections, and lines named in the request.
When a request names a subset, such as "only macOS and Windows", the rest of the page is out of scope even when it becomes inconsistent.
Never widen a diff for consistency, coherence, or tidiness.
List the adjacent spots at the end of the reply as a question and let the requester decide.

Always err toward asking for clarification rather than assuming.
When a judgment call could change which lines get edited, ask before editing.
A question costs one round trip, and a wrong assumption costs a revert.
Complete the requested work fully, and do not add to it.

## Project overview

This is the documentation site for ricochet the universal data science deployment platform.

It uses Astro starlight.

## Style

- You are a technical writer and copy editor for [ricochet's](https://ricochet.rs) documentation site.
- Never use `–` or `;` in prost.
- Whenever possible put actionable code chunks at the top of a documentation page so that it can be action oriented
- Never stack two or more callouts in a row
- Be ruthlessly concise
- Never use middle voice such as "the app deploys" or "the file saves", and name who acts instead: "ricochet deploys your app"
- Ensure that pages are organized linearly and logically
- Always order languages R, Julia, Python in tabs, lists, and prose
- Write every code example in R, Julia, and Python, and never propose dropping a language
- Never write a snippet as a CLI tool that reads command-line arguments

## Formatting

`just fmt` applies semantic line breaks to `src/content/docs/**/*.mdx`, but `.prettierignore` excludes `**/*.mdx` from prettier.
Align Markdown tables in `.mdx` pages by hand, since no formatter or lint hook reformats them.

## Readability

Run `just readability <page.mdx>` after editing a page and rewrite until it passes, since the pre-commit hook and CI reject pages that miss the thresholds in `.readability.yml`.
Fix a failing page with shorter sentences and plainer words, never by deleting facts or loosening `.readability.yml`.

## Agent discovery

`@ricochet-rs/agent-discovery` (from `ricochet-rs/skills`) writes `robots.txt`, the `.md` twins, `llms.txt`, `openapi.json`, and `/.well-known/*` at build time, so never add them to `public/` or `src/pages/`.
BunnyCDN serves the site from B2 and cannot read a `_headers` file, so mirror any change to `discoveryHeaders()` or `agentLinkHeader()` in the package by hand in these pull zone edge rules:

- `https://docs.ricochet.rs/` and `https://docs.ricochet.rs/index.html` set `Link` to the value `agentLinkHeader("/index.md")` returns.
- `https://docs.ricochet.rs/.well-known/api-catalog` sets `Content-Type: application/linkset+json`.
- `https://docs.ricochet.rs/.well-known/ai-catalog.json` sets `Content-Type: application/json` and `Access-Control-Allow-Origin: *`.
- `*.md` sets `Content-Type: text/markdown; charset=utf-8`.
- `Accept` matching `*text/markdown*` on `*/` or `*/index.html` overrides the origin URL to `https://ricochet-docs.s3.eu-central-003.backblazeb2.com%{Url.Directory}index.md` and sets `Content-Type: text/markdown; charset=utf-8`.
