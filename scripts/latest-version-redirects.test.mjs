import { test, expect } from "bun:test";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import {
  findLatestVersion,
  generateLatestVersionRedirects,
} from "./latest-version-redirects.mjs";

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
