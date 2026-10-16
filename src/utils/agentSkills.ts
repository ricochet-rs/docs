import { createHash } from "node:crypto";

const REPO = "ricochet-rs/skills";
const BRANCH = "main";
const SKILL_FILE =
  /^plugins\/[^/]+\/skills\/([^/]+)\/((?:references\/)?[^/]+\.md)$/;

export interface SkillFile {
  skill: string;
  path: string;
  content: string;
}

let files: Promise<SkillFile[]> | undefined;

async function fetchOk(url: string): Promise<Response> {
  const res = await fetch(url);
  if (!res.ok) throw new Error(`${url} returned ${res.status}`);
  return res;
}

// Each skill's SKILL.md and references/*.md from the ricochet-rs/skills repo.
export function fetchSkillFiles(): Promise<SkillFile[]> {
  files ??= (async () => {
    const res = await fetchOk(
      `https://api.github.com/repos/${REPO}/git/trees/${BRANCH}?recursive=1`,
    );
    const { tree } = (await res.json()) as { tree: { path: string }[] };
    const matches = tree.flatMap(({ path }) => {
      const match = path.match(SKILL_FILE);
      return match ? [{ repoPath: path, skill: match[1], path: match[2] }] : [];
    });
    return Promise.all(
      matches.map(async ({ repoPath, skill, path }) => {
        const raw = await fetchOk(
          `https://raw.githubusercontent.com/${REPO}/${BRANCH}/${repoPath}`,
        );
        return { skill, path, content: await raw.text() };
      }),
    );
  })();
  return files;
}

export function frontmatterField(content: string, field: string): string {
  const match = content.match(new RegExp(`^${field}:\\s*(.+)$`, "m"));
  if (!match) throw new Error(`SKILL.md is missing \`${field}\``);
  return match[1].trim();
}

export function sha256Digest(content: string): string {
  return `sha256:${createHash("sha256").update(content).digest("hex")}`;
}
