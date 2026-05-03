# EcoR Toolkit - Script 03
# Create a site x species community matrix for vegan

library(dplyr)
library(tidyr)
library(tibble)

# Required columns: site, species, abondance

community_matrix <- transect_data %>%
  filter(!is.na(site), !is.na(species), !is.na(abondance)) %>%
  group_by(site, species) %>%
  summarise(total_abundance = sum(abondance), .groups = "drop") %>%
  pivot_wider(
    names_from = species,
    values_from = total_abundance,
    values_fill = 0
  ) %>%
  column_to_rownames("site")

community_matrix <- as.data.frame(community_matrix)

cat("Community matrix dimensions:\n")
print(dim(community_matrix))

