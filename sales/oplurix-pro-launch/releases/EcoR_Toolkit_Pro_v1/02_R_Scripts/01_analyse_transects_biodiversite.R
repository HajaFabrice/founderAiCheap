# =============================================================================
# ANALYSE DE TRANSECTS BIODIVERSITE AVEC R
# Généré avec l'aide de Claude — Haja Fabrice RAZAFINDRABE MAMINIAINA
# Madagascar, 2026
# =============================================================================
# Ce script couvre :
#   1. Import et nettoyage des données
#   2. Statistiques descriptives
#   3. Indices de diversité (Shannon, Simpson, Chao1)
#   4. Courbes d'accumulation d'espèces
#   5. Comparaison entre sites
#   6. Visualisations publication-ready avec ggplot2
#   7. Export des figures
# =============================================================================


# ---- 0. PACKAGES REQUIS -----------------------------------------------------
# Copiez ce bloc et demandez à Claude :
# "J'ai une erreur sur cette ligne, aide-moi à la corriger"

required_packages <- c("vegan", "ggplot2", "dplyr", "tidyr", "tibble",
                       "iNEXT", "readr", "RColorBrewer", "ggrepel",
                       "patchwork", "scales")

# Installation automatique si manquant
new_packages <- required_packages[!(required_packages %in% installed.packages()[,"Package"])]
if(length(new_packages)) install.packages(new_packages)

lapply(required_packages, library, character.only = TRUE)


# ---- 1. CRÉATION DES DONNÉES EXEMPLE ----------------------------------------
# Remplacez cette section par : read.csv("vos_donnees.csv")
# Ou demandez à Claude : "Génère-moi un code pour importer mes données depuis Excel"

set.seed(42)  # Pour reproductibilité

# Simulation de données de transects reptiles/amphibiens — Madagascar
species_list <- c(
  "Zonosaurus_karsteni", "Zonosaurus_laticaudatus", "Furcifer_lateralis",
  "Brookesia_superciliaris", "Calumma_parsonii", "Uroplatus_fimbriatus",
  "Gephyromantis_boulengeri", "Mantella_aurantiaca", "Boophis_tephraeomystax",
  "Ptychadena_mascareniensis", "Heterixalus_tricolor", "Cophyla_phyllodactyla"
)

sites <- c("Foret_Ihofa", "Lisiere_Nord", "Lisiere_Sud",
           "Zone_Degradee", "Foret_Primaire")

# Génération du jeu de données
transect_data <- expand.grid(
  site        = sites,
  transect_id = 1:5,
  species     = species_list
) %>%
  mutate(
    abondance = rpois(n(), lambda = ifelse(
      site == "Foret_Primaire", 4,
      ifelse(site %in% c("Foret_Ihofa", "Lisiere_Nord"), 2, 0.8)
    )),
    longueur_m    = 200,
    date          = as.Date("2024-07-01") + sample(0:60, n(), replace = TRUE),
    temperature_c = round(rnorm(n(), mean = 24, sd = 3), 1),
    heure_debut   = paste0(sample(6:10, n(), replace = TRUE), "h00")
  )

cat("Données chargées :", nrow(transect_data), "observations\n")
head(transect_data, 8)


# ---- 2. NETTOYAGE ET PRÉPARATION --------------------------------------------
# Prompt Claude utile :
# "Voici la structure de mon dataframe : [str(transect_data)]
#  Je veux créer une matrice site x espèce. Génère le code."

# Filtrer les absences et agréger
obs_positives <- transect_data %>%
  filter(abondance > 0) %>%
  group_by(site, species) %>%
  summarise(total = sum(abondance), .groups = "drop")

# Matrice communauté (sites x espèces) — format requis par vegan
comm_matrix <- obs_positives %>%
  pivot_wider(names_from = species, values_from = total, values_fill = 0) %>%
  column_to_rownames("site")

cat("\nMatrice communauté :", nrow(comm_matrix), "sites x",
    ncol(comm_matrix), "espèces\n")


# ---- 3. STATISTIQUES DESCRIPTIVES -------------------------------------------

# Richesse spécifique par site
richesse <- transect_data %>%
  filter(abondance > 0) %>%
  group_by(site) %>%
  summarise(
    nb_especes      = n_distinct(species),
    nb_individus    = sum(abondance),
    nb_transects    = n_distinct(transect_id),
    densite_moy     = round(sum(abondance) / (n_distinct(transect_id) * 200) * 100, 2),
    .groups = "drop"
  ) %>%
  arrange(desc(nb_especes))

cat("\n=== RICHESSE PAR SITE ===\n")
print(richesse)


# ---- 4. INDICES DE DIVERSITE ------------------------------------------------
# Prompt Claude utile :
# "Explique-moi la différence entre Shannon et Simpson
#  et lequel utiliser pour comparer mes 5 sites forestiers"

diversity_indices <- data.frame(
  site     = rownames(comm_matrix),
  Shannon  = round(diversity(comm_matrix, index = "shannon"), 3),
  Simpson  = round(diversity(comm_matrix, index = "simpson"), 3),
  InvSimp  = round(diversity(comm_matrix, index = "invsimpson"), 3),
  Richesse = specnumber(comm_matrix),
  Pielou_J = round(diversity(comm_matrix) / log(specnumber(comm_matrix)), 3)
)

cat("\n=== INDICES DE DIVERSITE ===\n")
print(diversity_indices)

# Estimation Chao1 (richesse estimée incluant espèces non détectées)
chao1 <- estimateR(comm_matrix)
cat("\n=== ESTIMATION CHAO1 (richesse estimée) ===\n")
print(round(t(chao1), 2))


# ---- 5. COURBES D'ACCUMULATION D'ESPECES ------------------------------------
# Prompt Claude utile :
# "Mon code iNEXT renvoie une erreur : [ERREUR].
#  Voici mes données : [str(ma_liste)]. Qu'est-ce qui cloche ?"

# Préparer les données pour iNEXT (liste d'abondances par site)
abond_list <- lapply(sites, function(s) {
  transect_data %>%
    filter(site == s, abondance > 0) %>%
    group_by(species) %>%
    summarise(total = sum(abondance), .groups = "drop") %>%
    pull(total)
})
names(abond_list) <- sites

# Raréfaction et extrapolation
inext_result <- iNEXT(abond_list, q = 0, datatype = "abundance",
                      nboot = 50, se = TRUE)

cat("\n=== COURBES D'ACCUMULATION CALCULÉES ===\n")
cat("Résumé iNEXT :\n")
print(inext_result$AsyEst)


# ---- 6. COMPARAISON ENTRE SITES — NMDS --------------------------------------
# Ordination pour visualiser la similarité des communautés

nmds <- metaMDS(comm_matrix, distance = "bray", k = 2,
                trymax = 100, trace = FALSE)

cat("\n=== NMDS ===\n")
cat("Stress :", round(nmds$stress, 4),
    ifelse(nmds$stress < 0.1, "(excellent)", ifelse(nmds$stress < 0.2, "(bon)", "(acceptable)")), "\n")

nmds_coords <- as.data.frame(scores(nmds)) %>%
  mutate(site = rownames(.),
         type = case_when(
           site == "Foret_Primaire" ~ "Forêt primaire",
           site %in% c("Foret_Ihofa", "Lisiere_Nord", "Lisiere_Sud") ~ "Forêt secondaire",
           TRUE ~ "Zone dégradée"
         ))


# ---- 7. TEST STATISTIQUE ANOSIM (comparaison entre types) -------------------
groupes <- c("Forêt secondaire", "Forêt secondaire", "Forêt secondaire",
             "Zone dégradée", "Forêt primaire")

anosim_result <- anosim(comm_matrix, grouping = groupes,
                        permutations = 999, distance = "bray")

cat("\n=== TEST ANOSIM ===\n")
cat("R =", round(anosim_result$statistic, 3),
    "| p =", anosim_result$signif, "\n")
cat("Interprétation :", ifelse(anosim_result$signif < 0.05,
    "Les communautés diffèrent significativement entre types de forêt.",
    "Pas de différence significative détectée."), "\n")


# =============================================================================
# ---- 8. VISUALISATIONS GGPLOT2 ----------------------------------------------
# =============================================================================

# Palette daltonien-friendly pour Madagascar
palette_sites <- c(
  "Foret_Ihofa"    = "#1D9E75",
  "Lisiere_Nord"   = "#5DCAA5",
  "Lisiere_Sud"    = "#9FE1CB",
  "Zone_Degradee"  = "#D85A30",
  "Foret_Primaire" = "#0F6E56"
)

theme_terrain <- theme_minimal(base_size = 12) +
  theme(
    plot.title    = element_text(face = "bold", size = 13),
    plot.subtitle = element_text(color = "grey50", size = 10),
    axis.text     = element_text(size = 9),
    legend.title  = element_text(size = 10, face = "bold"),
    panel.grid.minor = element_blank()
  )


# --- Figure 1 : Richesse spécifique par site ----------------------------------
p1 <- richesse %>%
  ggplot(aes(x = reorder(site, nb_especes), y = nb_especes, fill = site)) +
  geom_col(width = 0.65) +
  geom_text(aes(label = nb_especes), hjust = -0.3, size = 3.5, fontface = "bold") +
  coord_flip() +
  scale_fill_manual(values = palette_sites) +
  scale_y_continuous(expand = expansion(mult = c(0, 0.15))) +
  labs(
    title    = "Richesse spécifique par site",
    subtitle = "Herpétofaune — Forêt Ihofa, Alaotra-Mangoro",
    x        = NULL,
    y        = "Nombre d'espèces",
    caption  = "Survey juillet–septembre 2024"
  ) +
  theme_terrain +
  theme(legend.position = "none")

print(p1)


# --- Figure 2 : Indices de diversité comparés ---------------------------------
p2 <- diversity_indices %>%
  pivot_longer(cols = c(Shannon, Simpson, Pielou_J),
               names_to = "index", values_to = "valeur") %>%
  ggplot(aes(x = reorder(site, valeur), y = valeur, fill = site)) +
  geom_col(width = 0.65) +
  coord_flip() +
  facet_wrap(~index, scales = "free_x",
             labeller = labeller(index = c(
               Shannon  = "Indice de Shannon (H')",
               Simpson  = "Indice de Simpson (D)",
               Pielou_J = "Équitabilité de Pielou (J)"
             ))) +
  scale_fill_manual(values = palette_sites) +
  labs(
    title    = "Indices de diversité par site",
    subtitle = "Shannon · Simpson · Équitabilité",
    x        = NULL, y        = "Valeur"
  ) +
  theme_terrain +
  theme(legend.position = "none",
        strip.text = element_text(size = 9, face = "bold"))

print(p2)


# --- Figure 3 : Courbes de raréfaction (iNEXT) --------------------------------
p3 <- ggiNEXT(inext_result, type = 1, se = TRUE) +
  scale_color_manual(values = unname(palette_sites)) +
  scale_fill_manual(values = unname(palette_sites)) +
  labs(
    title    = "Courbes d'accumulation d'espèces",
    subtitle = "Raréfaction (trait plein) · Extrapolation (pointillés) · IC 95%",
    x        = "Nombre d'individus échantillonnés",
    y        = "Richesse spécifique",
    color    = "Site",
    fill     = "Site"
  ) +
  theme_terrain

print(p3)


# --- Figure 4 : NMDS — similarité des communautés ----------------------------
p4 <- ggplot(nmds_coords, aes(x = NMDS1, y = NMDS2, color = site)) +
  geom_point(size = 5) +
  geom_text_repel(aes(label = gsub("_", " ", site)),
                  size = 3, fontface = "italic", max.overlaps = 10) +
  scale_color_manual(values = palette_sites) +
  annotate("text", x = min(nmds_coords$NMDS1), y = max(nmds_coords$NMDS2),
           label = paste0("Stress = ", round(nmds$stress, 3)),
           hjust = 0, size = 3.5, color = "grey40") +
  labs(
    title    = "Ordination NMDS — Composition des communautés",
    subtitle = "Distance de Bray-Curtis · Points proches = communautés similaires",
    x        = "NMDS1", y = "NMDS2"
  ) +
  theme_terrain +
  theme(legend.position = "none")

print(p4)


# --- Figure 5 : Heatmap abondance espèces x sites ----------------------------
p5 <- obs_positives %>%
  mutate(species = gsub("_", " ", species)) %>%
  ggplot(aes(x = site, y = species, fill = log1p(total))) +
  geom_tile(color = "white", linewidth = 0.5) +
  scale_fill_gradient(low = "#E1F5EE", high = "#085041",
                      name = "log(n+1)") +
  scale_x_discrete(labels = function(x) gsub("_", "\n", x)) +
  labs(
    title    = "Abondance par espèce et par site",
    subtitle = "Échelle logarithmique · log(n+1)",
    x        = NULL, y = NULL
  ) +
  theme_terrain +
  theme(axis.text.y = element_text(face = "italic", size = 8),
        axis.text.x = element_text(size = 8))

print(p5)


# ---- 9. FIGURE COMBINÉE PUBLICATION (patchwork) ----------------------------
# Prompt Claude utile :
# "Combine mes 4 figures en une seule planche publication-ready
#  avec patchwork. Je veux 2 colonnes."

figure_finale <- (p1 | p4) / (p2) / (p3)

figure_finale + plot_annotation(
  title   = "Diversité de l'herpétofaune — Forêt Ihofa, Madagascar",
  subtitle = "Étude multi-approche · Juillet–Septembre 2024",
  caption  = "Données : RAZAFINDRABE MAMINIAINA H.F. & RAZAFINDRATSIMA O., UC Berkeley 2024",
  theme    = theme(
    plot.title    = element_text(face = "bold", size = 14),
    plot.subtitle = element_text(color = "grey40"),
    plot.caption  = element_text(size = 8, color = "grey50", hjust = 0)
  )
)


# ---- 10. EXPORT DES FIGURES -------------------------------------------------
# Prompt Claude utile :
# "Comment exporter mes figures en 300 DPI pour soumission au journal ?"

dir.create("figures_output", showWarnings = FALSE)

ggsave("figures_output/fig1_richesse.png",     p1, width = 8, height = 5, dpi = 300)
ggsave("figures_output/fig2_diversite.png",    p2, width = 10, height = 6, dpi = 300)
ggsave("figures_output/fig3_rarefaction.png",  p3, width = 8, height = 6, dpi = 300)
ggsave("figures_output/fig4_nmds.png",         p4, width = 7, height = 6, dpi = 300)
ggsave("figures_output/fig5_heatmap.png",      p5, width = 8, height = 7, dpi = 300)

cat("\nFigures exportées dans figures_output/ — prêtes pour publication.\n")


# ---- 11. EXPORT DES RÉSULTATS EN CSV ----------------------------------------
write.csv(richesse,          "richesse_par_site.csv",    row.names = FALSE)
write.csv(diversity_indices, "indices_diversite.csv",    row.names = FALSE)

cat("Tableaux exportés.\n")
cat("\nAnalyse complète terminée.\n")
cat("Pour toute question sur ce code, copiez l'erreur et demandez à Claude !\n")

# =============================================================================
# PROMPTS CLAUDE PRÊTS À L'EMPLOI
# =============================================================================
#
# DÉBOGUER UNE ERREUR :
# "Voici mon script R et l'erreur que j'obtiens :
#  [coller l'erreur]
#  Voici les premières lignes de mes données :
#  [coller head(data)]
#  Qu'est-ce qui cause cette erreur et comment la corriger ?"
#
# INTERPRÉTER LES RÉSULTATS :
# "Shannon H' = 2.14 pour Foret_Primaire et 1.87 pour Zone_Degradee.
#  ANOSIM R = 0.72, p = 0.003.
#  Aide-moi à rédiger la section Results pour un article scientifique."
#
# AMÉLIORER UNE FIGURE :
# "Voici mon code ggplot2 : [coller le code].
#  Je veux ajouter des lettres de signification statistique (a, b, ab)
#  au-dessus des barres. Modifie le code."
#
# CHOISIR LE BON TEST STATISTIQUE :
# "J'ai 5 sites, données d'abondance d'espèces, non-normales.
#  Je veux tester si les sites diffèrent en richesse.
#  Quel test utiliser ? Génère le code R complet."
# =============================================================================
