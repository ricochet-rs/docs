import { defineRouteMiddleware } from "@astrojs/starlight/route-data";
import { getCollection } from "astro:content";
import { ogImagePath } from "./utils/ogImage";

// Old releases point search engines at their `/latest/` equivalent, and pages without one are not indexed.
const OLD_RELEASE = /^v\d+-\d+(?:\/|$)/;

let docIds: Promise<Set<string>> | undefined;

function getDocIds() {
  docIds ??= getCollection("docs").then(
    (entries) => new Set(entries.map(({ id }) => id)),
  );
  return docIds;
}

export const onRequest = defineRouteMiddleware(async (context) => {
  const { entry, head } = context.locals.starlightRoute;
  const ids = await getDocIds();

  // Swap the site-wide social card from `astro.config.mjs` for the page's generated one, which pages outside the collection, such as 404, lack
  if (ids.has(entry.id)) {
    const image = new URL(ogImagePath(entry.id), context.site).href;
    for (const { attrs } of head) {
      if (attrs?.property === "og:image" || attrs?.name === "twitter:image") {
        attrs.content = image;
      }
    }
  }

  const isDev = /^dev(?:\/|$)/.test(entry.id);
  if (!isDev && !OLD_RELEASE.test(entry.id)) return;

  const latestId = entry.id.replace(/^[^/]+/, "latest");
  const canonical = head.find(
    ({ tag, attrs }) => tag === "link" && attrs?.rel === "canonical",
  );

  if (!isDev && canonical && ids.has(latestId)) {
    canonical.attrs!.href = new URL(`/${latestId}/`, context.site).href;
  } else {
    head.push({ tag: "meta", attrs: { name: "robots", content: "noindex" } });
  }
});
