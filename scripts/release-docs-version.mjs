// On a new minor release, freeze `latest` as its own version, rebuild `latest`
// from `dev`, then prune old versions. Patch releases change nothing.
//
// Encapsulates the docs-mutation logic the release workflow used to inline as
// shell+sed. The workflow is still responsible for cloning the repo, running
// prettier, and committing/pushing — this script only edits files on disk.
//
// Usage: node scripts/release-docs-version.mjs <tag>
//   <tag> defaults to $CI_COMMIT_TAG (e.g. "v1.2.3" or "1.2.3-beta.1").

import {
  cpSync,
  existsSync,
  readFileSync,
  readdirSync,
  rmSync,
  statSync,
  writeFileSync,
} from "node:fs";
import { fileURLToPath } from "node:url";
import { join } from "node:path";

// How many frozen `v<major>-<minor>` versions to keep alongside `latest`. Older
// ones are pruned during release. `latest` and `dev` are never counted or pruned.
export const KEEP_MINOR_VERSIONS = 4;

const VERSION_REGEX = /^v(\d+)-(\d+)$/;

function escapeRe(s) {
  return s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

export function parseTag(tag) {
  const stripped = tag.replace(/^v/, "");
  const base = stripped.replace(/(-.*|\+.*)$/, "");
  const [major, minor] = base.split(".");
  if (major === undefined || minor === undefined) {
    throw new Error(`Cannot parse major.minor from tag "${tag}"`);
  }
  return {
    slug: `v${major}-${minor}`,
    label: `${major}.${minor}`,
    major: parseInt(major, 10),
    minor: parseInt(minor, 10),
  };
}

export function listVersions(names) {
  const versions = [];
  for (const name of names) {
    const m = name.match(VERSION_REGEX);
    if (!m) continue;
    versions.push({
      slug: name,
      major: parseInt(m[1], 10),
      minor: parseInt(m[2], 10),
    });
  }
  versions.sort((a, b) => b.major - a.major || b.minor - a.minor);
  return versions;
}

// Slugs to remove: everything beyond the newest `keep` minor versions. `dev`
// and any non-versioned directory are ignored, so they are never returned.
export function versionsToPrune(names, keep = KEEP_MINOR_VERSIONS) {
  return listVersions(names)
    .slice(keep)
    .map((v) => v.slug);
}

export function setCurrentLabel(src, label) {
  return src.replace(
    /(current:\s*\{\s*label:\s*")[0-9]+\.[0-9]+(\s*\(latest\)")/,
    `$1${label}$2`,
  );
}

const LATEST_ENTRY = /\{\s*slug:\s*"latest",\s*label:\s*"[^"]*"\s*\},?/;

// Insert a frozen version right after `latest`, so the switcher lists newest first.
export function insertVersionAfterLatest(src, slug, label) {
  if (new RegExp(`slug:\\s*"${escapeRe(slug)}"`).test(src)) return src;
  return src.replace(
    LATEST_ENTRY,
    (entry) =>
      `${entry.replace(/,$/, "")}, { slug: "${slug}", label: "${label}" },`,
  );
}

// Point the `latest` entry and the redirect integration at the release `latest/` now holds.
export function setLatestRelease(src, slug, label) {
  return src
    .replace(/(\{\s*slug:\s*"latest",\s*label:\s*")[^"]*(")/, `$1${label}$2`)
    .replace(
      /(latestVersionRedirectsIntegration\(\s*"[^"]*",\s*")[^"]*(")/,
      `$1${slug}$2`,
    );
}

export function removeVersionEntries(src, slugs) {
  let out = src;
  for (const slug of slugs) {
    const re = new RegExp(
      `\\s*\\{\\s*slug:\\s*"${escapeRe(slug)}",\\s*label:\\s*"[^"]*"\\s*\\},?`,
      "g",
    );
    out = out.replace(re, "");
  }
  return out;
}

export function addLycheeExclude(src, slug) {
  const pattern = `https://docs.ricochet.rs/${slug}/.*`;
  if (src.includes(pattern)) return src;
  return src.replace(/^\]$/m, `    "${pattern}",\n]`);
}

export function removeLycheeExcludes(src, slugs) {
  let out = src;
  for (const slug of slugs) {
    const re = new RegExp(
      `^.*https://docs\\.ricochet\\.rs/${escapeRe(slug)}/.*$\\n?`,
      "m",
    );
    out = out.replace(re, "");
  }
  return out;
}

// Rewrite site-root `/<from>/...` links to `/<to>/...`. A copied version is
// littered with absolute links to its source that must point at the copy
// itself. Paths inside external URLs, such as `.../docs/latest/`, are left alone.
export function rewriteVersionLinks(src, from, to) {
  return src.replace(
    new RegExp(`(?<![\\w.-])/${escapeRe(from)}/`, "g"),
    `/${to}/`,
  );
}

function walkFiles(dir) {
  const out = [];
  for (const name of readdirSync(dir)) {
    const path = join(dir, name);
    if (statSync(path).isDirectory()) out.push(...walkFiles(path));
    else out.push(path);
  }
  return out;
}

// Copy a version's content and sidebar, then repoint the copy's internal links.
function copyVersion(root, from, to) {
  const docsDir = join(root, "src/content/docs");
  const versionsDir = join(root, "src/content/versions");
  rmSync(join(docsDir, to), { recursive: true, force: true });
  cpSync(join(docsDir, from), join(docsDir, to), { recursive: true });
  cpSync(join(versionsDir, `${from}.json`), join(versionsDir, `${to}.json`));
  for (const file of walkFiles(join(docsDir, to))) {
    if (!/\.mdx?$/.test(file)) continue;
    edit(file, (src) => rewriteVersionLinks(src, from, to));
  }
}

function readCurrentLabel(src) {
  const m = src.match(/current:\s*\{\s*label:\s*"([0-9]+\.[0-9]+)/);
  return m ? m[1] : null;
}

// Edit a file in place via a (string) -> string transform.
function edit(path, fn) {
  if (!existsSync(path)) return;
  writeFileSync(path, fn(readFileSync(path, "utf8")));
}

export function pruneVersions(root, slugs) {
  if (slugs.length === 0) return;
  const docsDir = join(root, "src/content/docs");
  const versionsDir = join(root, "src/content/versions");
  for (const slug of slugs) {
    rmSync(join(docsDir, slug), { recursive: true, force: true });
    rmSync(join(versionsDir, `${slug}.json`), { force: true });
  }
  edit(join(root, "astro.config.mjs"), (s) => removeVersionEntries(s, slugs));
  edit(join(root, "lychee.toml"), (s) => removeLycheeExcludes(s, slugs));
}

export function releaseDocsVersion(
  root,
  tag,
  { keep = KEEP_MINOR_VERSIONS } = {},
) {
  const { slug, label } = parseTag(tag);
  const docsDir = join(root, "src/content/docs");
  const astroConfig = join(root, "astro.config.mjs");
  const latestLabel = readCurrentLabel(readFileSync(astroConfig, "utf8"));
  if (!latestLabel) {
    throw new Error(`No "<major>.<minor> (latest)" label in ${astroConfig}`);
  }

  if (latestLabel === label || existsSync(join(docsDir, slug))) {
    console.log(
      `Version ${slug} already exists — patch release, no docs changes.`,
    );
    return { created: false, slug, label, pruned: [] };
  }

  const frozenSlug = `v${latestLabel.replace(".", "-")}`;
  console.log(
    `Freezing latest as ${frozenSlug} and rebuilding latest from dev...`,
  );
  copyVersion(root, "latest", frozenSlug);
  copyVersion(root, "dev", "latest");

  edit(astroConfig, (src) => {
    let out = insertVersionAfterLatest(src, frozenSlug, latestLabel);
    out = setCurrentLabel(out, label);
    return setLatestRelease(out, slug, label);
  });

  edit(join(root, "lychee.toml"), (s) => addLycheeExclude(s, frozenSlug));

  const pruned = versionsToPrune(readdirSync(docsDir), keep);
  if (pruned.length) {
    console.log(
      `Pruning old versions (keeping newest ${keep}): ${pruned.join(", ")}`,
    );
    pruneVersions(root, pruned);
  }

  return { created: true, slug, label, pruned };
}

function main() {
  const tag = process.argv[2] ?? process.env.CI_COMMIT_TAG;
  if (!tag) {
    console.error(
      "Usage: node scripts/release-docs-version.mjs <tag> (or set CI_COMMIT_TAG)",
    );
    process.exit(1);
  }
  releaseDocsVersion(process.cwd(), tag);
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  main();
}
