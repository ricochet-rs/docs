import type { APIRoute, GetStaticPaths } from "astro";
import { fetchSkillFiles } from "../../../../utils/agentSkills";

export const getStaticPaths = (async () => {
  const files = await fetchSkillFiles();
  return files.map(({ skill, path, content }) => ({
    params: { skill, file: path },
    props: { content },
  }));
}) satisfies GetStaticPaths;

export const GET: APIRoute = ({ props }) =>
  new Response(props.content, {
    headers: { "Content-Type": "text/markdown; charset=utf-8" },
  });
