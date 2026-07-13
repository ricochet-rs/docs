import { visit } from "unist-util-visit";

// The static host serves pages only at their trailing-slash URL
// (e.g. `/foo/bar/`) and 404s the bare path. Astro renders authored
// markdown links verbatim, so internal links written without a trailing
// slash break in production. This plugin appends the slash at build time.
export default function rehypeTrailingSlash() {
  return (tree) => {
    visit(tree, "element", (node) => {
      if (node.tagName !== "a") return;
      const href = node.properties?.href;
      if (typeof href !== "string" || href.length === 0) return;

      // Skip external links, protocol-relative URLs, and special schemes.
      if (/^[a-z][a-z0-9+.-]*:/i.test(href) || href.startsWith("//")) return;
      // Skip pure fragments.
      if (href.startsWith("#")) return;

      const hashIndex = href.indexOf("#");
      const queryIndex = href.indexOf("?");
      let cut = href.length;
      if (hashIndex !== -1) cut = Math.min(cut, hashIndex);
      if (queryIndex !== -1) cut = Math.min(cut, queryIndex);

      const path = href.slice(0, cut);
      const suffix = href.slice(cut);
      if (path.length === 0 || path.endsWith("/")) return;

      // Skip links to files (last segment has an extension).
      const lastSegment = path.slice(path.lastIndexOf("/") + 1);
      if (lastSegment.includes(".")) return;

      node.properties.href = `${path}/${suffix}`;
    });
  };
}
