# EcoR Toolkit - Diversity barplot template

library(ggplot2)
library(dplyr)

source("06_Figure_Templates/01_ecor_theme.R")

ggplot(diversity_indices, aes(x = reorder(site, shannon), y = shannon)) +
  geom_col(fill = palette_ecor["forest"], width = 0.65) +
  geom_text(aes(label = round(shannon, 2)), hjust = -0.25, fontface = "bold") +
  coord_flip() +
  scale_y_continuous(expand = expansion(mult = c(0, 0.15))) +
  theme_ecor_publication() +
  labs(
    title = "Shannon diversity by site",
    x = NULL,
    y = "Shannon diversity"
  )

