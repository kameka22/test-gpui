# LongBridge Pro - Trading Desktop

Une application de trading desktop haute performance construite avec [GPUI](https://www.gpui.rs), le framework d'interface utilisateur en Rust créé par les développeurs de Zed.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![GPUI](https://img.shields.io/badge/GPUI-UI_Framework-blue?style=for-the-badge)

## À propos

Cette application est inspirée de [LongBridge Desktop](https://longbridge.com/desktop/), une application de trading professionnelle. Elle démontre les capacités de GPUI pour créer des interfaces utilisateur natives performantes avec Rust.

### Caractéristiques

- **Interface Moderne** : Thème sombre professionnel optimisé pour le trading
- **Haute Performance** : Rendu GPU accéléré grâce à GPUI
- **Données en Temps Réel** : Affichage simulé de cotations boursières
- **Navigation par Onglets** : Markets, Watchlist, Portfolio, Orders
- **Visualisation Heatmap** : Carte thermique des performances du marché
- **Tableau de Marché** : Liste détaillée des actions avec prix, variations, volumes
- **Watchlist** : Suivi rapide des actions favorites

## Architecture

```
├── Sidebar Navigation
│   ├── Markets (vue principale du marché)
│   ├── Watchlist (heatmap des actions favorites)
│   ├── Portfolio (à venir)
│   └── Orders (à venir)
├── Main Panel
│   └── Tableau de données avec:
│       ├── Symbol
│       ├── Name
│       ├── Price
│       ├── Change (%)
│       ├── Volume
│       ├── Market Cap
│       └── P/E Ratio
└── Right Sidebar
    └── Watchlist compacte
```

## Technologies

- **[GPUI](https://www.gpui.rs)** : Framework UI GPU-accéléré en Rust
- **Rust** : Langage de programmation système performant et sûr
- **rand** : Génération de données de marché simulées
- **chrono** : Gestion des dates et heures

## Installation et Compilation

### Prérequis

#### Linux
```bash
# Installer les dépendances système
sudo apt-get update
sudo apt-get install -y \
    libfontconfig-dev \
    libfreetype6-dev \
    libxcb-composite0-dev \
    libxkbcommon-dev \
    libssl-dev
```

#### macOS
```bash
# Installer Xcode Command Line Tools
xcode-select --install
```

### Compilation

```bash
# Cloner le repository
git clone <repository-url>
cd test-gpui

# Compiler et lancer
cargo run --release
```

## Utilisation

### Navigation

- Cliquez sur les onglets de la sidebar pour naviguer entre les vues
- **Markets** : Vue d'ensemble du marché avec toutes les actions
- **Watchlist** : Visualisation heatmap des actions favorites

### Fonctionnalités Actuelles

1. **Tableau de Marché** : Affiche 10 actions populaires avec :
   - Prix en temps réel (simulé)
   - Variations en dollars et pourcentage
   - Volume de trading
   - Capitalisation boursière
   - Ratio P/E

2. **Heatmap** : Visualisation des performances avec :
   - Couleur verte pour les gains (intensité basée sur le %)
   - Couleur rouge pour les pertes (intensité basée sur le %)
   - Informations au survol

3. **Watchlist Rapide** : Panneau latéral avec résumé des actions favorites

## Architecture du Code

### Structure Principale

```rust
struct TradingApp {
    current_tab: NavigationTab,    // Onglet actif
    stocks: Vec<Stock>,             // Liste des actions
    watchlist: Vec<Stock>,          // Actions favorites
    selected_stock: Option<usize>,  // Action sélectionnée
}
```

### Composants UI

- `render_sidebar()` : Navigation principale
- `render_market_data()` : Tableau de marché
- `render_stock_table()` : Grille de données
- `render_watchlist()` : Panneau latéral
- `render_heatmap()` : Visualisation thermique

### Palette de Couleurs

```rust
BG_PRIMARY: #0a0e1a      // Fond principal
BG_SECONDARY: #141824    // Fond secondaire
BG_TERTIARY: #1e2330     // Fond tertiaire
TEXT_PRIMARY: #e4e6eb    // Texte principal
TEXT_SECONDARY: #9ca3af  // Texte secondaire
ACCENT_BLUE: #3b82f6     // Accent bleu
GREEN_POSITIVE: #10b981  // Gains (vert)
RED_NEGATIVE: #ef4444    // Pertes (rouge)
BORDER_COLOR: #2d3748    // Bordures
```

## Développement Futur

- [ ] Graphiques de prix interactifs
- [ ] Données de marché en temps réel (API)
- [ ] Vue Portfolio avec positions
- [ ] Système de gestion des ordres
- [ ] Alertes de prix configurables
- [ ] Export de données
- [ ] Thèmes personnalisables
- [ ] Support multi-marchés (US, EU, ASIA)

## Performance

GPUI permet d'atteindre :
- **Jusqu'à 120 FPS** pour un rendu fluide
- **Faible consommation CPU/RAM** grâce au rendu GPU
- **Démarrage rapide** < 2 secondes
- **Interface réactive** même avec de grandes listes de données

## Contributions

Les contributions sont les bienvenues ! N'hésitez pas à :
- Signaler des bugs
- Proposer de nouvelles fonctionnalités
- Améliorer la documentation
- Soumettre des pull requests

## Licence

Ce projet est un exemple éducatif démontrant les capacités de GPUI.

## Ressources

- [GPUI Documentation](https://www.gpui.rs)
- [Zed Editor](https://zed.dev) - Éditeur construit avec GPUI
- [Rust Programming Language](https://www.rust-lang.org)
- [LongBridge Desktop](https://longbridge.com/desktop/) - Inspiration

## Crédits

- **GPUI Framework** : [Zed Industries](https://zed.dev)
- **Inspiration** : [LongBridge Desktop](https://longbridge.com/desktop/)
- **Langage** : Rust Programming Language

---

Développé avec ❤️ et Rust 🦀
