# EcoR Toolkit - Script 10
# Export tables and figures for reports or manuscripts

library(dplyr)

dir.create("results_output", showWarnings = FALSE)
dir.create("figures_output", showWarnings = FALSE)

if (exists("diversity_indices")) {
  write.csv(diversity_indices, "results_output/diversity_indices.csv", row.names = FALSE)
}

if (exists("richness_by_site")) {
  write.csv(richness_by_site, "results_output/richness_by_site.csv", row.names = FALSE)
}

if (exists("permanova_result")) {
  capture.output(permanova_result, file = "results_output/permanova_result.txt")
}

session_info <- capture.output(sessionInfo())
writeLines(session_info, "results_output/r_session_info.txt")

cat("Export complete. Check results_output/ and figures_output/.\n")

