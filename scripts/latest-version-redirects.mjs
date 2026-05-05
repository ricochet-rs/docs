import { readdirSync } from "node:fs";
import { join, relative, sep } from "node:path";

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
