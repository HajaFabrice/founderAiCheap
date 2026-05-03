# EcoR Toolkit - Script 06
# NMDS ordination and PERMANOVA

library(vegan)
library(ggplot2)
library(ggrepel)
library(dplyr)

# Input required:
# community_matrix: sites x species
# site_metadata: dataframe with columns site and habitat_type

nmds <- metaMDS(community_matrix, distance = "bray", k = 2, trymax = 100, trace = FALSE)

nmds_scores <- as.data.frame(scores(nmds, display = "sites")) %>%
  tibble::rownames_to_column("site") %>%
  left_join(site_metadata, by = "site")

permanova_result <- adonis2(
  community_matrix ~ habitat_type,
  data = site_metadata,
  method = "bray",
  permutations = 999
)

print(permanova_result)

nmds_plot <- ggplot(nmds_scores, aes(NMDS1, NMDS2, color = habitat_type)) +
  geom_point(size = 4) +
  geom_text_repel(aes(label = site), size = 3) +
  annotate("text", x = -Inf, y = Inf, hjust = -0.1, vjust = 1.2,
           label = paste("Stress =", round(nmds$stress, 3))) +
  theme_minimal(base_size = 12) +
  labs(title = "NMDS ordination", color = "Habitat")

print(nmds_plot)

dir.create("figures_output", showWarnings = FALSE)
ggsave("figures_output/nmds_permanova.png", nmds_plot, width = 7, height = 6, dpi = 300)

