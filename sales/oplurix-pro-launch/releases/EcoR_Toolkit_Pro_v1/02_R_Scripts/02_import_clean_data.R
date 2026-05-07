# EcoR Toolkit - Script 02
# Import and clean biodiversity field data

library(readr)
library(dplyr)
library(stringr)
library(lubridate)

input_file <- "04_Data_Templates/transect_observations_template.csv"

transect_data <- read_csv(input_file, show_col_types = FALSE) %>%
  janitor::clean_names() %>%
  mutate(
    site = str_squish(site),
    species = str_replace_all(species, "\\s+", "_"),
    species = str_squish(species),
    date = ymd(date),
    abondance = as.numeric(abondance),
    longueur_m = as.numeric(longueur_m)
  )

cat("Rows imported:", nrow(transect_data), "\n")
cat("Sites:", n_distinct(transect_data$site), "\n")
cat("Species:", n_distinct(transect_data$species), "\n")

missing_report <- transect_data %>%
  summarise(across(everything(), ~ sum(is.na(.x)))) %>%
  tidyr::pivot_longer(everything(), names_to = "column", values_to = "missing_values")

print(missing_report)

