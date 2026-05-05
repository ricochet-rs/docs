import { readdirSync } from "node:fs";

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
