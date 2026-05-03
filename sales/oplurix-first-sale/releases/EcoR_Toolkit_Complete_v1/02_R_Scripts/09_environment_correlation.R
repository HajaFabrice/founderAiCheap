# EcoR Toolkit - Script 09
# Correlate biodiversity indices with environmental variables

library(dplyr)
library(ggplot2)

# Required:
# diversity_indices: output from diversity script
# environmental_data: dataframe with site plus numeric environmental variables

analysis_data <- diversity_indices %>%
  left_join(environmental_data, by = "site")

numeric_data <- analysis_data %>%
  select(where(is.numeric))

correlation_matrix <- cor(numeric_data, use = "pairwise.complete.obs", method = "spearman")

print(round(correlation_matrix, 2))

example_plot <- ggplot(analysis_data, aes(x = canopy_cover_pct, y = shannon)) +
  geom_point(size = 3) +
  geom_smooth(method = "lm", se = TRUE) +
  theme_minimal(base_size = 12) +
  labs(
    title = "Shannon diversity vs canopy cover",
    x = "Canopy cover (%)",
    y = "Shannon diversity"
  )

print(example_plot)

dir.create("figures_output", showWarnings = FALSE)
ggsave("figures_output/shannon_canopy_correlation.png", example_plot, width = 7, height = 5, dpi = 300)

