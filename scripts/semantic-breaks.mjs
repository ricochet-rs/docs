#!/usr/bin/env bun
/**
 * Adds semantic line breaks (one sentence per line) to MDX prose.
 * Also collapses multiple blank lines into single blank lines.
 * Preserves: frontmatter, imports, JSX, code blocks, lists, headings, tables.
 */

import { readFileSync, writeFileSync } from "fs";
import { Glob } from "bun";

// Match sentence endings followed by space and new sentence
// Handles: uppercase starts, quotes, code, brackets, and lowercase (for brand names like "ricochet")
const SENTENCE_BREAK = /([.!?]["']?)(\s+)(?=[A-Za-z"'`\[(])/g;

function isProseLineStart(line) {
  const trimmed = line.trim();
  if (!trimmed) return false;

  // Skip these patterns
  if (trimmed.startsWith("---")) return false; // frontmatter
  if (trimmed.startsWith("```")) return false; // code fence
  if (trimmed.startsWith("import ")) return false; // imports
  if (trimmed.startsWith("export ")) return false; // exports
  if (trimmed.startsWith("<")) return false; // JSX
  if (trimmed.startsWith("{")) return false; // JSX expressions
  if (trimmed.startsWith("#")) return false; // headings
  if (trimmed.startsWith("-")) return false; // unordered lists
  if (trimmed.startsWith("*")) return false; // unordered lists
  if (trimmed.match(/^\d+\./)) return false; // ordered lists
  if (trimmed.startsWith("|")) return false; // tables
  if (trimmed.startsWith(">")) return false; // blockquotes
  if (trimmed.startsWith("[")) return false; // link definitions

  return true;
}

function processFile(content) {
  const lines = content.split("\n");
  const result = [];

  let inFrontmatter = false;
  let inCodeBlock = false;
  let frontmatterCount = 0;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const trimmed = line.trim();

    // Track frontmatter
    if (trimmed === "---") {
      frontmatterCount++;
      inFrontmatter = frontmatterCount === 1;
      result.push(line);
      continue;
    }

    if (inFrontmatter) {
      result.push(line);
      continue;
    }

    // Track code blocks
    if (trimmed.startsWith("```")) {
      inCodeBlock = !inCodeBlock;
      result.push(line);
      continue;
    }

    if (inCodeBlock) {
      result.push(line);
      continue;
    }

    // Only process prose lines
    if (isProseLineStart(line)) {
      // Get the leading whitespace
      const indent = line.match(/^(\s*)/)[1];
      // Apply sentence breaks, preserving indentation
      const formatted = trimmed.replace(SENTENCE_BREAK, "$1\n" + indent);
      result.push(indent + formatted);
    } else {
      result.push(line);
    }
  }

  // Collapse multiple consecutive blank lines into single blank lines
  const collapsed = [];
  let prevBlank = false;
  for (const line of result) {
    const isBlank = line.trim() === "";
    if (isBlank && prevBlank) {
      continue; // Skip consecutive blank lines
    }
    collapsed.push(line);
    prevBlank = isBlank;
  }

  return collapsed.join("\n");
}

async function main() {
  const pattern = process.argv[2] || "src/content/docs/**/*.mdx";
  const glob = new Glob(pattern);

  let changed = 0;
  for await (const file of glob.scan(".")) {
    const content = readFileSync(file, "utf-8");
    const formatted = processFile(content);

    if (content !== formatted) {
      writeFileSync(file, formatted);
      console.log(`✓ ${file}`);
      changed++;
    }
  }

  if (changed === 0) {
    console.log("No changes needed");
  } else {
    console.log(`\nFormatted ${changed} file(s)`);
  }
}

main().catch(console.error);
