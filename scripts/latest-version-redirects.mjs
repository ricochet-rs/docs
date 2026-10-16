import { existsSync, mkdirSync, readdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join, relative, sep } from "node:path";

const LATEST = "latest";

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

// `latestRelease` is the version slug, such as `v0-24`, that `latest/` holds.
export function generateLatestVersionRedirects(contentDir, latestRelease) {
  if (existsSync(join(contentDir, latestRelease))) {
    throw new Error(
      `${latestRelease} has its own content directory, so it is not the release in ${LATEST}/`,
    );
  }
  const versionDir = join(contentDir, LATEST);
  const redirects = {};
  for (const rel of walkPages(versionDir)) {
    const noExt = rel.replace(/\.mdx?$/, "");
    if (noExt === "index") {
      redirects[`/${latestRelease}`] = `/${LATEST}/`;
      continue;
    }
    const path = noExt.endsWith("/index")
      ? noExt.slice(0, -"/index".length)
      : noExt;
    redirects[`/${latestRelease}/${path}`] = `/${LATEST}/${path}/`;
    const topSegment = path.split("/")[0];
    if (
      existsSync(join(contentDir, topSegment)) ||
      existsSync(join(contentDir, `${topSegment}.mdx`)) ||
      existsSync(join(contentDir, `${topSegment}.md`))
    ) {
      continue;
    }
    redirects[`/${path}`] = `/${LATEST}/${path}/`;
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

export function latestVersionRedirectsIntegration(contentDir, latestRelease) {
  return {
    name: "latest-version-redirects",
    hooks: {
      "astro:build:done": ({ dir }) => {
        const distDir = fileURLToPath(dir);
        const redirects = generateLatestVersionRedirects(
          contentDir,
          latestRelease,
        );
        for (const [source, destination] of Object.entries(redirects)) {
          // Skip when the destination wasn't built (e.g. draft pages) — a
          // redirect stub pointing at a missing page would 404 in production.
          const destPath = join(
            distDir,
            destination.slice(1).replace(/\/$/, ""),
            "index.html",
          );
          if (!existsSync(destPath)) continue;
          const filePath = join(distDir, source.slice(1), "index.html");
          mkdirSync(dirname(filePath), { recursive: true });
          writeFileSync(filePath, renderRedirectHtml(destination));
        }
      },
    },
  };
}
