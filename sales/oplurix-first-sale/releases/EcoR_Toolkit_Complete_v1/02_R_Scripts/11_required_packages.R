# EcoR Toolkit - Required packages installer

required_packages <- c(
  "vegan", "ggplot2", "dplyr", "tidyr", "tibble", "readr", "janitor",
  "lubridate", "stringr", "iNEXT", "RColorBrewer", "ggrepel",
  "patchwork", "scales"
)

new_packages <- required_packages[!(required_packages %in% installed.packages()[, "Package"])]

if (length(new_packages) > 0) {
  install.packages(new_packages)
}

invisible(lapply(required_packages, library, character.only = TRUE))

cat("EcoR Toolkit packages loaded.\n")

