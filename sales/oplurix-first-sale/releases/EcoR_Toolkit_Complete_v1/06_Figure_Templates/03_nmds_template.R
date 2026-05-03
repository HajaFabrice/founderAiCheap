# EcoR Toolkit - NMDS figure template

library(ggplot2)
library(ggrepel)

source("06_Figure_Templates/01_ecor_theme.R")

ggplot(nmds_scores, aes(NMDS1, NMDS2, color = habitat_type)) +
  geom_point(size = 4) +
  geom_text_repel(aes(label = site), size = 3) +
  theme_ecor_publication() +
  labs(
    title = "Community composition by habitat",
    x = "NMDS1",
    y = "NMDS2",
    color = "Habitat"
  )

