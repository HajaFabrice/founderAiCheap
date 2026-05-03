# EcoR Toolkit - Script 04
# Calculate common biodiversity indices

library(vegan)
library(dplyr)

# Input required: community_matrix, with sites as rows and species as columns

diversity_indices <- data.frame(
  site = rownames(community_matrix),
  richness_observed = specnumber(community_matrix),
  shannon = diversity(community_matrix, index = "shannon"),
  simpson = diversity(community_matrix, index = "simpson"),
  inverse_simpson = diversity(community_matrix, index = "invsimpson")
) %>%
  mutate(
    pielou_evenness = shannon / log(richness_observed),
    across(where(is.numeric), ~ round(.x, 3))
  )

chao1_estimates <- estimateR(community_matrix) %>%
  t() %>%
  as.data.frame()

print(diversity_indices)
print(round(chao1_estimates, 2))

write.csv(diversity_indices, "indices_diversite.csv", row.names = FALSE)

