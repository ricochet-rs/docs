import { test, expect } from "bun:test";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { generateLatestVersionRedirects } from "./latest-version-redirects.mjs";

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

test("generates redirect for a regular page", () => {
  const { root, cleanup } = makeFixture({
    files: { "latest/admin/pricing/5-community-edition.mdx": "" },
  });
  try {
    expect(generateLatestVersionRedirects(root, "v0-7")).toEqual({
      "/admin/pricing/5-community-edition":
        "/latest/admin/pricing/5-community-edition/",
      "/v0-7/admin/pricing/5-community-edition":
        "/latest/admin/pricing/5-community-edition/",
    });
  } finally {
    cleanup();
  }
});

test("walks nested directories", () => {
  const { root, cleanup } = makeFixture({
    files: {
      "latest/admin/foo.mdx": "",
      "latest/admin/bar/baz.mdx": "",
      "latest/user/quickstart.md": "",
    },
  });
  try {
    expect(generateLatestVersionRedirects(root, "v0-7")).toEqual({
      "/admin/foo": "/latest/admin/foo/",
      "/admin/bar/baz": "/latest/admin/bar/baz/",
      "/user/quickstart": "/latest/user/quickstart/",
      "/v0-7/admin/foo": "/latest/admin/foo/",
      "/v0-7/admin/bar/baz": "/latest/admin/bar/baz/",
      "/v0-7/user/quickstart": "/latest/user/quickstart/",
    });
  } finally {
    cleanup();
  }
});

test("maps section index to its parent path", () => {
  const { root, cleanup } = makeFixture({
    files: { "latest/admin/index.mdx": "" },
  });
  try {
    expect(generateLatestVersionRedirects(root, "v0-7")).toEqual({
      "/admin": "/latest/admin/",
      "/v0-7/admin": "/latest/admin/",
    });
  } finally {
    cleanup();
  }
});

test("redirects the release root but not the site root", () => {
  const { root, cleanup } = makeFixture({
    files: {
      "latest/index.mdx": "",
      "latest/admin/foo.mdx": "",
    },
  });
  try {
    expect(generateLatestVersionRedirects(root, "v0-7")).toEqual({
      "/v0-7": "/latest/",
      "/admin/foo": "/latest/admin/foo/",
      "/v0-7/admin/foo": "/latest/admin/foo/",
    });
  } finally {
    cleanup();
  }
});

test("ignores non-markdown files in version dir", () => {
  const { root, cleanup } = makeFixture({
    files: {
      "latest/admin/foo.mdx": "",
      "latest/admin/data.json": "",
      "latest/admin/image.png": "",
    },
  });
  try {
    expect(generateLatestVersionRedirects(root, "v0-7")).toEqual({
      "/admin/foo": "/latest/admin/foo/",
      "/v0-7/admin/foo": "/latest/admin/foo/",
    });
  } finally {
    cleanup();
  }
});

test("skips unversioned redirects whose top segment shadows a root directory", () => {
  const { root, cleanup } = makeFixture({
    dirs: ["releases"],
    files: {
      "latest/admin/foo.mdx": "",
      "latest/releases/0.7.0.mdx": "",
    },
  });
  try {
    expect(generateLatestVersionRedirects(root, "v0-7")).toEqual({
      "/admin/foo": "/latest/admin/foo/",
      "/v0-7/admin/foo": "/latest/admin/foo/",
      "/v0-7/releases/0.7.0": "/latest/releases/0.7.0/",
    });
  } finally {
    cleanup();
  }
});

test("skips unversioned redirects whose top segment shadows a root file", () => {
  const { root, cleanup } = makeFixture({
    files: {
      "shadowed.mdx": "",
      "latest/admin/foo.mdx": "",
      "latest/shadowed.mdx": "",
    },
  });
  try {
    expect(generateLatestVersionRedirects(root, "v0-7")).toEqual({
      "/admin/foo": "/latest/admin/foo/",
      "/v0-7/admin/foo": "/latest/admin/foo/",
      "/v0-7/shadowed": "/latest/shadowed/",
    });
  } finally {
    cleanup();
  }
});

test("includes single-segment paths that don't shadow root content", () => {
  const { root, cleanup } = makeFixture({
    files: { "latest/quickstart.mdx": "" },
  });
  try {
    expect(generateLatestVersionRedirects(root, "v0-7")).toEqual({
      "/quickstart": "/latest/quickstart/",
      "/v0-7/quickstart": "/latest/quickstart/",
    });
  } finally {
    cleanup();
  }
});

test("throws when the latest release has its own content directory", () => {
  const { root, cleanup } = makeFixture({
    dirs: ["v0-7"],
    files: { "latest/admin/foo.mdx": "" },
  });
  try {
    expect(() => generateLatestVersionRedirects(root, "v0-7")).toThrow(
      /own content directory/,
    );
  } finally {
    cleanup();
  }
});
