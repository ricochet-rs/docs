#!/usr/bin/env node
import { watch } from "fs";
import { spawn } from "child_process";
import { readFileSync, writeFileSync } from "fs";
import path from "path";

console.log("👀 Watching src/content/ for changes...");

let isProcessing = false;

const regenerateDocs = () => {
  if (isProcessing) return;

  isProcessing = true;
  console.log("📝 Content changed, regenerating docs...");

  const renderProcess = spawn("./render-doc.R", [], { stdio: "inherit" });

  renderProcess.on("close", (code) => {
    if (code === 0) {
      console.log("✅ Docs regenerated successfully");

      // Run syntax highlighting
      const highlightProcess = spawn("node", ["highlight-partials.mjs"], {
        stdio: "inherit",
      });

      highlightProcess.on("close", (code) => {
        if (code === 0) {
          console.log("✨ Syntax highlighting applied");

          // Force cargo rebuild by making a trivial change to trigger recompilation
          console.log("🔄 Triggering cargo rebuild...");
          try {
            const docsPath = "src/docs/mod.rs";
            let content = readFileSync(docsPath, "utf8");

            // Add and immediately remove a comment to trigger file change detection
            const timestamp = Date.now();
            const marker = `// Auto-rebuild trigger: ${timestamp}\n`;

            // Add the marker
            writeFileSync(docsPath, marker + content);

            // Remove it immediately to keep the file clean
            setTimeout(() => {
              writeFileSync(docsPath, content);
              console.log("⚡ Cargo rebuild triggered");
            }, 100);
          } catch (error) {
            console.log("⚠️ Failed to trigger rebuild:", error.message);
          }
          isProcessing = false;
        } else {
          console.log("⚠️ Highlighting failed");
          isProcessing = false;
        }
      });
    } else {
      console.log("❌ Doc generation failed");
      isProcessing = false;
    }
  });
};

// Watch the src/content directory
watch("src/content", { recursive: true }, (eventType, filename) => {
  if (filename && filename.endsWith(".qmd")) {
    console.log(`🔄 Detected change: ${filename}`);

    // Debounce - wait 500ms for multiple changes
    clearTimeout(regenerateDocs.timeout);
    regenerateDocs.timeout = setTimeout(regenerateDocs, 500);
  }
});

console.log(
  "✅ Content watcher started. Edit .qmd files to trigger regeneration.",
);
