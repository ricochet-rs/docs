import { existsSync, mkdirSync, readdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join, relative, sep } from "node:path";

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
    const topSegment = path.split("/")[0];
    if (
      existsSync(join(contentDir, topSegment)) ||
      existsSync(join(contentDir, `${topSegment}.mdx`)) ||
      existsSync(join(contentDir, `${topSegment}.md`))
    ) {
      continue;
    }
    redirects[`/${path}`] = `/${latest}/${path}/`;
  }
  return redirects;
}

function renderRedirectHtml(destination) {
  return `<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta http-equiv="refresh" content="0; url=${destination}">
    <link rel="canonical" href="${destination}">
    <meta name="robots" content="noindex">
    <title>Redirecting…</title>
  </head>
  <body>
    Redirecting to <a href="${destination}">${destination}</a>.
  </body>
</html>
`;
}

export function latestVersionRedirectsIntegration(contentDir) {
  return {
    name: "latest-version-redirects",
    hooks: {
      "astro:build:done": ({ dir }) => {
        const distDir = fileURLToPath(dir);
        const redirects = generateLatestVersionRedirects(contentDir);
        for (const [source, destination] of Object.entries(redirects)) {
          const filePath = join(distDir, source.slice(1), "index.html");
          mkdirSync(dirname(filePath), { recursive: true });
          writeFileSync(filePath, renderRedirectHtml(destination));
        }
      },
    },
  };
}
