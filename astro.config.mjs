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
  integrations: [
    markdoc(),
    starlight({
      title: "ricochet - Documentation",
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
      customCss: ["./src/styles/global.css", "./src/styles/custom.css"],
      components: {
        // Override the `ThemeSelect` component from the Nova theme
        ThemeSelect: "./src/components/ThemeSelect.astro",
      },
      // https://expressive-code.com/reference/configuration/
      expressiveCode: {
        // themes: ['dracula', 'github-light'],
        // https://expressive-code.com/reference/style-overrides/
        useStarlightDarkModeSwitch: true,
        useStarlightUiThemeColors: true,
        styleOverrides: { borderRadius: "0.5rem", codePaddingBlock: "0.8rem" },
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
      editLink: {
        baseUrl:
          "https://github.com/ricochet-rs/ricochet-docs/_edit/main/astro/src/content/docs/",
      },
      sidebar: [
        // {
          // label: "leading",
          // items: [
            // { label: "Docs", link: "/docs" },
            // { label: "Demos", link: "/demos/1" },
          // ],
        // },
        {
          label: "Admin",
          items: [
            {
              label: "Configuration",
              autogenerate: { directory: "/configuration" },
            },
            {
              label: "Installation",
              items: [
                { label: "Prerequisites", slug: "installation/prerequisites" },
                { label: "Container", slug: "installation/container" },
                { label: "Binary", slug: "installation/binary" },
              ],
            },
          ],
        },
        {
          label: "User",
          items: [
            { label: "Deployment", autogenerate: { directory: "deployment" } },
          ],
        },
      ],
    }),
  ],

  vite: {
    plugins: [tailwindcss()],
  },
});
