# Stellar Drift
Projet d'apprentissage en Rust + Bevy reprenant le jeu Asteroids d'Atari.

![Rust](https://img.shields.io/badge/Rust-2021-FD6035)
![Bevy](https://img.shields.io/badge/Bevy-0.12-131254)
![Platform](https://img.shields.io/badge/web%20|%20linux%20|%20windows-lightgrey)
[![Play](https://img.shields.io/badge/▶_Play-itch.io-FD6035)](https://ficelo.itch.io/stellar-drift)

![Stellar Drift - aperçu du gameplay](media/stellar_drift.gif)

## Statut

Il s'agit de mon premier projet développé entièrement en Rust sous Bevy.

Jouable sur navigateur dès maintenant - [sur ma page itch.io](https://ficelo.itch.io/stellar-drift)

## Description du projet
Ce projet est à but éducatif.

J'y explore la conception de jeux indépendants. Stellar Drift est volontairement minimaliste afin de me concentrer sur l'entraînement de la stack technique.

Il me permet aussi d'apprendre l'**ECS**, *Entity Component System*, ainsi que de me familiariser avec **Bevy**, un moteur développé en Rust.

J'y intègre des notions clés, afin de saisir en profondeur les étapes de développement d'un jeu, comme :
- Un système de rendu physique, avec Rapier2D.
- Une gestion des états de jeu (Menu, Pause, Jeu).
- Une progression par vagues à difficulté croissante.
- La fragmentation des astéroïdes (gros → moyens → petits).
- Une intégration audio.

## Contrôles
| Action | Touche(s) |
|---|---|
| Se déplacer | <kbd>W</kbd> <kbd>A</kbd> <kbd>S</kbd> <kbd>D</kbd> ou <kbd>↑</kbd> <kbd>←</kbd> <kbd>↓</kbd> <kbd>→</kbd> |
| Tirer | <kbd>Espace</kbd> |
| Lancer / rejouer | <kbd>Entrée</kbd> |
| Pause | <kbd>Entrée</kbd> |

## Stack technique
- **Langage** : Rust (édition 2021).
- **Moteur** : Bevy 0.12.1 (contrainte liée à l'environnement de développement).
- **Physique** : bevy_rapier2d.
- **Architecture** : ECS (Entity Component System).
- **Plateforme cible** : compilable sous Linux, exécutable sous Windows (via les Releases), et jouable sur navigateur (Web/HTML5 via WASM).

## Structure du projet
```text
src/
├── main.rs      # Setup de l'app, plugins, ordonnancement des systèmes
├── states.rs    # Machine à états (menu / jeu / pause / transitions / game over)
├── player.rs    # Déplacement du vaisseau, tir, mort et respawn
├── asteroid.rs  # Spawn, déplacement et fragmentation des astéroïdes
├── game.rs      # État du run (vies, niveau), fin de partie, wrap d'écran
├── hud.rs       # Interface : menus, vies, prompts à l'écran
├── audio.rs     # Intégration sonore
├── camera.rs    # Caméra 2D
└── entity.rs    # Composants partagés (Health) et nettoyage des entités
```

## Lancer le projet
Prérequis :
- Rust installé ([rustup.rs](https://rustup.rs))

### Version native (Linux / desktop)
```bash
git clone https://github.com/ficeloo/stellar_drift.git
cd stellar_drift
cargo run --release
```
> Le mode release produit un binaire optimisé : la compilation est plus longue,
> mais le jeu tourne de façon fluide.

### Version web (en local)
Prérequis : trunk (`cargo install trunk`) + la cible wasm (`rustup target add wasm32-unknown-unknown`).
```bash
trunk serve   # puis ouvrir l'URL locale affichée
```

### Sous Windows
1. Téléchargez la dernière version de la build dans la section [Releases](https://github.com/ficeloo/stellar_drift/releases) (actuellement v1.0).
2. Téléchargez également le dossier `assets/` correspondant.
3. Placez l'exécutable et le dossier `assets/` dans un même répertoire.
4. Lancez l'exécutable.

## Screenshots
<table>
  <tr>
    <td><img src="media/MainMenu.png" alt="Menu principal" width="100%"></td>
    <td><img src="media/CoreGameplay.png" alt="Gameplay" width="100%"></td>
  </tr>
  <tr>
    <td><img src="media/PauseMenu.png" alt="Menu pause" width="100%"></td>
    <td><img src="media/GameOver.png" alt="Game over" width="100%"></td>
  </tr>
</table>

## Ressources & crédits
- **Moteur** : ![Bevy](https://bevyengine.org/) (Rust).
- **Physique** : ![bevy_rapier2d](https://github.com/dimforge/bevy_rapier).
- **Assets** : réalisés par anim86 — ![profil itch.io](https://anim86.itch.io/).
- **Sound Design** : réalisés en co-création avec Nam-Son - ![instagram](https://www.instagram.com/ns0_pr4/).
