import type { APIRoute } from "astro";
import {
  API_DOCS_URL,
  API_ORIGIN,
  fetchOpenApiSpec,
} from "../../utils/ricochetApi";

export const GET: APIRoute = async ({ site }) => {
  const spec = await fetchOpenApiSpec();
  const info = spec.info as { title: string; version: string };
  const linkset = {
    linkset: [
      {
        anchor: `${API_ORIGIN}/api/`,
        "service-desc": [
          {
            href: new URL("/openapi.json", site).href,
            type: "application/openapi+json",
            title: `${info.title} ${info.version}`,
          },
        ],
        "service-doc": [{ href: API_DOCS_URL, type: "text/html" }],
      },
    ],
  };
  return new Response(JSON.stringify(linkset, null, 2), {
    headers: { "Content-Type": "application/linkset+json" },
  });
};
