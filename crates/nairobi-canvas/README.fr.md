[English](README.md) | [简体中文](README.zh-CN.md) | [廣東話](README.yue.md) | [Français](README.fr.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Suomi](README.fi.md) | [Nederlands](README.nl.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Canvas : Compilateur Visuel de Graphe de Nœuds en Mode Immédiat

Nairobi Canvas est un compilateur visuel accéléré par le matériel pour la construction de pipelines de traitement de données. Il fournit une interface utilisateur de graphe de nœuds en mode immédiat basée sur `egui`/`egui-snarl` qui compile les flux de travail visuels au format GVariant DAG (Graphe Acyclique Dirigé) pour exécution par le Nairobi Hub.

## Caractéristiques

- **Constructeur de Pipeline Visuel** : Interface de graphe de nœuds par glisser-déposer pour les flux de travail de données
- **Sélecteur de Fichiers Natif** : Cliquez sur le bouton 📂 sur les nœuds Ingest pour parcourir les fichiers CSV
- **Préréglages de Requêtes SQL** : Modèles de requêtes préconfigurés (Toutes les colonnes, Colonne unique, Clause Where, Multi-colonnes)
- **Sérialisation GVariant** : Compile les graphes au format GVariant pour un IPC sans copie
- **Tri Topologique** : Détection automatique des cycles et ordonnancement de l'exécution

## Types de Nœuds

| Nœud | Entrées | Sorties | Description |
|------|---------|---------|-------------|
| **Ingest** | 0 | 1 | Charge les jeux de données CSV via le sélecteur de fichiers natif |
| **SqlQuery** | 1 | 1 | Exécute des requêtes SQL Polars sur les données d'entrée |
| **AxiomCrunch** | 1 | 1 | Calcule des statistiques (moyenne, écart-type, kurtosis) |
| **LagosPlot** | 1 | 0 | Génère des visualisations (sparkline, nuage de points, PNG, JPG) |

## Installation

```bash
pip install nairobi-os
```

Ou construire à partir de la source :
```bash
cargo build --release
# La démo canvas est un binaire Rust - voir examples/canvas_compile_demo.rs
```

## Utilisation

### Rust (Natif)

Exécutez l'application de démonstration :
```bash
cargo run --example canvas_compile_demo
```

### Python

Utilisation du package installé :
```python
import nairobi_os as nb

# Ouvrir le canevas visuel pour la compilation DAG
dag_bytes = nb.canvas.open()

# Exécuter le pipeline compilé
if dag_bytes:
    nb.canvas.execute(dag_bytes)
```

Ou exécutez le script de test complet :
```bash
python test_canvas.py
```

Ce script effectue :
1. `nairobi_os.ignite()` - Lance les démons Axum Refinery et Nairobi Hub
2. `nb.canvas.open()` - Lance l'éditeur de graphe de nœuds visuel
3. `nb.canvas.execute(dag_bytes)` - Exécute le pipeline compilé avec des mesures de temps

Le canevas exporte un DAG encodé en GVariant qui peut être :
- Exécuté via `nb.canvas.execute()`
- Sauvegardé sur disque pour une utilisation ultérieure
- Transmis via D-Bus/mémoire partagée

## Construction de Graphes

1. **Clic droit** sur la grille du canevas pour ouvrir le menu des nœuds
2. Sélectionnez un type de nœud (Ingest, SQL Query, Axiom Crunch ou Lagos Plot)
3. **Connectez** les nœuds en faisant glisser des broches de sortie (bleu) vers les broches d'entrée (vert)
4. Cliquez sur **Compile Graph** pour sérialiser le flux de travail

## Flux d'Exécution

```
Graphe Canvas → DAG GVariant → Nairobi Hub → Axum Refinery / Lagos Vision
```

Le DAG compilé est transmis via IPC au Hub, qui route les nœuds vers :
- **Axum Refinery** : Ingestion de données et traitement statistique
- **Lagos Vision** : Rendu de visualisation accéléré par le matériel

Pour les détails de l'architecture et l'aperçu complet du système, voir le [README du dépôt principal](../README.md).

## Support
Si vous trouvez Nairobi OS utile, envisagez de soutenir le projet :

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## Licence
Ce projet est sous licence **Apache License 2.0**.

© 2026 Kevin Chege. Tous droits réservés.
