#!/usr/bin/env bun
/**
 * Checks MDX pages against the readability thresholds in .readability.yml.
 * The readability CLI only reads plain Markdown, so each page is converted first:
 * imports, exports, and component tags are blanked and component bodies are dedented.
 * The CLI skips text in lists and tables, so list items and table cells become sentences.
 * Every step keeps line numbers aligned with the source page.
 *
 * Usage: bun run scripts/readability.mjs [file.mdx ...]
 * Without arguments, every page under src/content/docs is checked.
 */

import { createHash } from "node:crypto";
import {
  chmodSync,
  existsSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, relative, resolve } from "node:path";
import { Glob } from "bun";

// renovate: datasource=github-releases depName=adaptive-enforcement-lab/readability
const READABILITY_VERSION = "v3.1.2";

const ROOT = resolve(import.meta.dir, "..");
const DOCS_GLOB = "src/content/docs/**/*.mdx";
const CONFIG = join(ROOT, ".readability.yml");

const OPEN_TAG = /^\s*<([A-Z][\w.]*)\b/;
const MODULE_LINE = /^\s*(import|export)\s/;
const INLINE_TAG = /<\/?[A-Z][\w.]*(\s[^<>]*)?\/?>/g;
const FENCE = /^\s*(```|~~~)/;
const LIST_ITEM = /^\s*(?:[-*+]|\d+[.)])\s+/;
const TABLE_ROW = /^\s*\|/;
const TABLE_RULE = /^\s*\|[\s|:-]+\|\s*$/;
const DASHES_ONLY = /^[\s—–-]*$/;

function indentOf(line) {
  return line.match(/^\s*/)[0].length;
}

/** Index of the line that ends the tag opened at `start`, i.e. the first line containing `>`. */
function tagEnd(lines, start) {
  for (let i = start; i < lines.length; i++) {
    if (lines[i].includes(">")) return i;
  }
  return lines.length - 1;
}

/** Index of the closing tag that matches the component opened before `start`. */
function closingLine(lines, start, name) {
  const open = new RegExp(`^\\s*<${name}\\b`);
  const close = new RegExp(`</${name}>`);
  let depth = 1;
  for (let i = start; i < lines.length; i++) {
    if (open.test(lines[i]) && !lines[i].trimEnd().endsWith("/>")) depth++;
    if (close.test(lines[i])) depth--;
    if (depth === 0) return i;
  }
  return -1;
}

function unwrapComponents(lines) {
  const out = [];
  for (let i = 0; i < lines.length; i++) {
    const match = lines[i].match(OPEN_TAG);
    if (!match) {
      out.push(lines[i]);
      continue;
    }
    const name = match[1];
    const end = tagEnd(lines, i);
    const selfClosing = lines[end].trimEnd().endsWith("/>");
    const sameLineClose = lines[end].includes(`</${name}>`);
    if (selfClosing || sameLineClose) {
      for (let j = i; j <= end; j++) out.push(selfClosing ? "" : lines[j]);
      i = end;
      continue;
    }
    const close = closingLine(lines, end + 1, name);
    if (close === -1) {
      out.push(lines[i]);
      continue;
    }
    for (let j = i; j <= end; j++) out.push("");
    const body = lines.slice(end + 1, close);
    const indent = Math.min(
      ...body.filter((l) => l.trim()).map(indentOf),
      Number.MAX_SAFE_INTEGER,
    );
    out.push(
      ...unwrapComponents(
        body.map((l) => l.slice(Math.min(indent, indentOf(l)))),
      ),
    );
    out.push("");
    i = close;
  }
  return out;
}

/** Ends a fragment with a period so the CLI counts it as its own sentence. */
function sentence(text) {
  const trimmed = text.trim();
  return /[.!?]$/.test(trimmed) ? trimmed : `${trimmed}.`;
}

/** Splits a table row into cells, ignoring escaped pipes. */
function cells(row) {
  return row
    .trim()
    .replace(/^\||\|$/g, "")
    .split(/(?<!\\)\|/)
    .map((cell) => cell.trim())
    .filter((cell) => !DASHES_ONLY.test(cell));
}

/** Turns list items and table rows into plain sentences, one source line at a time. */
function flattenListsAndTables(lines) {
  let fenced = false;
  let inList = false;
  return lines.map((line) => {
    if (FENCE.test(line)) {
      fenced = !fenced;
      return inList ? line.trimStart() : line;
    }
    if (fenced) return inList ? line.trimStart() : line;
    if (!line.trim()) return line;
    if (TABLE_RULE.test(line)) return "";
    if (TABLE_ROW.test(line)) return cells(line).map(sentence).join(" ");
    if (LIST_ITEM.test(line)) {
      inList = true;
      return sentence(line.replace(LIST_ITEM, ""));
    }
    if (inList && /^\s/.test(line)) return sentence(line);
    inList = false;
    return line;
  });
}

/** Converts MDX to Markdown the readability CLI can score, keeping line numbers stable. */
export function mdxToMarkdown(source) {
  const lines = source.split("\n");
  let start = 0;
  if (lines[0] === "---") {
    const end = lines.indexOf("---", 1);
    if (end !== -1) start = end + 1;
  }
  const body = lines
    .slice(start)
    .map((line) => (MODULE_LINE.test(line) ? "" : line));
  return [
    ...lines.slice(0, start),
    ...flattenListsAndTables(unwrapComponents(body)),
  ]
    .map((line) => line.replace(INLINE_TAG, ""))
    .join("\n");
}

function platformAsset() {
  const os = { darwin: "darwin", linux: "linux" }[process.platform];
  const arch = { x64: "amd64", arm64: "arm64" }[process.arch];
  if (!os || !arch) {
    throw new Error(
      `readability has no release for ${process.platform}/${process.arch}`,
    );
  }
  return `readability_${os}_${arch}`;
}

async function download(url) {
  const response = await fetch(url);
  if (!response.ok)
    throw new Error(`GET ${url} failed with ${response.status}`);
  return Buffer.from(await response.arrayBuffer());
}

/** Downloads the pinned release once, verifies its checksum, and returns the binary path. */
async function ensureBinary() {
  const asset = platformAsset();
  const dir = join(
    ROOT,
    "node_modules",
    ".cache",
    "readability",
    READABILITY_VERSION,
  );
  const binary = join(dir, asset);
  if (existsSync(binary)) return binary;

  const base = `https://github.com/adaptive-enforcement-lab/readability/releases/download/${READABILITY_VERSION}`;
  const archive = await download(`${base}/${asset}.tar.gz`);
  const checksums = (await download(`${base}/checksums.txt`)).toString();
  const expected = checksums
    .split("\n")
    .find((line) => line.endsWith(`  ${asset}.tar.gz`))
    ?.split(" ")[0];
  const actual = createHash("sha256").update(archive).digest("hex");
  if (!expected || expected !== actual) {
    throw new Error(`checksum mismatch for ${asset}.tar.gz`);
  }

  mkdirSync(dir, { recursive: true });
  const tarball = join(dir, `${asset}.tar.gz`);
  writeFileSync(tarball, archive);
  const tar = Bun.spawnSync(["tar", "-xzf", tarball, "-C", dir, asset]);
  if (tar.exitCode !== 0) throw new Error(tar.stderr.toString());
  rmSync(tarball);
  chmodSync(binary, 0o755);
  return binary;
}

function formatMetrics(result) {
  const r = result.readability;
  return [
    `grade ${r.flesch_kincaid_grade.toFixed(1)}`,
    `ARI ${r.ari.toFixed(1)}`,
    `ease ${r.flesch_reading_ease.toFixed(1)}`,
    `words ${result.structural.words}`,
  ].join(", ");
}

async function main() {
  const files = process.argv.slice(2).length
    ? process.argv.slice(2).map((f) => relative(ROOT, resolve(f)))
    : [...new Glob(DOCS_GLOB).scanSync({ cwd: ROOT })];
  const pages = files.filter((f) => f.endsWith(".mdx")).sort();
  if (!pages.length) return;

  const binary = await ensureBinary();
  const work = mkdtempSync(join(tmpdir(), "readability-"));
  try {
    for (const page of pages) {
      const target = join(work, page.replace(/\.mdx$/, ".md"));
      mkdirSync(dirname(target), { recursive: true });
      writeFileSync(
        target,
        mdxToMarkdown(readFileSync(join(ROOT, page), "utf8")),
      );
    }

    const run = Bun.spawnSync([
      binary,
      "--config",
      CONFIG,
      "--format",
      "json",
      work,
    ]);
    if (run.exitCode !== 0) throw new Error(run.stderr.toString());

    const failures = JSON.parse(run.stdout.toString()).filter(
      (r) => r.status === "fail",
    );
    for (const result of failures) {
      const page = relative(work, result.file).replace(/\.md$/, ".mdx");
      console.log(`${page} (${formatMetrics(result)})`);
      for (const d of result.diagnostics) {
        console.log(`  ${page}:${d.line} ${d.rule}: ${d.message}`);
      }
    }
    console.log(
      `readability: ${pages.length - failures.length}/${pages.length} pages pass`,
    );
    if (failures.length) process.exitCode = 1;
  } finally {
    rmSync(work, { recursive: true, force: true });
  }
}

if (import.meta.main) await main();
