# Design: Latest-version redirects for unversioned URLs

## Problem

Documentation pages live under versioned paths (e.g. `/v0-7/admin/pricing/5-community-edition/`). The unversioned form of the same URL (e.g. `/admin/pricing/5-community-edition`) currently 404s.

External links, SEO-indexed URLs, and people who manually trim version segments end up on dead pages. We want unversioned paths to redirect to the equivalent page under the latest version.

The root URL `/` already redirects to `/v0-7/` via `src/content/docs/index.mdx` (a Markdoc page with `<meta http-equiv="refresh">`). We want the same behaviour for every page under the latest version.

## Goal

For every page that exists under the latest versioned content directory (currently `src/content/docs/v0-7/`), the corresponding unversioned URL should serve a meta-refresh redirect to the versioned URL.

When a new version directory is added (e.g. `v0-8`), the redirects should automatically follow the new latest version with no per-page maintenance.

## Non-goals

- No redirects for `/releases/...` — that content already lives at the docs root, not under a version.
- No redirects for pages that exist only in older versions but were removed from the latest version.
- No HTTP 301/308 redirects — we keep the meta-refresh approach the existing root redirect uses, so the solution remains platform-independent.
- No support for the `dev/` directory as a redirect target.

## Approach

Generate redirect entries at Astro config-load time and pass them to Astro's `redirects` option. In static output mode, Astro emits one HTML file per redirect entry containing a `<meta http-equiv="refresh">` tag.

A small helper module is imported by `astro.config.mjs`. The helper:

1. Scans `src/content/docs/` for directories matching the regex `^v(\d+)-(\d+)$`.
2. Picks the highest by `[major, minor]` numeric tuple. `dev` and any non-conforming names are ignored.
3. Recursively walks that directory for `.md` and `.mdx` files.
4. Builds a `{ source: destination }` map.
5. Skips any source path whose first segment shadows an existing file or directory at the docs root (e.g. `index.mdx`, `releases/`).
6. Returns the map.

If no version directory is found, the helper throws a clear error so the build fails loud rather than silently producing no redirects.

## Path mapping rules

For a file at `src/content/docs/<latest>/<rest>`:

- Regular page: `<latest>/admin/pricing/5-community-edition.mdx` → key `/admin/pricing/5-community-edition`, value `/<latest>/admin/pricing/5-community-edition/`.
- Index page: `<latest>/admin/index.mdx` → key `/admin`, value `/<latest>/admin/`.
- Top-level index: `<latest>/index.mdx` → not emitted (the root `index.mdx` already handles `/`).

Destinations always include a trailing slash to match the existing `index.mdx` convention and Astro's directory-style output. Sources are emitted without a trailing slash; Astro's `trailingSlash: "ignore"` accepts either form.

## Collision handling

Before adding a redirect, the helper checks whether the source's first path segment exists at `src/content/docs/`. If `<top>` exists as a file (e.g. `index.mdx`) or directory (e.g. `releases/`, or another version dir like `v0-6/`), the redirect is skipped.

Concrete cases this rules out today:

- `<latest>/index.mdx` would produce `/` — already handled by `src/content/docs/index.mdx`.
- No conflicts expected for `admin/` or `user/` since those don't exist at the docs root.

## Files

- **New:** `scripts/latest-version-redirects.mjs` — exports `generateLatestVersionRedirects(contentDir)`. Pure function over a directory path, returning an object suitable for Astro's `redirects` config. Easy to unit-test.
- **Modified:** `astro.config.mjs` — imports the helper and adds `redirects: generateLatestVersionRedirects("src/content/docs")` at the top of the `defineConfig({ ... })` call.

## Failure modes

- **No version directories found**: throw `Error("No version directories matching /^v\\d+-\\d+$/ under <contentDir>")`. The build fails immediately.
- **Source path collides with existing root content**: skip silently (this is expected behaviour — the existing root content takes precedence).
- **Empty version directory**: returns an empty redirects map; build succeeds. The helper does not enforce that the latest version has any content.

## Verification

After running `bun astro build`:

1. `dist/admin/pricing/5-community-edition/index.html` exists and contains a meta-refresh to `/v0-7/admin/pricing/5-community-edition/`.
2. `dist/v0-7/admin/pricing/5-community-edition/index.html` still exists (the original page is unaffected).
3. `dist/releases/...` paths are unchanged (no redirect generated).
4. `dist/index.html` still redirects to `/v0-7/` (the existing root redirect is unaffected).
5. After temporarily renaming `src/content/docs/v0-7` to `src/content/docs/v0-8`, redirects point to `/v0-8/...` instead — confirms auto-detection.

## Out of scope for this spec

- HTTP-level redirects (would require hosting-platform configuration or output mode change).
- Redirecting older pages that no longer exist in the latest version to a "this page moved" landing page.
- Sitemap or canonical-URL adjustments.
