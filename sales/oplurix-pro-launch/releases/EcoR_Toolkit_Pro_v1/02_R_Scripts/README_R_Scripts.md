# R Scripts Overview

Run `01_analyse_transects_biodiversite.R` first if you want the complete example workflow.

The other scripts are modular versions of the same workflow:

1. `02_import_clean_data.R` - import and clean field data
2. `03_community_matrix_vegan.R` - create a site x species matrix
3. `04_diversity_indices.R` - calculate richness and diversity indices
4. `05_rarefaction_inext.R` - generate rarefaction curves
5. `06_nmds_permanova.R` - run NMDS and PERMANOVA
6. `07_richness_barplot.R` - create a richness barplot
7. `08_abundance_heatmap.R` - create a species abundance heatmap
8. `09_environment_correlation.R` - explore diversity-environment correlations
9. `10_results_export.R` - export result tables and session info

Some scripts depend on objects created by earlier scripts. The easiest order is numeric order.

