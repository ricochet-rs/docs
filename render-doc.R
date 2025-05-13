#!/usr/local/bin/Rscript
library(rvest)
library(litedown)

all_docs <- list.files(
  "docs",
  full.names = TRUE,
  pattern = "*.qmd"
)

for (doc in all_docs) {
  out <- litedown::mark(doc)

  read_html(out) |>
    html_node("body") |>
    xml2::write_html(file.path("src/docs", basename(out)))

  # delete the initially rendered html file
  file.remove(out)
}
