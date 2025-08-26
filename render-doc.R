#!/usr/local/bin/Rscript
library(rvest)
library(litedown)

# Function to process documents in a directory
process_docs <- function(input_dir, output_dir) {
  # Create output directory if it doesn't exist
  if (!dir.exists(output_dir)) {
    dir.create(output_dir, recursive = TRUE)
  }
  
  all_docs <- list.files(
    input_dir,
    full.names = TRUE,
    pattern = "*.qmd"
  )

  for (doc in all_docs) {
    out <- litedown::mark(doc, options = list(smart = FALSE))

    read_html(out) |>
      html_node("body") |>
      xml2::write_html(file.path(output_dir, basename(out)))

    # delete the initially rendered html file
    file.remove(out)
  }
}

# Process root docs (for backwards compatibility)
root_docs <- list.files(
  "src/content",
  full.names = TRUE,
  pattern = "*.qmd"
)

if (length(root_docs) > 0) {
  for (doc in root_docs) {
    out <- litedown::mark(doc, options = list(smart = FALSE))

    read_html(out) |>
      html_node("body") |>
      xml2::write_html(file.path("src/generated", basename(out)))

    # delete the initially rendered html file
    file.remove(out)
  }
}

# Process versioned docs
version_dirs <- c("v0.1", "dev")

for (version in version_dirs) {
  input_dir <- file.path("src/content", version)
  output_dir <- file.path("src/generated", version)
  
  if (dir.exists(input_dir)) {
    cat("Processing version:", version, "\n")
    process_docs(input_dir, output_dir)
  }
}
