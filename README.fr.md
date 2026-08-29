[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md) | [Finnish](README.fi.md) | [Cantonese](README.yue.md) | [Français](README.fr.md) | [Nederlands](README.nl.md)

# Nairobi OS : Infrastructure IA et Data Science Haute Performance, Zéro-Copie

[![PyPI Version](https://img.shields.io/pypi/v/nairobi-os.svg)](https://pypi.org/project/nairobi-os/)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
![System](https://img.shields.io/badge/Kernel-Linux_6.17_Native-orange.svg)
![Arch](https://img.shields.io/badge/Architecture-x86__64_/_ARM64-red.svg)

---

## L'Origine : Du Creuset au Métal



Mon parcours de programmation est enraciné dans l'architecture système de bas niveau et l'optimisation extrême. En 2015, j'ai exposé ma vision pour la construction de capacités techniques décentralisées sur le continent africain dans [ce traité sur la Silicon Valley du Kenya](https://www.linkedin.com/pulse/building-kenyas-silicon-valley-making-work-kevin-chege/). Lorsque la ruée vers l'or des LLM a commencé en 2023, j'étais en avance. J'ai construit et déployé des "wrappers" LLM, mais j'ai rapidement reconnu leurs limites, comme documenté dans cette [démonstration de 2023](https://www.linkedin.com/feed/update/urn:li:activity:7102930955807449088/).

J'ai réalisé que construire des couches de haut niveau sur des API instables était une impasse architecturale. La véritable guerre se joue à l'intersection des contraintes matérielles locales et de l'allocation des ressources.

Tout au long de l'année 2025, j'ai vécu sur un Lenovo X13 ThinkPad avec un profil matériel extrêmement limité :

```
Processeur : AMD Ryzen 5 PRO 4650U (6 Cœurs, 12 Threads)
Graphismes : iGPU AMD Radeon RX Vega 6
Mémoire : 32 Go RAM (avec une utilisation système élevée)
Stockage : 256 Go NVMe (rempli à 99%)
```

Sur cette machine exacte, j'ai passé l'année 2025 à construire **Tumz** ([Sarafakai](http://www.sarafakai.com)), une IA de support à la décision clinique, déconnectée (air-gapped) et à latence zéro. Elle exécutait simultanément la transcription audio en temps réel et l'inférence clinique sur le GPU intégré (iGPU), tout en maintenant l'intégralité du système UMLS (Unified Medical Language System) résident en RAM. Nous sommes actuellement en partenariat avec un hôpital kenyan pour piloter Tumz dans le cadre d'un essai clinique d'un an — car la santé humaine exige une validation empirique rigoureuse, et non les suppositions des développeurs.

Pendant le développement de Tumz, j'ai rencontré les inefficacités systémiques massives de la pile moderne de science des données :
1. **La Taxe Python** : Copie de mémoire de bout en bout, goulots d'étranglement du GIL et surcharge massive du runtime.
2. **La Taxe Navigateur** : Complications de Manifest V3, latence de rendu et échecs de communication haute fréquence dans les conversations agentiques de longue durée.
3. **Le Goulot d'Étranglement du Noyau (Kernel)** : Ordonnancement inefficace des processus, famine des threads CPU et surcharge du serveur d'affichage (commutation de contexte Wayland vs X11).

Ainsi, fin 2025, j'ai entrepris de construire une pile d'infrastructure qui contourne entièrement ces limites : un Système d'Exploitation Agentique conçu pour des pipelines de données zéro-copie et une exécution IA native au matériel. Ce dépôt est le cœur open-source de ce moteur.

---

## Le Dossier de Combat : 9 180 Contributions au Cours de l'Année Écoulée

Certains critiques dans la communauté de développement moderne regardent les projets nouveau et très avancés et les rejettent comme du "code boiler généré par IA". À ces sceptiques, je propose la preuve brute et physique du journal de commits.

Mon profil GitHub (https://github.com/ChegeKenya) témoigne d'un enregistrement empirique d'ingénierie système intense et quotidienne. En 2025 seul, j'ai enregistré 7 888 contributions. Dans les cinq premiers mois de 2026, j'ai ajouté 1 420 contributions. Soit 9 180 contributions au cours des 365 derniers jours — une séquence d'commits verts presque ininterrupte couvrant des runtimes Rust à faible latence, des pipelines IA cliniques et des systèmes de mémoire partagée zéro-copie. Ce code est écrit dans les tranchées, compilé sur du matériel bare-metal et audité octet par octet.

```
2025: [██████████████████████████████████████████████████] 7 888 Commits
  2026: [██████████] 1 420 Commits
  Total (Dernière Année) : 9 180 Commits de Code Système Pur
```

### Contraintes de Développement Nativement Matérielles

Mon parcours de programmation est enraciné dans l'architecture système de bas niveau et l'optimisation extrême, mécanique. En 2015, j'ai exposé ma vision pour la construction de capacités techniques décentralisées sur le continent africain dans [ce traité sur la Silicon Valley du Kenya](https://www.linkedin.com/pulse/building-kenyas-silicon-valley-making-work-kevin-chege/). Quand la transition LLM a commencé en 2023, j'ai reconnu tôt les limites structurelles des wrappers de haut niveau, comme documenté dans ma [démonstration de wrapper LLM de 2023](https://www.linkedin.com/feed/update/urn:li:activity:7102930955807449088/).

J'ai réalisé que construire des wrappers de haut niveau sur des API web instables était une impasse architecturale. La vraie guerre se livre à l'intersection des contraintes matérielles locales et de l'allocation directe des ressources.

Tout au long des années 2025 et 2026, j'ai vécu et développé sur un Lenovo X13 ThinkPad hautement contraint :

```
Processeur : AMD Ryzen 5 PRO 4650U (6 Cœurs, 12 Threads)
Graphismes : AMD Radeon RX Vega 6 iGPU (Architecture Mémoire Partagée)
Mémoire : 29 GiB RAM (avec une utilisation système élevée)
Stockage : 256 GB NVMe (99% plein, fortement contraint)
Noyau Hôte : Linux 6.17.0-29-generic
```

Sur cette machine exacte, j'ai passé 2025 à construire **Tumz** ([Sarafakai](http://www.sarafakai.com)), une IA de support à la décision clinique air-gapped, zéro-latence. Elle exécutait simultanément la transcription audio en temps réel et l'inférence clinique sur le GPU intégré (iGPU), gardant le dictionnaire complet du Unified Medical Language System (UMLS) résident dans la RAM partagée. Sarafakai collabore actuellement avec un hôpital kenyan pour piloter Tumz dans un essai clinique d'un an — car la santé humaine exige une validation rigoureuse et empirique, pas des suppositions de développeurs.

---

## Traction Mondiale & Télémétrie

Lancé le 6 mai 2026, Nairobi OS a rapidement gagné du terrain parmi les programmeurs système, les chercheurs quantitatifs et les architectes d'edge computing du monde entier. Ces statistiques de téléchargement proviennent du [Tableau de bord ClickPy Nairobi OS](https://clickpy.clickhouse.com/dashboard/nairobi-os) en direct.

### Distribution Mondiale Cumulée (6 mai 2026 – 23 mai 2026)

| Métrique | Mesure | Contexte |
| :--- | :--- | :--- |
| **Rang Mondial** | **#75 293** | Sur 797 894 paquets actifs sur PyPI |
| **Percentile** | **9,43%** | Classement de premier plan pour les extensions Python système |
| **Total Téléchargements** | **1 525** | Téléchargements organiques, haute intention |

### Volume de Téléchargement par Version

```
  0.2.0 [████████████████████████████████████████] 342
  0.2.1 [██████████████████████████] 224
  0.3.0 [████████████████████████] 212
  0.3.1 [████████████████████] 176
  0.1.0 [███████████████████] 169
  0.4.1 [██████████████] 120
```

### Top 10 des Régions Souveraines d'Adoption

| Rang | Région | Code Pays | Volume de Téléchargement |
| :--- | :--- | :--- | :--- |
| 1 | États-Unis | US | 661 |
| 2 | Hong Kong | HK | 103 |
| 3 | Chine | CN | 84 |
| 4 | Allemagne | DE | 74 |
| 5 | Japon | JP | 65 |
| 6 | Singapour | SG | 56 |
| 7 | Royaume-Uni | GB | 51 |
| 8 | France | FR | 51 |
| 9 | Russie | RU | 42 |
| 10 | Corée du Sud | KR | 30 |

---

## Soutien & Souveraineté

Si Nairobi OS optimise vos pipelines de données, réduit vos factures cloud ou propulse vos architectures agentiques locales, envisagez de soutenir notre recherche système indépendante. Chaque contribution est directement déployée dans l'optimisation des compilateurs au niveau matériel et les tests d'edge-compute à Nairobi.

[![Soutenir le développement de Nairobi OS](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

Pour toute demande directe : aiwithafrica@gmail.com

---

## Caractéristiques Clés

* **Utilisation de l'ordinateur sans pixels** : Contourne les pipelines d'agents basés sur la vision, lents et coûteux. Interagit nativement avec le bureau Linux via AT-SPI2 et l'algorithme de compression TOON (Token-Oriented Object Notation), envoyant des arbres hiérarchiques bruts directement aux LLM.
* **Ingestion Zéro-Copie** : Chargement de données accéléré par le matériel, contournant le noyau via `io_uring` et des Pages Géantes (Huge Pages) de 1 Go.
* **Visualisation Accélérée par le Matériel** : Tracés Jupyter interactifs à faible latence utilisant le démon de rendu `lagos-lite`, construit sur `wgpu` et `egui`.
* **Exécution Analytique Vectorisée** : Saturation extrême du CPU utilisant l'exécution de requêtes Polars et des pipelines de données multi-threadés Rayon.
* **Interface Souveraine** : Une API Python fluide (`SovereignFrame`) qui encapsule les descripteurs de fichiers mappés en mémoire et l'IPC.

---

## Open Source vs Architecture Entreprise

Nairobi OS est structurellement bifurqué. Le dépôt open-source fournit les primitives fondamentales de traitement de données haute performance et de visualisation sur un seul nœud. L'écosystème commercial fermé contient les implémentations avancées multi-agents, haute disponibilité et spécifiques à l'industrie.

```
                                  +---------------------------------------+
                                  |         API Python Nairobi            |
                                  +---------------------------------------+
                                                      |
                                     [ GVariant via D-Bus / mémoire partagée ]
                                                      |
                                                      v
                                  +---------------------------------------+
                                  |           Hub Nairobi                 |
                                  +---------------------------------------+
                                                      |
                    +---------------------------------+---------------------------------+
                    |                                                                   |
                    v                                                                   v
     +------------------------------+                                    +------------------------------+
     |   Raffinerie Axum (Données)  | <===[ IPC Zéro-Copie / iceoryx2 ]==> |    Vision Lagos (Visuel)     |
     +------------------------------+                                    +------------------------------+
```

### Open Source Crate Workspace (`crates/`)
1. `nairobi-axum-refinery` — Rust daemon managing raw data ingestion, Rayon-parallelized statistics, and Polars-vectorized query execution.
2. `nairobi-hub` — Central IPC orchestrator; routes file descriptors and signals between clients and the refinery daemon.
3. `lagos-lite` — Local/headless rendering engine using egui/wgpu hardware acceleration with zero-copy mmap data access.
4. `nairobi-protocol` — Shared protocol layer: GVariant serialization schemes, error types, and shared-memory layouts.
5. `nairobi-python` — The Python extension module, compiled via PyO3 and packaged with Maturin (`nairobi-os`).
6. `nairobi-canvas` — Immediate-mode node-graph compiler with hardware-accelerated UI (wgpu/egui), including a native file picker and SQL query presets.
7. `nairobi-connector` — Model Context Protocol (MCP) server and AT-SPI2 semantic accessibility bridge exposing TOON representations for LLM agents.
### Écosystème Privé Corporate (`modules/`)

Nos composants de niveau entreprise sont conservés dans un dépôt privé (`Sovereign-Systems-Lab`) et sous licence pour les infrastructures industrielles, financières et étatiques.

1. **`sovereign-ui`** : Le moteur AT-SPI2 d'entreprise. Implémente la sécurité du protocole Aegis, le verrouillage matériel et la manipulation de bureau de qualité production.
3. **`tactical-rtos-node`** : Ordonnanceur de système d'exploitation en temps réel à ultra-faible latence pour l'automatisation industrielle critique.
4. **`industrial-guardian-rust` / `industrial-guardian-python`** : Couche d'ingénierie de fiabilité de site (SRE) autonome avec évitement prédictif des OOM, des fuites de mémoire et des plantages système.
5. **`fintech-bridge-rust`** : Analyseur de transactions haute fréquence en temps réel et pont vers les systèmes hérités (parsing de terminaux EBCDIC/SBA).
6. **`aviation-audio-rust`** : Traitement de flux audio sans verrou (lock-free) en moins d'une milliseconde, analyse de télémétrie acoustique et DSP d'ondes brutes.
7. **`drawbridge_api`** : Pont gRPC sécurisé, authentifié et multi-tenant isolant le noyau local des appels d'agents cloud non fiables.

### Matrice de Comparaison des Capacités

| Capacité / Fonctionnalité | Cœur Open Source (`crates/`) | Suite Entreprise (`modules/`) |
| :--- | :---: | :---: |
| **Moteur d'Ingestion** | `mmap` / `copy_file_range` | `io_uring` + `SQPOLL` + 1Go Huge Pages |
| **Analyse Statistique** | Stats descriptives de base | Vectorisé, multi-passes, corrélation |
| **Moteur de Requête** | Polars SQL en processus | Cluster distribué Apache Arrow / DataFusion |
| **Mécanisme IPC** | Mémoire partagée POSIX / D-Bus | Arènes de mémoire partagée `iceoryx2` |
| **Visualisation** | Jupyter `anywidget` local | Flux WebRTC GStreamer / Overlays Wayland |
| **Sécurité & Conformité** | Limites POSIX standards | Protocole Aegis, Registre Forensique SHA-256 |
| **Authentification** | Aucune (Utilisateur local) | Liaison Matérielle (TPM 2.0 / CPU ID), PKI privée |
| **Cible Plateforme** | Linux nœud unique | Cloud Distribué / Nœud Edge / Trading HF |

---

## Installation & Configuration

### Prérequis
- **OS** : Linux (Ubuntu 22.04+ recommandé) ou Windows Subsystem for Linux (WSL2).
- **GPU** : Pilote compatible Vulkan, Metal ou OpenGL.
- **Python** : 3.10 ou plus récent.
- **Rust** : Chaîne d'outils stable (si compilation à partir des sources).

### Installation Rapide (PyPI)
```bash
pip install nairobi-os
```

### Compilation à partir des Sources
Pour compiler l'intégralité de l'espace de travail, y compris les démons natifs et l'extension Python :

1. **Cloner le dépôt** :
   ```bash
   git clone https://github.com/KevinKenya/nairobi-connector-open-source.git
   cd nairobi-connector-open-source
   ```

2. **Configurer l'environnement virtuel** :
   ```bash
   python3 -m venv .venv
   source .venv/bin/activate
   pip install maturin pyo3-build-config zbus anywidget traitlets pandas
   ```

3. **Exécuter la compilation** :
   ```bash
   chmod +x build_wheel.sh
   ./build_wheel.sh --release
   ```
   Ceci compile les démons natifs, les copie dans le répertoire du paquet et génère un wheel sous `crates/nairobi-python/target/wheels/`.

---

## Guide d'Utilisation

### 1. Analyse de Données (Le Pipeline In-Memory)

Nairobi OS fournit l'API `SovereignFrame`. Elle gère le mappage mémoire brut en arrière-plan, permettant une manipulation rapide des données.

```python
import nairobi_os as nb

# Allumer le démon de la raffinerie en arrière-plan
nb.connect()

# Ingestion du dataset via un pipe mémoire zéro-copie
frame = nb.read_csv("simulator/fndds_ingredient_nutrient_value.csv")

# Calculs vectorisés via la raffinerie Rust
profile = frame.crunch("value")
print(f"Moyenne : {profile['mean']:.4f}")
print(f"Écart-type : {profile['std_dev']:.4f}")

# Exécuter des requêtes SQL arbitraires directement sur le frame mappé en mémoire
subset = frame.query("SELECT * FROM dataset WHERE value > 50.0")

# Lancer le widget de traçage interactif accéléré par Lagos
subset.plot(column="value")
```

### 2. Utilisation de l'ordinateur sans pixels (MCP)

Pour utiliser l'interface sémantique AT-SPI2, votre agent IA doit interagir avec les outils du serveur MCP exposés plutôt que de lire des captures d'écran :

```
                     SÉQUENCE D'UTILISATION ORDINATEUR
                     
  [ Agent LLM ]                                 [ Nairobi OS ]
        |                                             |
        |===> nairobi_find_window("Éditeur Texte") ==>| (Localise la cible)
        |<=== Retourne ID Fenêtre & Limites ==========|
        |                                             |
        |===> nairobi_get_ui_map() ==================>| (Génère le TOON)
        |<=== Retourne l'arbre Markdown compressé ====|
        |     "[ID: 12] Bouton: 'Enregistrer'"        |
        |                                             |
        |===> nairobi_interact(12, "click") =========>| (Exécute l'action)
        |<=== Retourne le statut de succès ===========|
```

---

## Optimisation Système (Guide du Contributeur)

Pour atteindre les profils de performance affichés dans nos benchmarks, votre noyau hôte doit être configuré pour le mappage mémoire au niveau système.

### Pages Géantes (Huge Pages) de 1 Go
Nairobi OS utilise des pages de 1 Go pour contourner la surcharge de traduction du TLB (Translation Lookaside Buffer) du CPU sur les jeux de données massifs.

Pour allouer une page géante sur votre hôte Linux :
```bash
echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
```
*Note : Si le système ne peut pas allouer une page de 1 Go en raison de la fragmentation, le moteur se replie automatiquement sur les Transparent Huge Pages (THP).*

### Configuration du D-Bus Broker
Dans les environnements à haute fréquence, assurez-vous que `dbus-broker` est installé à la place du `dbus-daemon` hérité pour gérer la propagation rapide des signaux sur le plan de contrôle.

---

## Licence

Ce projet est sous licence **Apache License 2.0**.  
*(Note : Des portions du format TOON et de l'implémentation du pont sont créditées aux auteurs de TOON.)*

---
© 2026 Kevin Chege. Tous droits réservés.  
*Sovereign Systems Lab, Nairobi, Kenya.*
