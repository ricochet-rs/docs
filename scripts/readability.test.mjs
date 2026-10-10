import { test, expect } from "bun:test";
import { mdxToMarkdown } from "./readability.mjs";

test("blanks imports and keeps frontmatter", () => {
  const source = [
    "---",
    "title: Page",
    "---",
    'import { Aside } from "@astrojs/starlight/components";',
    "",
    "Text.",
  ].join("\n");
  expect(mdxToMarkdown(source)).toBe(
    ["---", "title: Page", "---", "", "", "Text."].join("\n"),
  );
});

test("unwraps nested components and dedents their bodies", () => {
  const source = [
    "<Tabs>",
    '  <TabItem label="A">',
    "    Prose inside a tab.",
    "",
    "    ```sh",
    "    echo hi",
    "    ```",
    "  </TabItem>",
    "</Tabs>",
  ].join("\n");
  expect(mdxToMarkdown(source).split("\n")).toEqual([
    "",
    "",
    "Prose inside a tab.",
    "",
    "```sh",
    "echo hi",
    "```",
    "",
    "",
  ]);
});

test("removes self-closing components spanning several lines", () => {
  const source = [
    "<LinkCard",
    '  title="Next"',
    '  href="/next/"',
    "/>",
    "After.",
  ].join("\n");
  expect(mdxToMarkdown(source).split("\n")).toEqual(["", "", "", "", "After."]);
});

test("keeps the line count of the source", () => {
  const source = [
    '<Aside type="tip">',
    "  A tip.",
    "</Aside>",
    "",
    "Body.",
  ].join("\n");
  expect(mdxToMarkdown(source).split("\n")).toHaveLength(5);
});

test("turns list items into sentences", () => {
  const source = [
    "- First item",
    "  continues here",
    "1. Second item.",
    "",
    "   ```sh",
    "   - not a list",
    "   ```",
    "After.",
  ].join("\n");
  expect(mdxToMarkdown(source).split("\n")).toEqual([
    "First item.",
    "continues here.",
    "Second item.",
    "",
    "```sh",
    "- not a list",
    "```",
    "After.",
  ]);
});

test("turns table cells into sentences", () => {
  const source = [
    "| Key | Meaning |",
    "| --- | ------- |",
    "| `a` | Uses a \\| b |",
    "| `b` | —       |",
  ].join("\n");
  expect(mdxToMarkdown(source).split("\n")).toEqual([
    "Key. Meaning.",
    "",
    "`a`. Uses a \\| b.",
    "`b`.",
  ]);
});
