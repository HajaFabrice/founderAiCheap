# EcoR Toolkit - Publication theme for ggplot2

library(ggplot2)

theme_ecor_publication <- function(base_size = 12) {
  theme_minimal(base_size = base_size) +
    theme(
      plot.title = element_text(face = "bold", size = base_size + 2),
      plot.subtitle = element_text(color = "grey35"),
      axis.title = element_text(face = "bold"),
      panel.grid.minor = element_blank(),
      legend.title = element_text(face = "bold"),
      legend.position = "right"
    )
}

palette_ecor <- c(
  forest = "#116149",
  secondary = "#4BAE8A",
  degraded = "#D76B45",
  water = "#3A7CA5",
  neutral = "#6B7280"
)

