# 🔬 50 Prompts Claude — Analyse de Données Biodiversité avec R
## Prêts à copier-coller · Par Haja Fabrice RAZAFINDRABE MAMINIAINA

> **Mode d'emploi :** Copiez le prompt, remplacez les parties entre [CROCHETS]
> par vos vraies données ou informations, puis envoyez à Claude.

---

## 📥 CATÉGORIE 1 — IMPORT & NETTOYAGE DES DONNÉES (10 prompts)

---

**P01 — Importer un fichier Excel avec plusieurs feuilles**
```
Tu es un expert R en écologie des communautés.
Contexte : J'ai un fichier Excel "transects_2024.xlsx" avec 3 feuilles :
"Reptiles", "Amphibiens", "Variables_env".
Chaque feuille a ces colonnes : site, date, espece, nb_individus, observateur.
Tâche : Écris le code R complet pour importer les 3 feuilles,
les fusionner en un seul dataframe, et vérifier qu'il n'y a pas de doublons.
Format : Code commenté, avec messages de confirmation à chaque étape.
```

---

**P02 — Diagnostiquer et nettoyer un CSV brut de terrain**
```
Tu es un biostatisticien spécialisé en données écologiques.
Voici les premières lignes de mon CSV de terrain :
[COLLER : head(read.csv("votre_fichier.csv"), 20)]

Tâche : Génère le code R pour :
1. Détecter les valeurs manquantes (NA) par colonne
2. Identifier les valeurs aberrantes dans les colonnes numériques
3. Standardiser les noms d'espèces (supprimer espaces, uniformiser casse)
4. Convertir les dates au format standard Date
5. Créer un rapport de nettoyage en console
Format : Code avec commentaires et summary() final.
```

---

**P03 — Créer une matrice communauté site × espèce**
```
Tu es un expert du package vegan en R.
Voici la structure de mon dataframe :
[COLLER : str(mon_dataframe)]

Les colonnes importantes sont : [NOM_COL_SITE], [NOM_COL_ESPECE], [NOM_COL_ABONDANCE].
Tâche : Écris le code pour créer une matrice communauté (sites en lignes,
espèces en colonnes, abondances comme valeurs) utilisable directement avec vegan.
Inclure : gestion des NA, vérification que toutes les valeurs sont numériques,
et un print des dimensions finales.
```

---

**P04 — Fusionner données biologiques et variables environnementales**
```
Tu es un spécialiste R en écologie.
J'ai deux dataframes :
- bio_data : colonnes site, espece, abondance (n=[NOMBRE] lignes)
- env_data : colonnes site, temperature, precipitation, altitude, couverture_forestiere

Tâche : Écris le code pour les fusionner correctement (left_join),
vérifier qu'aucun site n'est perdu, identifier les sites sans données
environnementales, et afficher un tableau récapitulatif du merge.
```

---

**P05 — Standardiser les noms d'espèces avec une liste de référence**
```
Tu es un taxonomiste et développeur R.
Contexte : Mes données terrain ont des variations orthographiques dans les noms
d'espèces : "Zonosaurus karsteni", "Z. karsteni", "zonosaurus Karsteni",
"Zonosaurus_karsteni", etc.
Ma liste de référence officielle est : [COLLER : votre_liste_especes]
Tâche : Génère le code R pour détecter automatiquement les variantes et
les standardiser vers le nom canonique. Utilise stringdist ou fuzzyjoin
si nécessaire. Affiche un tableau des corrections effectuées.
```

---

**P06 — Détecter et traiter les doublons de transects**
```
Tu es un expert en qualité de données écologiques.
Mon dataframe de transects a ces colonnes :
site, date, transect_id, observateur, espece, nb_individus

Tâche : Écris le code R pour :
1. Détecter les combinaisons site+date+transect_id+espece dupliquées
2. Afficher les doublons suspects
3. Proposer une règle de dédoublonnage (garder le max ? la somme ? le premier ?)
4. Appliquer la règle et confirmer le résultat.
```

---

**P07 — Importer et préparer des données de camera trap**
```
Tu es un spécialiste des données camera trap en R.
J'ai un export CSV de [CAMTRAP / TIMELAPSE / AUTRE LOGICIEL] avec ces colonnes :
[COLLER les noms de colonnes]
Contexte : [NOMBRE] événements de détection, [NOMBRE] caméras,
[NOMBRE] jours de déploiement, [LIEU].
Tâche : Génère le code R pour calculer :
- Taux d'occupation naïf par espèce
- Nombre de nuits-caméra effectives
- Événements indépendants (fenêtre de 30 minutes)
- Un dataframe propre prêt pour les analyses suivantes.
```

---

**P08 — Gérer les données de présence-absence depuis des transects linéaires**
```
Tu es expert en modèles d'occupancy en R.
Mes données de transects linéaires ont : site, date, espece, present (0/1),
longueur_transect_m, habitat_type.
[NOMBRE] sites, [NOMBRE] visites par site, [NOMBRE] espèces cibles.
Tâche : Transforme ces données en format matrice de détection
(sites × visites) utilisable avec le package unmarked pour un modèle
d'occupancy single-season. Inclure la gestion des visites manquantes (NA).
```

---

**P09 — Calculer l'effort d'échantillonnage par site**
```
Tu es biostatisticien spécialisé en biodiversité.
Mon dataframe a : site, date, transect_id, longueur_m, duree_min, nb_observateurs.
Tâche : Calcule l'effort d'échantillonnage standardisé pour chaque site :
- Longueur totale prospectée (m)
- Temps total d'observation (h)
- Nombre de transects
- Densité = individus / 100m de transect
Formate le résultat en tableau récapitulatif propre prêt à intégrer
dans la section Methods d'un article.
```

---

**P10 — Créer un jeu de données test quand les vraies données ne sont pas disponibles**
```
Tu es expert R et biologiste de terrain.
Je veux tester mon pipeline d'analyse avant d'avoir mes vraies données.
Contexte : Étude herpétofaune, [NOMBRE] sites, [NOMBRE] espèces,
[NOMBRE] transects par site, habitat [FORET PRIMAIRE / SECONDAIRE / DEGRADEE].
Tâche : Génère un jeu de données simulé réaliste avec des patterns écologiques
cohérents (richesse plus élevée en forêt primaire, espèces rares correctement
représentées, variabilité inter-transects réaliste).
Utilise set.seed(42) pour la reproductibilité.
```

---

## 🐛 CATÉGORIE 2 — DÉBOGAGE D'ERREURS R (10 prompts)

---

**P11 — Diagnostic d'erreur général**
```
Tu es un expert R senior. J'ai une erreur que je ne comprends pas.

Mon code :
[COLLER VOTRE CODE COMPLET]

L'erreur exacte :
[COLLER LE MESSAGE D'ERREUR COMPLET]

Structure de mes données :
[COLLER : str(mon_dataframe)]
[COLLER : head(mon_dataframe)]

Tâche : Explique pourquoi cette erreur se produit en termes simples,
puis donne le code corrigé avec une explication de la correction.
```

---

**P12 — Erreur "object not found" ou "could not find function"**
```
Tu es un expert R. J'ai cette erreur :
Error: object '[NOM]' not found
ou
Error: could not find function "[NOM_FONCTION]"

Mon environment actuel : [COLLER : ls()]
Mes packages chargés : [COLLER : (.packages())]

Tâche : Identifie la cause probable (faute de frappe, package non chargé,
ordre d'exécution, portée de variable) et propose le code corrigé.
```

---

**P13 — Erreur dans vegan (diversity, metaMDS, etc.)**
```
Tu es un expert du package vegan en R.
J'utilise la fonction [diversity / metaMDS / adonis2 / anosim / autre]
et j'obtiens cette erreur :
[COLLER L'ERREUR EXACTE]

Mon code :
[COLLER VOTRE CODE]

Voici les dimensions et le type de ma matrice communauté :
dim(comm) = [RÉSULTAT]
class(comm) = [RÉSULTAT]
sapply(comm, class) = [RÉSULTAT ABRÉGÉ]

Tâche : Diagnostique l'erreur, explique ce que vegan attend exactement,
et donne le code de correction avec vérification.
```

---

**P14 — Problème de types de colonnes (character vs numeric vs factor)**
```
Tu es un expert R en manipulation de données.
Mon code plante car une colonne est du mauvais type.

Erreur : [COLLER L'ERREUR]
Résultat de str(mon_data) : [COLLER]

Tâche : Identifie quelles colonnes ont le mauvais type pour mon analyse,
génère le code de conversion avec as.numeric(), as.factor(), as.Date(),
et ajoute des vérifications avec stopifnot() pour prévenir le problème
à l'avenir.
```

---

**P15 — Figure ggplot2 qui ne s'affiche pas ou plante**
```
Tu es un expert ggplot2.
Mon code ne produit pas la figure attendue :

Code ggplot2 :
[COLLER VOTRE CODE GGPLOT2 COMPLET]

Erreur ou comportement inattendu :
[DÉCRIRE CE QUI SE PASSE : erreur / figure vide / mauvaises couleurs / etc.]

Structure des données utilisées :
[COLLER : str(mes_donnees)]

Tâche : Identifie le problème, explique pourquoi ggplot2 se comporte ainsi,
et donne le code corrigé.
```

---

**P16 — Package iNEXT : erreur dans les courbes de raréfaction**
```
Tu es expert du package iNEXT en R.
J'utilise iNEXT() pour mes courbes d'accumulation d'espèces.
Erreur : [COLLER L'ERREUR]

Mon code :
[COLLER]

Format de mes données d'entrée :
[COLLER : str(ma_liste_abondances)]
Exemple pour un site : [COLLER : head(ma_liste[[1]])]

Tâche : Explique quel format exact attend iNEXT (vecteur d'abondances,
matrice d'incidence, ou liste), vérifie si mes données sont au bon format,
et génère le code de conversion + appel iNEXT corrigé.
```

---

**P17 — Problème de mémoire ou calcul trop lent**
```
Tu es expert R en optimisation de code.
Mon script est trop lent ou cause une erreur de mémoire :
[COLLER L'ERREUR ou DÉCRIRE LE PROBLÈME]

Contexte :
- Taille du dataframe : [NROW × NCOL]
- RAM disponible : [RAM EN GO]
- Opération lente : [DÉCRIRE CE QUE FAIT LE CODE]

Mon code actuel :
[COLLER]

Tâche : Propose des optimisations (data.table au lieu de dplyr,
calcul par chunks, parallélisation avec parallel ou future,
utilisation de matrices au lieu de dataframes) avec benchmarks estimés.
```

---

**P18 — Erreur lors d'une boucle sur plusieurs sites ou espèces**
```
Tu es expert R. Ma boucle for (ou lapply) plante sur certaines itérations.

Mon code :
[COLLER LA BOUCLE]

L'erreur se produit à : [DÉCRIRE QUAND — à quelle espèce, quel site, etc.]

Tâche : Ajoute un tryCatch() pour gérer les erreurs sans arrêter la boucle,
affiche un message informatif pour les cas problématiques,
et propose une alternative avec purrr::map() / safely() si pertinent.
```

---

**P19 — Problème de projection ou coordonnées GPS en R**
```
Tu es expert R en analyse spatiale (sf, terra).
J'ai des coordonnées GPS de mes transects qui causent un problème :
[DÉCRIRE LE PROBLÈME : mauvaise projection, points hors bbox, etc.]

Mes colonnes GPS : latitude ([EXEMPLES DE VALEURS]), longitude ([EXEMPLES])
Système de coordonnées attendu : [WGS84 / UTM zone X / autre]

Mon code actuel :
[COLLER]

Tâche : Diagnostique le problème de projection, génère le code pour
convertir correctement les coordonnées, et crée un sf object valide
pour cartographie.
```

---

**P20 — Interpréter un warning R (pas une erreur, mais inquiétant)**
```
Tu es expert R et statisticien en écologie.
Mon code tourne mais produit ces warnings :
[COLLER LES WARNINGS EXACTS]

Mon code :
[COLLER]

Tâche : Explique ce que signifie chaque warning en termes biologiques/écologiques
(pas seulement techniques), dis-moi si cela invalide mes résultats ou si je
peux ignorer, et si nécessaire propose une correction.
```

---

## 📊 CATÉGORIE 3 — ANALYSES STATISTIQUES (10 prompts)

---

**P21 — Choisir le bon test statistique**
```
Tu es biostatisticien expert en écologie des communautés.
Contexte de mon étude :
- Variable réponse : [richesse spécifique / indice Shannon / abondance / autre]
- Variable explicative : [type d'habitat / saison / gradient altitude / autre]
- Nombre de groupes : [2 / 3 / 4+ groupes]
- Distribution des données : [COLLER : shapiro.test(ma_variable)]
- Taille d'échantillon : [n =]
- Structure des données : [données indépendantes / répétées / spatiales]

Tâche : Recommande le test statistique le plus approprié avec justification,
liste les alternatives, et génère le code R complet avec interprétation
des résultats.
```

---

**P22 — Calculer tous les indices de diversité en une seule fois**
```
Tu es expert du package vegan.
J'ai une matrice communauté (comm) de [NOMBRE] sites × [NOMBRE] espèces.
[COLLER : head(comm, 3)] pour un aperçu.

Tâche : Génère le code R pour calculer en une seule pipeline :
Shannon (H'), Simpson (D), Simpson inverse (1/D), Pielou (J'),
Margalef, Chao1 estimé, ACE estimé, et richesse observée.
Formate le résultat en un tableau récapitulatif propre avec
les sites en lignes et les indices en colonnes.
```

---

**P23 — Modèle linéaire pour expliquer la diversité**
```
Tu es biostatisticien expert en GLM/LM en R.
Je veux tester quelles variables environnementales expliquent
la diversité (Shannon H') dans mes [NOMBRE] sites.

Variables disponibles : [température, précipitation, altitude,
couverture forestière, distance bord forêt, etc.]
Structure des données : [COLLER : str(mes_donnees_env)]

Tâche :
1. Teste la normalité de H' et propose transformation si nécessaire
2. Construit un modèle saturé puis sélection par AIC (stepwise)
3. Vérifie les résidus (4 graphiques diagnostiques)
4. Interprète les coefficients en termes biologiques
5. Génère le tableau de résultats publication-ready
```

---

**P24 — NMDS et visualisation de la composition des communautés**
```
Tu es expert vegan et ggplot2 en écologie.
Ma matrice communauté : [NOMBRE] sites × [NOMBRE] espèces.
[COLLER : dim(comm)]

Mes groupes de sites : [COLLER : factor_groupes]
(ex: forêt primaire, secondaire, dégradée)

Tâche : Génère le code complet pour :
1. NMDS avec distance de Bray-Curtis (k=2, trymax=100)
2. Test ANOSIM et PERMANOVA (adonis2) entre groupes
3. Figure ggplot2 publication-ready avec ellipses de confiance,
   labels des sites, et interprétation du stress value
4. Identifier les espèces qui contribuent le plus à la séparation (simper)
```

---

**P25 — Courbes de raréfaction et richesse estimée**
```
Tu es expert iNEXT et vegan.
Contexte : [NOMBRE] sites, données d'abondance.
[COLLER : str(abond_list)] — liste de vecteurs d'abondance par site.

Tâche : Génère le code pour :
1. Raréfaction basée sur l'abondance avec iNEXT (q=0, q=1, q=2)
2. Richesse asymptotique estimée par site
3. Figure ggiNEXT avec intervalles de confiance
4. Test si l'échantillonnage est suffisant (pente de la courbe < 0.1)
5. Tableau comparatif richesse observée vs estimée par site
```

---

**P26 — Analyse de la bêta-diversité**
```
Tu es expert en analyse de diversité beta en R.
Ma matrice communauté : [COLLER : dim(comm)]
Mes groupes : [COLLER]

Tâche : Décompose la bêta-diversité totale en ses composantes :
- Remplacement d'espèces (turnover)
- Différences de richesse (nestedness)
Utilise le package betapart.
Génère :
1. La décomposition pour toutes les paires de sites
2. Un heatmap de dissimilarité entre sites
3. Un dendrogramme de clustering hiérarchique
4. L'interprétation écologique des résultats
```

---

**P27 — Modèle d'occupancy simple avec unmarked**
```
Tu es expert des modèles d'occupancy en R (package unmarked).
J'ai des données de détection/non-détection pour [NOMBRE] espèces,
[NOMBRE] sites, [NOMBRE] visites.

Matrice de détection (extrait) :
[COLLER : head(y_matrix)]

Covariables de site : [COLLER : str(site_covs)]
Covariables d'observation : [COLLER : str(obs_covs)]

Tâche : Génère le code pour un modèle occu() single-season,
sélection de modèles par AIC, extraction de psi (occupancy) et p (détection),
et carte de probabilité d'occupancy par site.
```

---

**P28 — Analyse de la sélection d'habitat**
```
Tu es expert en analyses de sélection d'habitat en R.
J'ai [NOMBRE] observations de [ESPÈCE] avec coordonnées GPS,
et [NOMBRE] points aléatoires (background) pour la même zone.

Colonnes présence : [COLLER : str(presence_data)]
Variables environnementales raster disponibles :
[liste : altitude, pente, couverture végétale, distance eau, etc.]

Tâche : Génère un modèle de distribution (MaxEnt via dismo ou ENMeval,
ou GLM presence/absence avec points background),
carte de prédiction, et importance des variables.
```

---

**P29 — Test de corrélation entre diversité et variables environnementales**
```
Tu es biostatisticien expert R.
Je veux explorer les relations entre mes indices de diversité
et des variables environnementales continues.

Mon dataframe (extrait) :
[COLLER : head(mon_df[, c("site","shannon","simpson","richesse",
                           "temperature","altitude","pluviometrie")])]

Tâche :
1. Matrice de corrélation (Pearson ou Spearman selon normalité)
2. Corrélogramme visuel avec ggcorrplot ou corrplot
3. Régressions bivariales pour les corrélations significatives (p < 0.05)
4. Correction pour tests multiples (Bonferroni ou FDR)
5. Interprétation écologique des corrélations significatives
```

---

**P30 — Analyse temporelle : changements de biodiversité entre deux périodes**
```
Tu es expert en analyse de séries temporelles écologiques en R.
J'ai des données de [ESPÈCE / COMMUNAUTÉ] à [NOMBRE] sites
mesurées en [ANNÉE 1] et [ANNÉE 2 ou PLUS].

Structure : [COLLER : str(mes_donnees_temporelles)]

Tâche :
1. Calcule les changements d'indices (ΔShannon, Δrichesse) par site
2. Test de Wilcoxon pairé pour tester la significativité du changement
3. Identifie les sites qui ont le plus changé (gagnants vs perdants)
4. Figure avant/après avec lignes de connexion par site
5. Rédige une phrase de résultats publication-ready
```

---

## 🎨 CATÉGORIE 4 — VISUALISATION GGPLOT2 (10 prompts)

---

**P31 — Figure de richesse spécifique par site (barplot)**
```
Tu es expert ggplot2 et design scientifique.
Mon dataframe richesse : [COLLER : str(richesse_df)]
avec colonnes : site, nb_especes, type_habitat, nb_individus.

Tâche : Crée un barplot horizontal publication-ready avec :
- Barres ordonnées par richesse décroissante
- Couleurs selon type_habitat (palette daltonien-friendly)
- Valeurs affichées sur les barres
- Barres d'erreur si données disponibles
- Thème minimal sans fond gris
- Taille adaptée pour journal (width=8in, height=5in, 300dpi)
Exporte en PNG haute résolution dans un dossier "figures/".
```

---

**P32 — Courbe d'accumulation d'espèces multi-sites**
```
Tu es expert iNEXT et ggplot2.
Résultat iNEXT disponible : inext_out (objet de classe iNEXT)
[NOMBRE] sites, noms : [LISTE DES SITES]
Palette souhaitée : [daltonien-friendly / par type d'habitat / autre]

Tâche : Génère une figure ggiNEXT personnalisée avec :
- Trait plein pour raréfaction, pointillés pour extrapolation
- Intervalles de confiance 95% transparents
- Point vertical = taille d'échantillon observée
- Légende claire en dehors du graphique
- Titre, axes en italique pour noms scientifiques si nécessaire
- Thème publication (pas de fond gris, grille minimale)
```

---

**P33 — Heatmap abondance espèces × sites**
```
Tu es expert ggplot2 et visualisation de données de biodiversité.
Mon dataframe long : [COLLER : head(obs_df, 5)]
colonnes : site, espece, abondance

Tâche : Crée une heatmap avec :
- Espèces en lignes (noms en italique), sites en colonnes
- Échelle de couleur divergente ou séquentielle (log+1 si distributions skewed)
- Dendrogramme de clustering sur les lignes ET colonnes (ordre phylogénétique
  ou par similarité de composition)
- Annotations des groupes d'espèces si disponibles
- Taille de police adaptée pour [NOMBRE] espèces × [NOMBRE] sites
```

---

**P34 — Ordination NMDS publication-ready**
```
Tu es expert vegan et ggplot2.
Mes coordonnées NMDS : [COLLER : nmds_coords_df]
avec colonnes : NMDS1, NMDS2, site, type_habitat, region

Stress value : [VALEUR]

Tâche : Crée une figure NMDS avec :
- Points colorés par type_habitat, forme par region
- Ellipses de confiance à 95% par groupe (stat_ellipse)
- Labels des sites avec ggrepel (pas de chevauchement)
- Annotation du stress value en bas à gauche
- Flèches des espèces contribuantes (envfit, top 5)
- Thème minimal publication-ready
```

---

**P35 — Boxplot comparatif multi-sites avec tests statistiques**
```
Tu es expert ggplot2 et ggpubr.
Je veux comparer [Shannon / richesse / abondance] entre [NOMBRE] groupes.
Mon dataframe : [COLLER : str(df)]

Tests à effectuer : [Kruskal-Wallis + Dunn / ANOVA + Tukey / Wilcoxon]

Tâche : Génère un boxplot avec :
- Jitter des points individuels superposés
- Notches si n > 15 par groupe
- Lettres de signification statistique (a, b, ab) au-dessus des boîtes
  via package rstatix + ggpubr
- Couleurs cohérentes avec les autres figures (même palette)
- N par groupe indiqué sous chaque boîte
```

---

**P36 — Carte de distribution des observations**
```
Tu es expert cartographie R (sf, ggplot2, terra).
J'ai [NOMBRE] observations avec latitude et longitude.
[COLLER : head(mes_obs[, c("lat","lon","espece","site")])]
Zone d'étude : [MADAGASCAR / KENYA / CAMEROUN / autre pays/région]

Tâche : Crée une carte avec :
- Fond de carte OpenStreetMap ou relief SRTM (package geodata)
- Points colorés par espèce ou site, taille proportionnelle à l'abondance
- Contours des sites d'étude ou zones protégées si disponibles
- Inset de localisation dans le pays/continent
- Échelle et flèche Nord
- Format export adapté publication
```

---

**P37 — Figure composite multi-panneaux avec patchwork**
```
Tu es expert patchwork et ggplot2.
J'ai ces figures individuelles déjà créées :
- p1 : [barplot richesse]
- p2 : [NMDS]
- p3 : [courbes raréfaction]
- p4 : [boxplot diversité]

Disposition souhaitée : [2 colonnes × 2 lignes / 1 colonne / autre]

Tâche : Assemble en une seule figure publication-ready avec :
- Labels A, B, C, D en haut à gauche de chaque panneau
- Taille relative des panneaux ajustée (certains plus grands)
- Légende commune si les couleurs sont partagées
- Titre général et légende de figure complète
- Export à 300 DPI au format journal (max 180mm de large)
```

---

**P38 — Graphique corrélation variables environnementales**
```
Tu es expert ggplot2 et corrplot.
Mon dataframe environnemental : [COLLER : str(env_df)]

Variables numériques à corréler : [LISTE]
Variables catégorielles à utiliser pour colorier : [LISTE]

Tâche :
1. Corrélogramme circulaire avec ggcorrplot (méthode = "circle")
   avec * pour significatif p<0.05, ** pour p<0.01
2. Scatter plots des 3 corrélations les plus fortes avec droite de régression
3. Palette cohérente avec l'article (tons verts/bleus pour biologie)
```

---

**P39 — Visualiser les résultats d'un modèle d'occupancy**
```
Tu es expert unmarked et ggplot2.
Mon modèle d'occupancy est ajusté : fm1 (objet unmarkedFitOccu)
[NOMBRE] sites, espèce cible : [NOM ESPECE]

Tâche : Génère les figures de :
1. Probabilité d'occupancy (psi) par site en barplot avec IC 95%
2. Courbe de réponse de psi à la covariable la plus importante
3. Probabilité de détection (p) selon l'effort ou la saison
4. Carte si coordonnées GPS disponibles (psi interpolée)
Avec interprétation de chaque figure en 2 phrases.
```

---

**P40 — Figure pour présentation orale (différente de publication)**
```
Tu es expert data visualisation pour présentations scientifiques.
J'ai cette figure de publication : [DÉCRIRE LA FIGURE]
Audience : [conférence internationale / soutenance thèse / réunion ONG]
Durée de présentation du slide : environ [DURÉE] secondes.

Tâche : Adapte le code ggplot2 pour une présentation (pas un journal) :
- Texte plus grand (base_size = 16)
- Couleurs plus contrastées et visibles de loin
- Message principal mis en évidence (annotation, flèche, couleur accent)
- Titre court et impactant
- Maximum 3 éléments d'information par figure
- Format 16:9 (width=10, height=5.6)
```

---

## ✍️ CATÉGORIE 5 — RÉDACTION SCIENTIFIQUE (10 prompts)

---

**P41 — Rédiger la section Methods — Analyse des données**
```
Tu es un biologiste expert en rédaction scientifique anglaise.
Voici les analyses que j'ai effectuées dans R :
[LISTE : ex. vegan::diversity(), iNEXT::iNEXT(), vegan::metaMDS(),
          vegan::adonis2(), glm(), etc.]

Contexte :
- Journal cible : [NOM OU TYPE DE JOURNAL]
- Niveau de détail demandé : [paragraphe court / section détaillée]
- Style de citation des packages : [citer auteurs package ou juste "R version X"]

Tâche : Rédige la sous-section "Statistical analyses" en anglais académique.
Inclure : logiciel R et version, packages et versions, toutes les analyses
effectuées avec leur justification, seuil de significativité utilisé.
Format : Paragraphe fluide sans listes à puces.
```

---

**P42 — Rédiger la section Results à partir des sorties R**
```
Tu es expert en rédaction de sections Results pour articles en écologie.
Voici mes résultats R :

Richesse par site :
[COLLER : print(richesse_df)]

Indices de diversité :
[COLLER : print(diversity_indices)]

Tests statistiques :
[COLLER : print(anosim_result) ou print(kruskal_result)]

NMDS stress :
[VALEUR]

Tâche : Rédige la section Results complète en anglais académique.
Style : factuel, pas d'interprétation, chiffres entre parenthèses,
tables/figures citées comme Table 1, Fig. 1, etc.
Maximum [NOMBRE DE MOTS] mots.
```

---

**P43 — Rédiger la section Discussion**
```
Tu es biologiste expert en écologie de la conservation, spécialiste [RÉGION/TAXON].
Mes résultats principaux :
[COLLER les 3-5 résultats clés en bullet points]

Hypothèses initiales :
[DÉCRIRE CE QUE VOUS ATTENDIEZ]

Tâche : Rédige la Discussion en anglais académique :
1. Confirmation ou infirmation de chaque hypothèse
2. Comparaison avec 3-5 études similaires en [RÉGION/TAXON]
   (tu peux citer des études plausibles que je vérifierai)
3. Mécanismes explicatifs biologiques
4. Limites de l'étude
5. Implications pour la conservation
6. Perspectives de recherche
Longueur : environ [NOMBRE] mots.
```

---

**P44 — Rédiger l'Abstract**
```
Tu es expert en rédaction d'abstracts scientifiques.
Mon article complet (ou les points clés) :

Question de recherche : [UNE PHRASE]
Méthodes principales : [2-3 PHRASES]
Résultats clés : [3-4 RÉSULTATS NUMÉRIQUES CONCRETS]
Conclusions : [1-2 PHRASES]
Implication conservation : [1 PHRASE]

Journal cible : [NOM] — limite abstract : [NOMBRE] mots

Tâche : Rédige un abstract structuré en anglais académique en
[NOMBRE] mots maximum. Style : une phrase par élément IMRaD,
pas de jargon inutile, chiffres clés inclus, finir sur l'implication
appliquée.
```

---

**P45 — Interpréter les résultats statistiques en termes biologiques**
```
Tu es biologiste de terrain et biostatisticien.
Voici mes résultats statistiques bruts :
[COLLER les résultats R : summary(), print(), etc.]

Contexte écologique :
- Espèces étudiées : [TAXONS]
- Sites : [DESCRIPTION des habitats]
- Région : [LOCALISATION]
- Saison : [PÉRIODE]

Tâche : Traduis ces résultats statistiques en langage biologique :
- Que signifient ces chiffres pour les espèces et l'écosystème ?
- Quels processus écologiques peuvent expliquer ces patterns ?
- Est-ce écologiquement significatif (pas juste statistiquement) ?
- Quelles nuances biologiques faut-il mentionner dans la Discussion ?
```

---

**P46 — Rédiger la légende des figures**
```
Tu es expert en rédaction de légendes de figures pour articles scientifiques.
Figure [NUMÉRO] : [DÉCRIRE CE QUE MONTRE LA FIGURE en 2 phrases]

Statistiques affichées : [barres d'erreur = IC 95% / SD / SE / autre]
Tests montrés : [lettres de signification / * ** *** / ns]
Couleurs/formes : [EXPLIQUER CE QU'ELLES REPRÉSENTENT]
Abréviations utilisées : [LISTE]

Tâche : Rédige une légende de figure complète en anglais académique.
Style : commence par "Figure X." + titre en gras + explication complète
en texte courant + définition de toutes les abréviations/symboles.
Longueur typique : 50-150 mots selon la complexité.
```

---

**P47 — Répondre aux commentaires des reviewers sur les statistiques**
```
Tu es expert biostatisticien et rédacteur scientifique expérimenté.
Journal : [NOM], Statut : Minor/Major revision

Commentaire du reviewer [NUMÉRO] :
"[COLLER LE COMMENTAIRE EXACT DU REVIEWER]"

Mon analyse actuelle :
[DÉCRIRE CE QUE J'AI FAIT]
[COLLER LES RÉSULTATS PERTINENTS]

Tâche : Rédige :
1. La réponse au reviewer (point-by-point response) en anglais académique
2. Le texte révisé à insérer dans le manuscrit
3. Si une analyse supplémentaire est demandée, le code R pour la faire
```

---

**P48 — Créer un tableau récapitulatif publication-ready**
```
Tu es expert R (knitr, gt, flextable) et mise en page scientifique.
Je veux un tableau pour la section Results ou Supplementary.

Données à mettre en tableau :
[COLLER : print(mon_tableau)]

Journal cible : [NOM] — format : [Word / LaTeX / HTML]
Style demandé : [tableau simple / avec groupement de lignes / avec couleurs]

Tâche :
1. Génère le code R avec le package gt (ou flextable pour Word) pour
   formater ce tableau proprement
2. Arrondis pertinents par colonne (entiers pour counts, 2 décimales pour
   indices, 3 pour p-values)
3. Notes de bas de tableau pour les abréviations
4. Export dans le format adapté au journal
```

---

**P49 — Rédiger la section de remerciements et affiliations**
```
Tu es expert en conventions de rédaction scientifique internationale.
Informations pour mes remerciements :

Financement : [NOM DES BAILLEURS + NUMÉRO DE GRANT si disponible]
Aide terrain : [NOMS des assistants, guides, communautés locales]
Soutien logistique : [INSTITUTIONS, PARCS NATIONAUX, CNFEREF, etc.]
Aide statistique : [NOM si quelqu'un a aidé]
Autorisation de recherche : [MINISTÈRE, NUMÉRO DE PERMIT]

Tâche : Rédige les remerciements en anglais académique (style standard
pour journal d'écologie), en respectant l'ordre de citation conventionnel :
financement d'abord, puis aide intellectuelle, puis aide terrain,
puis autorisations. Maximum [NOMBRE] mots.
```

---

**P50 — Écrire la cover letter pour soumettre l'article**
```
Tu es expert en rédaction de cover letters pour revues scientifiques.
Informations :

Journal : [NOM COMPLET]
Éditeur en chef (si connu) : [NOM]
Titre de l'article : [TITRE]
Résumé en 2 phrases : [RÉSUMÉ]
Pourquoi ce journal : [ADÉQUATION SCOPE, AUDIENCE CIBLE]
Originalité principale : [CE QUI EST NOUVEAU]
Reviewers suggérés (optionnel) : [NOMS + AFFILIATIONS]
Conflits d'intérêt : [AUCUN ou DÉCRIRE]

Tâche : Rédige une cover letter formelle en anglais académique
(3-4 paragraphes, maximum 350 mots) suivant les conventions standard :
accroche sur l'importance du sujet, résumé concis, originalité,
adéquation au journal, déclarations obligatoires.
```

---

## 📌 RÉCAPITULATIF PAR USAGE

| Besoin | Prompts à utiliser |
|--------|-------------------|
| Importer mes données Excel/CSV | P01, P02, P03 |
| Créer la matrice pour vegan | P03, P04, P06 |
| Mon code plante | P11, P12, P13, P14 |
| Erreur vegan/iNEXT spécifique | P13, P16 |
| Calcul indices de diversité | P22, P21 |
| NMDS et comparaison sites | P24, P26 |
| Tester différences statistiques | P21, P23, P29, P30 |
| Créer figures publication | P31, P32, P33, P34, P35 |
| Combiner mes figures | P37 |
| Rédiger Methods R | P41 |
| Rédiger Results | P42 |
| Rédiger Discussion | P43 |
| Rédiger l'Abstract | P44 |
| Répondre aux reviewers | P47 |
| Soumettre l'article | P50 |

---

*50 prompts · 5 catégories · Prêts à l'emploi*
*Haja Fabrice RAZAFINDRABE MAMINIAINA — hajafabrice.r@gmail.com*
*Généré avec Claude · Avril 2026*
