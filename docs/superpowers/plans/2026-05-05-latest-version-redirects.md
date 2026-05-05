# Latest-Version Redirects Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Generate static meta-refresh redirects from unversioned doc URLs (e.g. `/admin/pricing/5-community-edition`) to the equivalent path under the auto-detected latest version (e.g. `/v0-7/admin/pricing/5-community-edition/`), so external links don't 404.

**Architecture:** A small ESM helper, `scripts/latest-version-redirects.mjs`, walks `src/content/docs/<latest>/` at Astro config-load time and returns a `{ source: destination }` map. The helper is wired into `astro.config.mjs` via the top-level `redirects` option. Astro's static output mode emits one HTML file per redirect entry containing `<meta http-equiv="refresh">` — same mechanism the existing root `src/content/docs/index.mdx` uses. No new runtime, no new hosting config.

**Tech Stack:** Node ESM (`.mjs`), `node:fs`/`node:path` standard library, Bun's built-in test runner (`bun:test`, no new dependencies).

**Spec:** `docs/superpowers/specs/2026-05-05-latest-version-redirects-design.md`

---

## File Structure

- **Create:** `scripts/latest-version-redirects.mjs` — exports `findLatestVersion(contentDir)` and `generateLatestVersionRedirects(contentDir)`. Pure-ish (filesystem reads only, deterministic given fs state). One responsibility: produce the redirect map.
- **Create:** `scripts/latest-version-redirects.test.mjs` — Bun test suite using `os.tmpdir()` fixtures so tests are isolated from real `src/content/docs`.
- **Modify:** `astro.config.mjs` — import the helper and add `redirects: generateLatestVersionRedirects("src/content/docs")` to the top-level `defineConfig` object.

---

## Task 1: `findLatestVersion` helper

Picks the highest `v<major>-<minor>` directory by numeric tuple. Throws if none exist.

**Files:**

- Create: `scripts/latest-version-redirects.mjs`
- Create: `scripts/latest-version-redirects.test.mjs`

- [ ] **Step 1: Write the failing tests**

Create `scripts/latest-version-redirects.test.mjs`:

```js
import { test, expect } from "bun:test";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { findLatestVersion } from "./latest-version-redirects.mjs";

function makeFixture(layout) {
  const root = mkdtempSync(join(tmpdir(), "lvr-"));
  for (const path of layout.dirs ?? []) {
    mkdirSync(join(root, path), { recursive: true });
  }
  for (const [path, content] of Object.entries(layout.files ?? {})) {
    const full = join(root, path);
    mkdirSync(join(full, ".."), { recursive: true });
    writeFileSync(full, content);
  }
  return {
    root,
    cleanup: () => rmSync(root, { recursive: true, force: true }),
  };
}

test("findLatestVersion picks highest v<major>-<minor>", () => {
  const { root, cleanup } = makeFixture({
    dirs: ["v0-1", "v0-2", "v0-7", "v0-6"],
  });
  try {
    expect(findLatestVersion(root)).toBe("v0-7");
  } finally {
    cleanup();
  }
});

test("findLatestVersion sorts numerically, not lexically", () => {
  const { root, cleanup } = makeFixture({
    dirs: ["v0-2", "v0-10"],
  });
  try {
    expect(findLatestVersion(root)).toBe("v0-10");
  } finally {
    cleanup();
  }
});

test("findLatestVersion compares major before minor", () => {
  const { root, cleanup } = makeFixture({
    dirs: ["v0-9", "v1-0"],
  });
  try {
    expect(findLatestVersion(root)).toBe("v1-0");
  } finally {
    cleanup();
  }
});

test("findLatestVersion ignores non-version dirs and files", () => {
  const { root, cleanup } = makeFixture({
    dirs: ["v0-1", "v0-2", "dev", "releases"],
    files: { "index.mdx": "" },
  });
  try {
    expect(findLatestVersion(root)).toBe("v0-2");
  } finally {
    cleanup();
  }
});

test("findLatestVersion throws when no version dirs exist", () => {
  const { root, cleanup } = makeFixture({
    dirs: ["dev", "releases"],
    files: { "index.mdx": "" },
  });
  try {
    expect(() => findLatestVersion(root)).toThrow(/No version directories/);
  } finally {
    cleanup();
  }
});
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `bun test scripts/latest-version-redirects.test.mjs`
Expected: All 5 tests fail with `Cannot find module './latest-version-redirects.mjs'` or similar.

- [ ] **Step 3: Implement `findLatestVersion`**

Create `scripts/latest-version-redirects.mjs`:

```js
import { readdirSync } from "node:fs";

const VERSION_REGEX = /^v(\d+)-(\d+)$/;

export function findLatestVersion(contentDir) {
  const entries = readdirSync(contentDir, { withFileTypes: true });
  const versions = [];
  for (const entry of entries) {
    if (!entry.isDirectory()) continue;
    const match = entry.name.match(VERSION_REGEX);
    if (!match) continue;
    versions.push({
      name: entry.name,
      major: parseInt(match[1], 10),
      minor: parseInt(match[2], 10),
    });
  }
  if (versions.length === 0) {
    throw new Error(
      `No version directories matching /^v\\d+-\\d+$/ under ${contentDir}`,
    );
  }
  versions.sort((a, b) => b.major - a.major || b.minor - a.minor);
  return versions[0].name;
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `bun test scripts/latest-version-redirects.test.mjs`
Expected: All 5 tests pass.

- [ ] **Step 5: Commit**

```bash
git add scripts/latest-version-redirects.mjs scripts/latest-version-redirects.test.mjs
git commit -m "feat(redirects): add findLatestVersion helper

Picks the highest v<major>-<minor> directory under src/content/docs
by numeric tuple. Foundation for generating unversioned-URL redirects
to the latest version."
```

---

## Task 2: Page walking and basic redirect map

Walk `<latest>/` recursively, emit `{ '/foo': '/<latest>/foo/' }` for each `.md`/`.mdx` page. Handle index files.

**Files:**

- Modify: `scripts/latest-version-redirects.mjs`
- Modify: `scripts/latest-version-redirects.test.mjs`

- [ ] **Step 1: Write the failing tests**

Append to `scripts/latest-version-redirects.test.mjs`:

```js
import { generateLatestVersionRedirects } from "./latest-version-redirects.mjs";

test("generates redirect for a regular page", () => {
  const { root, cleanup } = makeFixture({
    files: { "v0-7/admin/pricing/5-community-edition.mdx": "" },
  });
  try {
    expect(generateLatestVersionRedirects(root)).toEqual({
      "/admin/pricing/5-community-edition":
        "/v0-7/admin/pricing/5-community-edition/",
    });
  } finally {
    cleanup();
  }
});

test("walks nested directories", () => {
  const { root, cleanup } = makeFixture({
    files: {
      "v0-7/admin/foo.mdx": "",
      "v0-7/admin/bar/baz.mdx": "",
      "v0-7/user/quickstart.md": "",
    },
  });
  try {
    expect(generateLatestVersionRedirects(root)).toEqual({
      "/admin/foo": "/v0-7/admin/foo/",
      "/admin/bar/baz": "/v0-7/admin/bar/baz/",
      "/user/quickstart": "/v0-7/user/quickstart/",
    });
  } finally {
    cleanup();
  }
});

test("maps section index to its parent path", () => {
  const { root, cleanup } = makeFixture({
    files: { "v0-7/admin/index.mdx": "" },
  });
  try {
    expect(generateLatestVersionRedirects(root)).toEqual({
      "/admin": "/v0-7/admin/",
    });
  } finally {
    cleanup();
  }
});

test("skips top-level <latest>/index.mdx", () => {
  const { root, cleanup } = makeFixture({
    files: {
      "v0-7/index.mdx": "",
      "v0-7/admin/foo.mdx": "",
    },
  });
  try {
    expect(generateLatestVersionRedirects(root)).toEqual({
      "/admin/foo": "/v0-7/admin/foo/",
    });
  } finally {
    cleanup();
  }
});

test("ignores non-markdown files in version dir", () => {
  const { root, cleanup } = makeFixture({
    files: {
      "v0-7/admin/foo.mdx": "",
      "v0-7/admin/data.json": "",
      "v0-7/admin/image.png": "",
    },
  });
  try {
    expect(generateLatestVersionRedirects(root)).toEqual({
      "/admin/foo": "/v0-7/admin/foo/",
    });
  } finally {
    cleanup();
  }
});
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `bun test scripts/latest-version-redirects.test.mjs`
Expected: 5 new tests fail (`generateLatestVersionRedirects` not exported); 5 existing `findLatestVersion` tests still pass.

- [ ] **Step 3: Implement `generateLatestVersionRedirects`**

Append to `scripts/latest-version-redirects.mjs`:

```js
import { join, relative, sep } from "node:path";

function* walkPages(dir, baseDir = dir) {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const fullPath = join(dir, entry.name);
    if (entry.isDirectory()) {
      yield* walkPages(fullPath, baseDir);
    } else if (entry.isFile() && /\.mdx?$/.test(entry.name)) {
      yield relative(baseDir, fullPath).split(sep).join("/");
    }
  }
}

export function generateLatestVersionRedirects(contentDir) {
  const latest = findLatestVersion(contentDir);
  const versionDir = join(contentDir, latest);
  const redirects = {};
  for (const rel of walkPages(versionDir)) {
    const noExt = rel.replace(/\.mdx?$/, "");
    if (noExt === "index") continue;
    const path = noExt.endsWith("/index")
      ? noExt.slice(0, -"/index".length)
      : noExt;
    redirects[`/${path}`] = `/${latest}/${path}/`;
  }
  return redirects;
}
```

Also update the existing `import { readdirSync } from "node:fs";` line at the top of the file to:

```js
import { readdirSync } from "node:fs";
import { join, relative, sep } from "node:path";
```

(If the `node:path` import was added inline in this step, ensure imports are grouped at the top of the file and there are no duplicates.)

- [ ] **Step 4: Run tests to verify they pass**

Run: `bun test scripts/latest-version-redirects.test.mjs`
Expected: All 10 tests pass.

- [ ] **Step 5: Commit**

```bash
git add scripts/latest-version-redirects.mjs scripts/latest-version-redirects.test.mjs
git commit -m "feat(redirects): generate redirect map from latest version pages

Walks the latest version directory and emits one entry per .md/.mdx
page mapping the unversioned path to the versioned path. Handles
section-index files and skips the top-level index (root already
redirects)."
```

---

## Task 3: Skip paths that collide with root content

If a generated redirect's first path segment shadows an existing file or directory at `contentDir`, skip it. Prevents conflicts with `releases/`, `index.mdx`, and any future top-level content.

**Files:**

- Modify: `scripts/latest-version-redirects.mjs`
- Modify: `scripts/latest-version-redirects.test.mjs`

- [ ] **Step 1: Write the failing tests**

Append to `scripts/latest-version-redirects.test.mjs`:

```js
test("skips redirects whose top segment shadows a root directory", () => {
  const { root, cleanup } = makeFixture({
    dirs: ["releases"],
    files: {
      "v0-7/admin/foo.mdx": "",
      "v0-7/releases/0.7.0.mdx": "",
    },
  });
  try {
    expect(generateLatestVersionRedirects(root)).toEqual({
      "/admin/foo": "/v0-7/admin/foo/",
    });
  } finally {
    cleanup();
  }
});

test("skips redirects whose top segment shadows a root file", () => {
  const { root, cleanup } = makeFixture({
    files: {
      "shadowed.mdx": "",
      "v0-7/admin/foo.mdx": "",
      "v0-7/shadowed.mdx": "",
    },
  });
  try {
    expect(generateLatestVersionRedirects(root)).toEqual({
      "/admin/foo": "/v0-7/admin/foo/",
    });
  } finally {
    cleanup();
  }
});
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `bun test scripts/latest-version-redirects.test.mjs`
Expected: 2 new tests fail (collision shadowing produces unexpected extra entries); 10 prior tests still pass.

- [ ] **Step 3: Implement collision skipping**

Modify `generateLatestVersionRedirects` in `scripts/latest-version-redirects.mjs`. First update the imports to add `existsSync`:

```js
import { existsSync, readdirSync } from "node:fs";
```

Then update the function body to check for collisions before adding each entry:

```js
export function generateLatestVersionRedirects(contentDir) {
  const latest = findLatestVersion(contentDir);
  const versionDir = join(contentDir, latest);
  const redirects = {};
  for (const rel of walkPages(versionDir)) {
    const noExt = rel.replace(/\.mdx?$/, "");
    if (noExt === "index") continue;
    const path = noExt.endsWith("/index")
      ? noExt.slice(0, -"/index".length)
      : noExt;
    const topSegment = path.split("/")[0];
    if (
      existsSync(join(contentDir, topSegment)) ||
      existsSync(join(contentDir, `${topSegment}.mdx`)) ||
      existsSync(join(contentDir, `${topSegment}.md`))
    ) {
      continue;
    }
    redirects[`/${path}`] = `/${latest}/${path}/`;
  }
  return redirects;
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `bun test scripts/latest-version-redirects.test.mjs`
Expected: All 12 tests pass.

- [ ] **Step 5: Commit**

```bash
git add scripts/latest-version-redirects.mjs scripts/latest-version-redirects.test.mjs
git commit -m "feat(redirects): skip redirects that would shadow root content

Prevents collisions with files/dirs that already exist directly under
src/content/docs (e.g. releases/, index.mdx). Generated redirects only
fill in unversioned URLs that would otherwise 404."
```

---

## Task 4: Wire into Astro config and verify build

Hook the helper up to `astro.config.mjs` and confirm the produced `dist/` contains the expected redirect HTML files.

**Files:**

- Modify: `astro.config.mjs`

- [ ] **Step 1: Sanity-check what the helper produces against real content**

Run a one-shot script to print the generated redirects for the real `src/content/docs`:

```bash
bun -e 'import("./scripts/latest-version-redirects.mjs").then(m => console.log(JSON.stringify(m.generateLatestVersionRedirects("src/content/docs"), null, 2)))'
```

Expected output: a JSON object with keys like `/admin/...` and `/user/...`, values like `/v0-7/admin/.../`. Spot-check that `/admin/pricing/5-community-edition` is present and maps to `/v0-7/admin/pricing/5-community-edition/`. No keys should start with `/releases/` or be a bare `/`.

- [ ] **Step 2: Modify `astro.config.mjs`**

Add an import near the top of `astro.config.mjs` (after the other imports):

```js
import { generateLatestVersionRedirects } from "./scripts/latest-version-redirects.mjs";
```

Add the `redirects` option to the top-level `defineConfig` object. The current config starts with:

```js
export default defineConfig({
  site: "https://docs.ricochet.rs",
  output: "static",
  trailingSlash: "ignore",
  integrations: [
```

Change it to:

```js
export default defineConfig({
  site: "https://docs.ricochet.rs",
  output: "static",
  trailingSlash: "ignore",
  redirects: generateLatestVersionRedirects("src/content/docs"),
  integrations: [
```

- [ ] **Step 3: Run the build**

Run: `bun astro build`
Expected: build completes without errors. Console output mentions generating redirect routes (Astro logs each route).

- [ ] **Step 4: Verify the redirect HTML files exist and target the right URLs**

Run:

```bash
test -f dist/admin/pricing/5-community-edition/index.html && \
  grep -F "/v0-7/admin/pricing/5-community-edition/" dist/admin/pricing/5-community-edition/index.html
```

Expected: command succeeds and prints a line containing the meta-refresh tag.

Also confirm the original versioned page is still built:

```bash
test -f dist/v0-7/admin/pricing/5-community-edition/index.html && echo "versioned page still present"
```

Expected: prints `versioned page still present`.

Confirm `releases/` is unaffected (should not have been re-redirected to a version):

```bash
ls dist/releases/ | head -5
```

Expected: real release pages, not redirect stubs.

Confirm the existing root redirect is unaffected:

```bash
grep -F "url=/v0-7/" dist/index.html
```

Expected: prints the existing meta-refresh line.

- [ ] **Step 5: Commit**

```bash
git add astro.config.mjs
git commit -m "feat(redirects): wire latest-version redirects into Astro config

Unversioned URLs like /admin/pricing/5-community-edition now redirect
to the auto-detected latest version (/v0-7/...) via meta-refresh,
matching the existing root index.mdx pattern. External links no
longer 404."
```

---

## Self-Review

**Spec coverage:**

- "Scans for `^v(\d+)-(\d+)$` and picks highest" → Task 1
- "Walks for `.md`/`.mdx`, builds map" → Task 2
- "Index page mapping" → Task 2
- "Top-level `<latest>/index.mdx` skipped" → Task 2
- "Collision handling" → Task 3
- "Throws clear error when no version dir found" → Task 1
- "Wired into `astro.config.mjs` `redirects`" → Task 4
- "Verification: dist files, original page intact, releases unaffected, root redirect intact" → Task 4 Step 4

All spec sections covered.

**Placeholder scan:** No "TBD"/"TODO"/"add appropriate"/etc. All test code, implementation code, and shell commands are concrete.

**Type/name consistency:** `findLatestVersion`, `generateLatestVersionRedirects`, `walkPages`, `VERSION_REGEX` used consistently across tasks. The `path` local variable in Task 2 and Task 3 refers to the same thing.

**Scope:** One feature, four tasks, ~15-25 minutes of work. Right-sized.
