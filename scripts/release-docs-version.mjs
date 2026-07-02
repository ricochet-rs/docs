// Create (or patch) a docs version during a release, then prune old versions.
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

// How many `v<major>-<minor>` versions to keep. Older ones are pruned during
// release. The `dev` version is never counted or pruned.
export const KEEP_MINOR_VERSIONS = 5;

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

export function prependVersionEntry(src, slug, label) {
  if (new RegExp(`slug:\\s*"${escapeRe(slug)}"`).test(src)) return src;
  const entry = `{ slug: "${slug}", label: "${label}" }`;
  if (/versions:\s*\[\s*\]/.test(src)) {
    return src.replace(/versions:\s*\[\s*\]/, `versions: [${entry}]`);
  }
  return src.replace(/(versions:\s*\[)/, `$1${entry}, `);
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

export function updateRootRedirect(src, slug) {
  return src
    .replace(/url=\/v[0-9]+-[0-9]+\//g, `url=/${slug}/`)
    .replace(/\/v[0-9]+-[0-9]+\//g, `/${slug}/`);
}

export function updateVersionIndex(src, slug) {
  return src
    .replace(/slug:\s*"(?:dev|v[0-9]+-[0-9]+)"/, `slug: "${slug}"`)
    .replace(/\/(?:dev|v[0-9]+-[0-9]+)\//g, `/${slug}/`);
}

// Rewrite absolute `/dev/...` links to `/<slug>/...`. A version is copied from
// `dev`, so its content is littered with absolute `/dev/` links that must point
// at the version itself — otherwise readers of a released version get sent into
// the dev docs, and the links break the moment dev is restructured.
export function rewriteDevLinks(src, slug) {
  return src.replace(/\/dev\//g, `/${slug}/`);
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

// Apply rewriteDevLinks to every markdown file under a freshly-copied version.
export function rewriteVersionLinks(versionDir, slug) {
  for (const file of walkFiles(versionDir)) {
    if (!/\.mdx?$/.test(file)) continue;
    edit(file, (src) => rewriteDevLinks(src, slug));
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
  const versionsDir = join(root, "src/content/versions");
  const versionConfig = join(versionsDir, `${slug}.json`);
  const versionDir = join(docsDir, slug);

  if (existsSync(versionConfig) || existsSync(versionDir)) {
    console.log(
      `Version ${slug} already exists — patch release, no docs changes.`,
    );
    return { created: false, slug, label, pruned: [] };
  }

  console.log(`Creating new docs version ${slug} from dev...`);
  cpSync(join(versionsDir, "dev.json"), versionConfig);
  cpSync(join(docsDir, "dev"), versionDir, { recursive: true });

  edit(join(root, "astro.config.mjs"), (src) => {
    let out = src;
    const oldLabel = readCurrentLabel(out);
    if (oldLabel) {
      const oldSlug = `v${oldLabel.replace(".", "-")}`;
      out = prependVersionEntry(out, oldSlug, oldLabel);
    }
    out = setCurrentLabel(out, label);
    out = prependVersionEntry(out, slug, label);
    return out;
  });

  edit(join(docsDir, "index.mdx"), (s) => updateRootRedirect(s, slug));
  edit(join(versionDir, "index.mdx"), (s) => updateVersionIndex(s, slug));
  rewriteVersionLinks(versionDir, slug);
  edit(join(root, "lychee.toml"), (s) => addLycheeExclude(s, slug));

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
