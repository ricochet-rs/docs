import { codeToHtml } from "shiki";
import fs from "fs/promises";
import path from "path";

const dir = "./src/generated";

async function processDirectory(dirPath) {
  const entries = await fs.readdir(dirPath, { withFileTypes: true });

  for (const entry of entries) {
    const fullPath = path.join(dirPath, entry.name);

    if (entry.isDirectory()) {
      await processDirectory(fullPath);
    } else if (entry.name.endsWith(".html")) {
      const html = await fs.readFile(fullPath, "utf8");
      const updated = await replaceCodeBlocks(html);
      await fs.writeFile(fullPath, updated);
    }
  }
}

await processDirectory(dir);

async function replaceCodeBlocks(html) {
  const matches = [
    ...html.matchAll(
      /<pre><code(?: class="(language-[\w-]+)")?>([\s\S]*?)<\/code><\/pre>/g,
    ),
  ];

  for (const match of matches) {
    const [fullBlock, langClass, rawCode] = match;

    // If there is no language class, set lang to 'none'
    const lang = langClass ? langClass.replace("language-", "") : "plain";
    const decodedCode = decode(rawCode);

    const highlighted = await codeToHtml(decodedCode, {
      lang,
      themes: {
        light: "github-light",
        dark: "catppuccin-mocha",
      },
    });

    html = html.replace(fullBlock, highlighted);
  }

  return html;
}

function decode(str) {
  return str
    .replace(/&lt;/g, "<")
    .replace(/&gt;/g, ">")
    .replace(/&amp;/g, "&")
    .replace(/&quot;/g, '"')
    .replace(/&#39;/g, "'");
}
