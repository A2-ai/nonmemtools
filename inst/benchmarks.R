rextendr::document()
devtools::load_all(".")
bench::mark(
  res <- tibble::as_tibble(parse_ext_file_impl("src/rust/tests/fixtures/CONTROL5-NOCOV/CONTROL5.ext"))
)
res
