suppressPackageStartupMessages({
  library(data.table)
  library(jsonlite)
  library(poorman)
})

all_files <- list.files("results", pattern = "\\.json$", full.names = TRUE)

all_results <- lapply(all_files, \(x) {
  results_json <- jsonlite::read_json(x)
  name <- basename(x)
  repo <- sub("^([^_]+)_([^_]+)_.*$", "\\1/\\2", name)
  type <- sub("^([^_]+)_([^_]+)_.*$", "\\3", name)
  
  lapply(results_json, \(x) {
    data.frame(
      repo = repo,
      type = type,
      name = x$message$name,
      filename = x$filename,
      row = x$location$row,
      column = x$location$column
    )
  }) 
})|>
    rbindlist()

main_results <- lapply(main_results_json, \(x) {
  data.frame(
    name = x$message$name,
    filename = x$filename,
    row = x$location$row,
    column = x$location$column
  )
}) |>
  rbindlist()

branch_results <- lapply(branch_results_json, \(x) {
  data.frame(
    name = x$message$name,
    filename = x$filename,
    row = x$location$row,
    column = x$location$column
  )
}) |>
  rbindlist()

new_lints <- anti_join(
  branch_results,
  main_results,
  by = c("name", "filename", "row", "column")
) |>
  nrow()

deleted_lints <- anti_join(
  main_results,
  branch_results,
  by = c("name", "filename", "row", "column")
) |>
  nrow()

paste0("**dplyr**: +", new_lints, " -", deleted_lints, " violations") |>
  writeLines("lint_comparison.md")
