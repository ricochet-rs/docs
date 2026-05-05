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
