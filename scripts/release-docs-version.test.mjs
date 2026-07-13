import { test, expect } from "bun:test";
import {
  mkdtempSync,
  mkdirSync,
  writeFileSync,
  readFileSync,
  existsSync,
  rmSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import {
  parseTag,
  listVersions,
  versionsToPrune,
  setCurrentLabel,
  prependVersionEntry,
  removeVersionEntries,
  addLycheeExclude,
  removeLycheeExcludes,
  updateRootRedirect,
  updateVersionIndex,
  rewriteDevLinks,
  releaseDocsVersion,
} from "./release-docs-version.mjs";

test("parseTag handles plain, v-prefixed and pre-release tags", () => {
  expect(parseTag("v1.2.3")).toMatchObject({ slug: "v1-2", label: "1.2" });
  expect(parseTag("0.10.0")).toMatchObject({ slug: "v0-10", label: "0.10" });
  expect(parseTag("1.2.3-beta.1")).toMatchObject({
    slug: "v1-2",
    label: "1.2",
  });
  expect(parseTag("2.0.0+build.5")).toMatchObject({
    slug: "v2-0",
    label: "2.0",
  });
});

test("listVersions sorts descending and ignores non-version names", () => {
  const v = listVersions(["v0-2", "dev", "v0-10", "releases", "v0-9"]);
  expect(v.map((x) => x.slug)).toEqual(["v0-10", "v0-9", "v0-2"]);
});

test("versionsToPrune keeps newest N and never returns dev", () => {
  const names = [
    "dev",
    "v0-1",
    "v0-2",
    "v0-6",
    "v0-7",
    "v0-8",
    "v0-9",
    "v0-10",
  ];
  expect(versionsToPrune(names, 5)).toEqual(["v0-2", "v0-1"]);
  expect(versionsToPrune(["dev", "v0-1", "v0-2"], 5)).toEqual([]);
});

test("setCurrentLabel swaps only the current label", () => {
  const src = `current: { label: "0.9 (latest)", redirect: "root" }`;
  expect(setCurrentLabel(src, "0.10")).toBe(
    `current: { label: "0.10 (latest)", redirect: "root" }`,
  );
});

test("prependVersionEntry adds, fills empty array, and is idempotent", () => {
  expect(
    prependVersionEntry(
      `versions: [{ slug: "v0-9", label: "0.9" }]`,
      "v0-10",
      "0.10",
    ),
  ).toBe(
    `versions: [{ slug: "v0-10", label: "0.10" }, { slug: "v0-9", label: "0.9" }]`,
  );
  expect(prependVersionEntry(`versions: []`, "v0-10", "0.10")).toBe(
    `versions: [{ slug: "v0-10", label: "0.10" }]`,
  );
  const existing = `versions: [{ slug: "v0-10", label: "0.10" }]`;
  expect(prependVersionEntry(existing, "v0-10", "0.10")).toBe(existing);
});

test("removeVersionEntries drops matching entries, multiline-safe", () => {
  const src = [
    "versions: [",
    '  { slug: "v0-10", label: "0.10" },',
    '  { slug: "v0-2", label: "0.2" },',
    '  { slug: "dev", label: "dev" },',
    '  { slug: "v0-1", label: "0.1" },',
    "],",
  ].join("\n");
  const out = removeVersionEntries(src, ["v0-1", "v0-2"]);
  expect(out).not.toContain('slug: "v0-1"');
  expect(out).not.toContain('slug: "v0-2"');
  expect(out).toContain('slug: "v0-10"');
  expect(out).toContain('slug: "dev"');
});

test("lychee exclude add is idempotent and remove is targeted", () => {
  const base = [
    "exclude = [",
    '    "https://docs.ricochet.rs/v0-9/.*",',
    "]",
  ].join("\n");
  const added = addLycheeExclude(base, "v0-10");
  expect(added).toContain('"https://docs.ricochet.rs/v0-10/.*"');
  expect(addLycheeExclude(added, "v0-10")).toBe(added);
  const removed = removeLycheeExcludes(added, ["v0-9"]);
  expect(removed).not.toContain("v0-9");
  expect(removed).toContain("v0-10");
});

test("updateRootRedirect rewrites both the meta refresh and the link", () => {
  const src = 'content: "0; url=/v0-9/"\nRedirecting to [docs](/v0-9/)...';
  const out = updateRootRedirect(src, "v0-10");
  expect(out).toBe(
    'content: "0; url=/v0-10/"\nRedirecting to [docs](/v0-10/)...',
  );
});

test("updateVersionIndex rewrites slug and paths copied from dev", () => {
  const src = 'slug: "dev"\nsee [x](/dev/user/quickstart/)';
  expect(updateVersionIndex(src, "v0-10")).toBe(
    'slug: "v0-10"\nsee [x](/v0-10/user/quickstart/)',
  );
});

test("rewriteDevLinks repoints absolute /dev/ links, leaving others alone", () => {
  const src =
    'href="/dev/admin/configuration/otel/"\nsee [x](/dev/user/quickstart/)\nkeep /v0-9/user/ and the word dev';
  expect(rewriteDevLinks(src, "v0-10")).toBe(
    'href="/v0-10/admin/configuration/otel/"\nsee [x](/v0-10/user/quickstart/)\nkeep /v0-9/user/ and the word dev',
  );
});

function makeRepo() {
  const root = mkdtempSync(join(tmpdir(), "rel-"));
  const docs = join(root, "src/content/docs");
  const versions = join(root, "src/content/versions");
  mkdirSync(join(docs, "dev"), { recursive: true });
  mkdirSync(versions, { recursive: true });
  writeFileSync(join(versions, "dev.json"), '{ "sidebar": [] }\n');
  writeFileSync(join(docs, "dev", "index.mdx"), 'slug: "dev"\nlink /dev/x/\n');
  mkdirSync(join(docs, "dev", "admin"), { recursive: true });
  writeFileSync(
    join(docs, "dev", "admin", "page.mdx"),
    "See [otel](/dev/admin/otel/) for details.\n",
  );
  for (let minor = 1; minor <= 9; minor++) {
    const slug = `v0-${minor}`;
    mkdirSync(join(docs, slug), { recursive: true });
    writeFileSync(join(docs, slug, "index.mdx"), `slug: "${slug}"\n`);
    writeFileSync(join(versions, `${slug}.json`), '{ "sidebar": [] }\n');
  }
  writeFileSync(
    join(docs, "index.mdx"),
    'content: "0; url=/v0-9/"\n[docs](/v0-9/)\n',
  );
  const versionsLine = Array.from(
    { length: 9 },
    (_, i) => `{ slug: "v0-${9 - i}", label: "0.${9 - i}" }`,
  ).join(", ");
  writeFileSync(
    join(root, "astro.config.mjs"),
    `current: { label: "0.9 (latest)", redirect: "root" },\nversions: [${versionsLine}, { slug: "dev", label: "dev" }],\n`,
  );
  writeFileSync(
    join(root, "lychee.toml"),
    "exclude = [\n" +
      Array.from(
        { length: 9 },
        (_, i) => `    "https://docs.ricochet.rs/v0-${i + 1}/.*",`,
      ).join("\n") +
      "\n]\n",
  );
  return {
    root,
    docs,
    versions,
    cleanup: () => rmSync(root, { recursive: true, force: true }),
  };
}

test("releaseDocsVersion creates v0-10 and prunes down to newest 5 + dev", () => {
  const { root, docs, versions, cleanup } = makeRepo();
  try {
    const result = releaseDocsVersion(root, "v0.10.0");
    expect(result.created).toBe(true);
    expect(result.slug).toBe("v0-10");
    expect(result.pruned.sort()).toEqual([
      "v0-1",
      "v0-2",
      "v0-3",
      "v0-4",
      "v0-5",
    ]);

    // New version exists and was rewritten from dev.
    expect(existsSync(join(docs, "v0-10", "index.mdx"))).toBe(true);
    expect(readFileSync(join(docs, "v0-10", "index.mdx"), "utf8")).toContain(
      'slug: "v0-10"',
    );
    expect(existsSync(join(versions, "v0-10.json"))).toBe(true);

    // Deep content links copied from dev are repointed at the new version.
    expect(readFileSync(join(docs, "v0-10", "admin", "page.mdx"), "utf8")).toBe(
      "See [otel](/v0-10/admin/otel/) for details.\n",
    );

    // Kept: v0-6..v0-10 + dev. Pruned: v0-1..v0-5.
    for (const minor of [6, 7, 8, 9, 10]) {
      expect(existsSync(join(docs, `v0-${minor}`))).toBe(true);
    }
    for (const minor of [1, 2, 3, 4, 5]) {
      expect(existsSync(join(docs, `v0-${minor}`))).toBe(false);
      expect(existsSync(join(versions, `v0-${minor}.json`))).toBe(false);
    }
    expect(existsSync(join(docs, "dev"))).toBe(true);

    const astro = readFileSync(join(root, "astro.config.mjs"), "utf8");
    expect(astro).toContain('current: { label: "0.10 (latest)"');
    expect(astro).toContain('slug: "v0-10"');
    expect(astro).toContain('slug: "dev"');
    expect(astro).not.toContain('slug: "v0-1"');
    expect(astro).not.toContain('slug: "v0-5"');

    const lychee = readFileSync(join(root, "lychee.toml"), "utf8");
    expect(lychee).toContain("v0-10");
    expect(lychee).not.toContain("v0-1/");
    expect(lychee).not.toContain("v0-5/");

    expect(readFileSync(join(docs, "index.mdx"), "utf8")).toContain(
      "url=/v0-10/",
    );
  } finally {
    cleanup();
  }
});

test("releaseDocsVersion is a no-op on patch releases", () => {
  const { root, cleanup } = makeRepo();
  try {
    const result = releaseDocsVersion(root, "v0.9.7");
    expect(result.created).toBe(false);
    expect(result.pruned).toEqual([]);
  } finally {
    cleanup();
  }
});
