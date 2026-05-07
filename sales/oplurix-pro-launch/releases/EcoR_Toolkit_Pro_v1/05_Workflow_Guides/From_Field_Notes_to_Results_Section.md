# From Field Notes To Results Section

## 1. Digitize Field Notes

Enter every observation into `transect_observations_template.csv`.

Use consistent species names and site names. Do not mix `Foret Primaire`, `foret_primaire`, and `Foret_Primaire` for the same site.

## 2. Clean The Dataset

Run:

```r
source("02_R_Scripts/11_required_packages.R")
source("02_R_Scripts/02_import_clean_data.R")
```

Check missing values and suspicious species names.

## 3. Build The Community Matrix

Run:

```r
source("02_R_Scripts/03_community_matrix_vegan.R")
```

This creates the site x species matrix required by `vegan`.

## 4. Calculate Diversity Metrics

Run:

```r
source("02_R_Scripts/04_diversity_indices.R")
```

Use these values in your Results section and tables.

## 5. Compare Sites

Use NMDS and PERMANOVA to examine whether community composition differs between habitat types.

Run:

```r
site_metadata <- readr::read_csv("04_Data_Templates/site_metadata_template.csv")
source("02_R_Scripts/06_nmds_permanova.R")
```

## 6. Export Figures

Run the plotting scripts:

```r
source("02_R_Scripts/07_richness_barplot.R")
source("02_R_Scripts/08_abundance_heatmap.R")
source("02_R_Scripts/05_rarefaction_inext.R")
```

Figures are saved in `figures_output/`.

## 7. Draft The Results Section

Use prompt P42 from the prompt library. Paste:

- `diversity_indices`
- PERMANOVA or ANOSIM output
- NMDS stress value
- Figure names

Ask Claude to write a factual Results section without over-interpreting.

