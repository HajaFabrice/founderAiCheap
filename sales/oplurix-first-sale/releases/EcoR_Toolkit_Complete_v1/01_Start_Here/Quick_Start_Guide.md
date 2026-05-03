# EcoR Toolkit Quick Start Guide

## The Fastest Way To Use This Toolkit

This guide gets you from field data to first results in about one afternoon.

You need:

- R
- RStudio
- Your biodiversity observations in CSV format
- Internet access for installing R packages the first time
- Claude, ChatGPT, or another AI assistant for prompt-based debugging

---

## Step 1: Prepare Your Data

Start with `04_Data_Templates/transect_observations_template.csv`.

Minimum required columns:

- `site`
- `transect_id`
- `date`
- `species`
- `abondance`
- `longueur_m`

Rules:

- Use one row per species observation per transect.
- Keep species names consistent.
- Use `0` for confirmed absence only when you deliberately recorded absence.
- Use blank cells only when data is truly missing.
- Keep dates in `YYYY-MM-DD` format.

---

## Step 2: Run The Example Script

Open:

`02_R_Scripts/01_analyse_transects_biodiversite.R`

Run the script once with the example data. This confirms that packages install correctly and that your R setup works.

The script produces:

- Species richness by site
- Shannon, Simpson, inverse Simpson, Pielou, and Chao1 indices
- Rarefaction curves
- NMDS ordination
- ANOSIM test
- Publication-ready ggplot2 figures
- CSV result tables

---

## Step 3: Replace Example Data With Your Data

Find the section:

```r
# ---- 1. CREATION DES DONNEES EXEMPLE ----------------------------------------
```

Replace it with:

```r
transect_data <- readr::read_csv("your_data.csv")
```

Then check:

```r
str(transect_data)
head(transect_data)
```

If your column names differ, either rename them in your CSV or adapt the script.

---

## Step 4: Debug With The Prompt Library

Open:

`03_AI_Prompts/50_Prompts_Claude_Biodiversite_R.md`

Use:

- P02 for messy CSV cleaning
- P03 for creating a site by species matrix
- P11-P20 for R errors
- P22-P30 for statistical analyses
- P31-P40 for figures
- P41-P50 for manuscript writing

Best practice: paste the exact R error, your code, and `str(your_data)`.

---

## Step 5: Export Your Results

The script automatically creates:

- `figures_output/`
- `richesse_par_site.csv`
- `indices_diversite.csv`

Use the figures in presentations, reports, or manuscript drafts. Always verify results with your supervisor or statistician before publication.

---

## One Afternoon Workflow

Hour 1: Clean your CSV and confirm column names.  
Hour 2: Run richness and diversity indices.  
Hour 3: Generate figures and fix visual issues.  
Hour 4: Use prompts P41-P46 to draft Methods, Results, and figure captions.

---

## Important Scientific Note

This toolkit accelerates analysis, but it does not replace ecological judgment. Confirm that the statistical method fits your sampling design, detection process, and research question.

