# EcoR Toolkit - Script 05
# Species rarefaction and extrapolation with iNEXT

library(dplyr)
library(iNEXT)
library(ggplot2)

# Required columns in transect_data: site, species, abondance

abundance_list <- transect_data %>%
  filter(abondance > 0) %>%
  group_by(site, species) %>%
  summarise(total = sum(abondance), .groups = "drop") %>%
  group_split(site) %>%
  setNames(unique(transect_data$site)) %>%
  lapply(function(df) df$total)

inext_result <- iNEXT(
  abundance_list,
  q = c(0, 1, 2),
  datatype = "abundance",
  nboot = 50
)

print(inext_result$AsyEst)

rarefaction_plot <- ggiNEXT(inext_result, type = 1, se = TRUE) +
  theme_minimal(base_size = 12) +
  labs(
    title = "Species accumulation curves",
    x = "Number of individuals",
    y = "Diversity estimate"
  )

print(rarefaction_plot)

dir.create("figures_output", showWarnings = FALSE)
ggsave("figures_output/rarefaction_inext.png", rarefaction_plot, width = 8, height = 6, dpi = 300)

