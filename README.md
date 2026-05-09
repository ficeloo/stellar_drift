# Stellar Drift
Projet d'apprentissage en Rust + Bevy reprenant le jeu Asteroid d'Atari.

## Statut
> 🚧 Projet en cours de développement. 🚧

Il s'agit de mon premier développé entièrement en Rust sous Bevy.

Jouable prochainement. Les prototypes peuvent être lancés.

## Description du projet
Ce projet est à but éducatif.
Il me permet d'apprendre l'**ECS**, *Entity Component System*, ainsi que de me familiariser avec **Bevy**, un moteur entièrement développé en Rust.

J'y intègre des notions clés, afin de saisir en profondeur les étapes de développement d'un jeu, comme :
- Un système de rendu physique, avec Rapier2D.
- Un gestion des états de jeu (Menu, Pause, Jeu)
- Une intégration audio (🚧 A VENIR 🚧)

## Stack technique
- **Langage** : Rust (édition 2021)
- **Moteur** : Bevy 0.12.1 (ou ta version actuelle)
- **Architecture** : ECS (Entity Component System)
- **Plateforme cible** : compilable sous Linux / executable sous Windows (via le dossier release)

## Lancer le projet
Prérequis :
- Rust installée ([rustup.rs](https://rustup.rs))

### Sous Linux
``` bash
git clobe https://github.com/ficeloo/stellar_drift.git
cd stellar_drift
cargo run --release
```
> Le `--release` permet de lancer sans le mode débug, avec une compilation bien plus rapide.

### Sous Windows
Tout d'abord, il faut créer un dossier à l'emplacement de votre choix, ici nommé `stellar_drift/`

Il faut installer la dernière version accessible depuis le dossier release et le placer dans ce nouveau dossier.
> Actuellement il s'agit de la v0.4

Puis installer le dossier d'assets

Ensuite il faudra placer les assets dans le dossier principale.

Et enfin, plus qu'à lancer !
