import type { CollectionEntry } from "astro:content";
import type { OGImageOptions } from "astro-og-canvas";

/** File name of a docs entry's social card, keeping dots such as `v0.25.0` intact. */
export function ogImageSlug(id: string) {
  return `${id || "index"}.png`;
}

/** Path of the generated social card for a docs entry, served by `src/pages/og/[...route].ts`. */
export function ogImagePath(id: string) {
  return `/og/${ogImageSlug(id)}`;
}

const SECTIONS: Record<string, string> = {
  user: "User guide",
  admin: "Admin guide",
  cloud: "ricochet Cloud",
};

/** Subtitle for pages without a description, such as "User guide · v0.24". */
function sectionLabel(id: string) {
  const [version, section] = id.split("/");
  if (version === "releases") return "Release notes";
  const name = SECTIONS[section] ?? "Documentation";
  if (version === "latest" || !version) return name;
  return `${name} · ${version === "dev" ? "dev" : version.replace("-", ".")}`;
}

export function ogImageOptions({
  id,
  data,
}: CollectionEntry<"docs">): OGImageOptions {
  return {
    title: data.title,
    description: data.description ?? sectionLabel(id),
    logo: { path: "./src/assets/og/logo.png", size: [222] },
    bgImage: { path: "./src/assets/og/background.png" },
    padding: 72,
    fonts: [
      "./src/assets/og/fonts/Inter-Bold.ttf",
      "./src/assets/og/fonts/Inter-Regular.ttf",
    ],
    font: {
      title: {
        families: ["Inter"],
        weight: "Bold",
        size: 72,
        lineHeight: 1.15,
        color: [255, 255, 255],
      },
      description: {
        families: ["Inter"],
        weight: "Normal",
        size: 34,
        lineHeight: 1.35,
        color: [215, 198, 246],
      },
    },
  };
}
