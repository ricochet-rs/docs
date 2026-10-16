import type { APIRoute } from "astro";
import { fetchOpenApiSpec } from "../utils/ricochetApi";

export const GET: APIRoute = async () =>
  new Response(JSON.stringify(await fetchOpenApiSpec()), {
    headers: { "Content-Type": "application/json" },
  });
