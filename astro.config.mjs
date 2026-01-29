// @ts-check
import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import starlightThemeNova from "starlight-theme-nova";
import tailwindcss from "@tailwindcss/vite";
// https://docs.astro.build/en/guides/integrations-guide/markdoc //
import markdoc from "@astrojs/markdoc";
import starlightLinksValidator from "starlight-links-validator";
import starlightUtils from "@lorenzo_lewis/starlight-utils";
import starlightScrollToTop from "starlight-scroll-to-top";
import starlightVersions from "starlight-versions";

// https://astro.build/config
export default defineConfig({
  site: "https://docs.ricochet.rs",
  output: "static",
  trailingSlash: "ignore",
  integrations: [
    markdoc(),
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
          current: { label: "0.1 (latest)", redirect: "root" },
          versions: [{ slug: "v0-1", label: "0.1" }],
        }),
        starlightThemeNova({
          nav: [{ label: "Release Notes", href: "/releases/" }],
        }),
        starlightLinksValidator(),
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
          href: "https://github.com/ricochet-rs/ricochet-docs",
        },
        {
          icon: "discord",
          label: "Discord",
          href: "https://discord.gg/tAsn8GbR",
        },
      ],
      editLink: {
        baseUrl:
          "https://github.com/ricochet-rs/ricochet-docs/edit/main/src/content/docs/",
      },
      // Sidebar for root (version selector page only)
      // Versioned docs use their own sidebar configs in src/content/versions/
      sidebar: [],
    }),
  ],

  vite: {
    plugins: [tailwindcss()],
  },
});
