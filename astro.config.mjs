// @ts-check
import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import sitemap from "@astrojs/sitemap";
import { agentDiscovery } from "@ricochet-rs/agent-discovery";
import starlightThemeNova from "starlight-theme-nova";
import tailwindcss from "@tailwindcss/vite";
// https://docs.astro.build/en/guides/integrations-guide/markdoc //
import markdoc from "@astrojs/markdoc";
import starlightLinksValidator from "starlight-links-validator";
import starlightUtils from "@lorenzo_lewis/starlight-utils";
import starlightScrollToTop from "starlight-scroll-to-top";
import starlightVersions from "starlight-versions";
import { latestVersionRedirectsIntegration } from "./scripts/latest-version-redirects.mjs";
import rehypeMermaid from "rehype-mermaid";
import rehypeFigure from "rehype-figure";
import rehypeTrailingSlash from "./scripts/rehype-trailing-slash.mjs";

// https://astro.build/config
export default defineConfig({
  site: "https://docs.ricochet.rs",
  output: "static",
  trailingSlash: "ignore",
  integrations: [
    markdoc(),
    agentDiscovery({
      siteTitle: "ricochet",
      twinPath: "index",
      strip: ["footer", ".expressive-code .copy", ".expressive-code .sr-only"],
      robots: { extraRules: ["Disallow: /pagefind/"] },
      llmsInclude: (path) => !/^\/(v\d+-\d+|dev)\//.test(path),
      llms: {
        title: "ricochet documentation",
        summary:
          "Deploy and run R, Julia, and Python apps, APIs, reports, and scripts with ricochet.",
        sections: [
          { heading: "User guide", prefix: "/latest/user/" },
          { heading: "Admin guide", prefix: "/latest/admin/" },
          { heading: "ricochet Cloud", prefix: "/latest/cloud/" },
          { heading: "Releases", prefix: "/releases/" },
        ],
        optional: [
          {
            title: "Website",
            href: "https://ricochet.rs",
            description: "the product site",
          },
          {
            title: "API reference",
            href: "https://try.ricochet.rs/api/docs",
            description: "the REST API reference",
          },
        ],
      },
    }),
    latestVersionRedirectsIntegration("src/content/docs", "v0-24"),
    // Only `/latest/` and unversioned pages go in the sitemap, since old releases and dev point search engines at `/latest/`
    sitemap({
      filter: (page) => !/^\/(v\d+-\d+|dev)\//.test(new URL(page).pathname),
    }),
    starlight({
      title: "ricochet",
      customCss: [
        "./src/styles/theme.css",
        "./src/styles/starlight-mappings.css",
        "./src/styles/global.css",
        "./src/styles/custom.css",
      ],
      // customCss: ["./src/styles/global.css"],
      head: [
        {
          // Prevent FOUC by setting base font-size before page renders
          tag: "style",
          content: ":root { font-size: 14px; }",
        },
        {
          tag: "link",
          attrs: {
            rel: "icon",
            href: "/favicon.ico",
            sizes: "32x32",
          },
        },
        {
          tag: "link",
          attrs: {
            rel: "icon",
            href: "/favicon-32x32.png",
            sizes: "32x32",
          },
        },
        {
          tag: "link",
          attrs: {
            rel: "icon",
            href: "/favicon.svg",
            sizes: "32x32",
          },
        },
        // {
        // tag: "link",
        // attrs: {
        // rel: "icon",
        // href: "/favicon-96x96.png",
        // sizes: "96x96",
        // },
        // },
        {
          tag: "link",
          attrs: {
            rel: "manifest",
            href: "/site.webmanifest",
            sizes: "96x96",
          },
        },
        // {
        // tag: "link",
        // attrs: {
        // rel: "icon",
        // href: "/web-app-manifest-192x192.png",
        // sizes: "192x192",
        // },
        // },
        // {
        // tag: "link",
        // attrs: {
        // rel: "icon",
        // href: "/web-app-manifest-512x512.png",
        // sizes: "512x512",
        // },
        // },
      ],
      components: {
        // Override the `ThemeSelect` component from the Nova theme
        ThemeSelect: "./src/components/ThemeSelect.astro",
        ThemeProvider: "./src/components/ThemeProvider.astro",
        SiteTitle: "./src/components/SiteTitle.astro",
        // Override Banner and PageTitle to remove version outdated warnings
        Banner: "./src/components/Banner.astro",
        PageTitle: "./src/components/PageTitle.astro",
        // Override ToC to wrap long titles
        TableOfContents: "./src/components/TableOfContents.astro",
        // Override Sidebar to render backticks as code
        Sidebar: "./src/components/Sidebar.astro",
        // Override Pagination to render backticks as code
        Pagination: "./src/components/Pagination.astro",
        // Override Header to add release notes icon on mobile
        Header: "./src/components/Header.astro",
        Footer: "./src/components/Footer.astro",
      },
      // https://expressive-code.com/reference/configuration/
      expressiveCode: {
        themes: ["catppuccin-mocha", "github-light"],
        // https://expressive-code.com/reference/style-overrides/
        useStarlightDarkModeSwitch: true,
        useStarlightUiThemeColors: false,
        styleOverrides: {
          borderRadius: "0",
          codePaddingBlock: "0.8rem",
          codeFontSize: "0.775rem",
        },
      },
      // https://github.com/ocavue/starlight-theme-nova
      plugins: [
        starlightVersions({
          current: { label: "0.24 (latest)", redirect: "root" },
          versions: [
            { slug: "latest", label: "0.24" },
            { slug: "v0-23", label: "0.23" },
            { slug: "v0-22", label: "0.22" },
            { slug: "v0-21", label: "0.21" },
            { slug: "v0-20", label: "0.20" },
            { slug: "dev", label: "dev" },
          ],
        }),
        starlightThemeNova({
          nav: [
            { label: "Release Notes", href: "/releases/" },
            {
              label: "API Reference",
              href: "https://try.ricochet.rs/api/docs",
            },
          ],
        }),
        starlightLinksValidator({
          errorOnRelativeLinks: false,
        }),
        starlightUtils({
          multiSidebar: {
            switcherStyle: "horizontalList",
          },
        }),
        starlightScrollToTop(),
      ],
      social: [
        {
          icon: "github",
          label: "GitHub",
          href: "https://github.com/ricochet-rs/docs",
        },
        {
          icon: "discord",
          label: "Discord",
          href: "https://discord.gg/ctWt3du7aw",
        },
        {
          icon: "matrix",
          label: "Matrix",
          href: "https://matrix.ricochet.rs",
        },
      ],
      editLink: {
        baseUrl: "https://github.com/ricochet-rs/docs/edit/main/",
      },
      routeMiddleware: "./src/routeData.ts",
      // Sidebar for root (version selector page only)
      // Versioned docs use their own sidebar configs in src/content/versions/
      sidebar: [],
    }),
  ],
  markdown: {
    // Astro 6.4.2 regressed the default for `gfm` from true to false;
    // see https://github.com/withastro/astro/issues/16971
    gfm: true,
    rehypePlugins: [
      [rehypeMermaid, { strategy: "pre-mermaid" }],
      rehypeFigure,
      rehypeTrailingSlash,
    ],
  },
  vite: {
    plugins: [tailwindcss()],
  },
});
