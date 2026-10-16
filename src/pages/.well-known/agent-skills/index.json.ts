import type { APIRoute } from "astro";
import {
  fetchSkillFiles,
  frontmatterField,
  sha256Digest,
} from "../../../utils/agentSkills";

export const GET: APIRoute = async () => {
  const files = await fetchSkillFiles();
  const skills = files
    .filter(({ path }) => path === "SKILL.md")
    .map(({ skill, content }) => ({
      name: frontmatterField(content, "name"),
      type: "skill-md",
      description: frontmatterField(content, "description"),
      url: `/.well-known/agent-skills/${skill}/SKILL.md`,
      digest: sha256Digest(content),
    }));
  const index = {
    $schema: "https://schemas.agentskills.io/discovery/0.2.0/schema.json",
    skills,
  };
  return new Response(JSON.stringify(index, null, 2), {
    headers: { "Content-Type": "application/json" },
  });
};
