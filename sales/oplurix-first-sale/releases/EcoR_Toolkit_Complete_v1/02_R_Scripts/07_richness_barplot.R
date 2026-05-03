# EcoR Toolkit - Script 07
# Publication-ready richness barplot

library(dplyr)
library(ggplot2)

richness_by_site <- transect_data %>%
  filter(abondance > 0) %>%
  group_by(site, habitat_type) %>%
  summarise(
    richness = n_distinct(species),
    total_individuals = sum(abondance),
    .groups = "drop"
  )

richness_plot <- ggplot(richness_by_site, aes(x = reorder(site, richness), y = richness, fill = habitat_type)) +
  geom_col(width = 0.65) +
  geom_text(aes(label = richness), hjust = -0.25, fontface = "bold") +
  coord_flip() +
  scale_y_continuous(expand = expansion(mult = c(0, 0.15))) +
  theme_minimal(base_size = 12) +
  labs(
    title = "Species richness by site",
    x = NULL,
    y = "Number of species",
    fill = "Habitat"
  )

print(richness_plot)

dir.create("figures_output", showWarnings = FALSE)
ggsave("figures_output/richness_by_site.png", richness_plot, width = 8, height = 5, dpi = 300)

