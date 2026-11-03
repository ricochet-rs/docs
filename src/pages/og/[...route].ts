import { getCollection } from "astro:content";
import { OGImageRoute } from "astro-og-canvas";
import { ogImageOptions, ogImageSlug } from "../../utils/ogImage";

const entries = await getCollection("docs");

export const { getStaticPaths, GET } = await OGImageRoute({
  pages: Object.fromEntries(entries.map((entry) => [entry.id, entry])),
  getSlug: (id) => ogImageSlug(id),
  getImageOptions: (_id, entry: (typeof entries)[number]) =>
    ogImageOptions(entry),
});
