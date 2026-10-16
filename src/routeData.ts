import { defineRouteMiddleware } from "@astrojs/starlight/route-data";
import { getCollection } from "astro:content";

// Old releases point search engines at their `/latest/` equivalent, and pages without one are not indexed.
const OLD_RELEASE = /^v\d+-\d+(?:\/|$)/;

let latestIds: Promise<Set<string>> | undefined;

function getLatestIds() {
  latestIds ??= getCollection("docs", ({ id }) =>
    /^latest(?:\/|$)/.test(id),
  ).then((entries) => new Set(entries.map(({ id }) => id)));
  return latestIds;
}

export const onRequest = defineRouteMiddleware(async (context) => {
  const { entry, head } = context.locals.starlightRoute;
  const isDev = /^dev(?:\/|$)/.test(entry.id);
  if (!isDev && !OLD_RELEASE.test(entry.id)) return;

  const latestId = entry.id.replace(/^[^/]+/, "latest");
  const canonical = head.find(
    ({ tag, attrs }) => tag === "link" && attrs?.rel === "canonical",
  );

  if (!isDev && canonical && (await getLatestIds()).has(latestId)) {
    canonical.attrs!.href = new URL(`/${latestId}/`, context.site).href;
  } else {
    head.push({ tag: "meta", attrs: { name: "robots", content: "noindex" } });
  }
});
