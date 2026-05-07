# EcoR Toolkit - Script 08
# Species abundance heatmap

library(dplyr)
library(ggplot2)

heatmap_data <- transect_data %>%
  filter(abondance > 0) %>%
  group_by(site, species) %>%
  summarise(total_abundance = sum(abondance), .groups = "drop")

abundance_heatmap <- ggplot(heatmap_data, aes(x = site, y = species, fill = log1p(total_abundance))) +
  geom_tile(color = "white", linewidth = 0.4) +
  scale_fill_gradient(low = "#EAF7F0", high = "#116149", name = "log(n + 1)") +
  theme_minimal(base_size = 11) +
  theme(
    axis.text.x = element_text(angle = 45, hjust = 1),
    axis.text.y = element_text(face = "italic")
  ) +
  labs(
    title = "Species abundance by site",
    x = NULL,
    y = NULL
  )

print(abundance_heatmap)

dir.create("figures_output", showWarnings = FALSE)
ggsave("figures_output/abundance_heatmap.png", abundance_heatmap, width = 8, height = 7, dpi = 300)

