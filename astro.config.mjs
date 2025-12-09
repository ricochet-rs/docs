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

// https://astro.build/config
export default defineConfig({
  site: "https://docs.ricochet.rs",
  output: "static",
  trailingSlash: "ignore",
  integrations: [
    markdoc(),
    starlight({
      title: "ricochet",
      customCss: ["./src/styles/theme.css", "./src/styles/starlight-mappings.css", "./src/styles/global.css", "./src/styles/custom.css"],
      // customCss: ["./src/styles/global.css"],
      head: [
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
            href: "/favicon.png",
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
      },
      // https://expressive-code.com/reference/configuration/
      expressiveCode: {
        themes: ["catppuccin-mocha", "github-light"],
        // https://expressive-code.com/reference/style-overrides/
        useStarlightDarkModeSwitch: true,
        useStarlightUiThemeColors: false,
        styleOverrides: { borderRadius: "0", codePaddingBlock: "0.8rem" },
      },
      // https://github.com/ocavue/starlight-theme-nova
      plugins: [
        starlightThemeNova(),
        starlightLinksValidator(),
        starlightUtils({
          multiSidebar: {
            switcherStyle: "horizontalList",
          },
          // navLinks: {
          //   leading: { useSidebarLabelled: "leading" },
          // },
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
      sidebar: [
        {
          label: "User",
          items: [
            "user/quickstart",
            {
              label: "Content Items",
              autogenerate: { directory: "user/content-items" },
            },
            {
              label: "Deployment",
              autogenerate: { directory: "user/deployment" },
            },
            {
              label: "Managing Content",
              autogenerate: { directory: "user/managing-content" },
            },
            { label: "Tasks", autogenerate: { directory: "user/tasks" } },
          ],
        },
        {
          label: "Admin",
          items: [
            {
              label: "Installation",
              autogenerate: { directory: "/admin/installation" },
            },
            {
              label: "Configuration",
              autogenerate: { directory: "/admin/configuration" },
            },
            {
              label: "Pricing",
              autogenerate: { directory: "/admin/pricing" },
            },
            {
              label: "Technical Details",
              autogenerate: { directory: "/admin/technical/" },
            },
          ],
        },
      ],
    }),
  ],

  vite: {
    plugins: [tailwindcss()],
  },
});
