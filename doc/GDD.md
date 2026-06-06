# GDD — Stellar Drift

Genre : arcade / shoot 'em up · Plateforme : Web (WASM) + Linux / Windows · Stack : Rust + Bevy (ECS) · Statut : Released (v1.0)
> _[itch.io](https://ficelo.itch.io/stellar-drift) — [GitHub](https://github.com/ficeloo/stellar_drift/)._

## La vision

Stellar Drift est un jeu volontairement minimaliste reprenant le concept d'*Asteroids* d'Atari : nettoyer des vagues successives d'astéroïdes et survivre le plus loin possible. La boucle de gameplay est simple, mais nerveuse.

Le projet est avant tout un exercice : une architecture délibérément réduite pour se concentrer sur les fondamentaux de l'**ECS** (*Entity Component System*) avec **Bevy**, un moteur écrit en Rust.

**Piliers de design**
- **Minimalisme assumé** — peu d'éléments, choix de la simplicité.
- **Boucle arcade courte** — lisible immédiatement, tendue en quelques secondes.
- **Vitrine d'une architecture ECS propre** — le code est une pièce du portfolio autant que le jeu.

**Non-objectifs** *(ce que le jeu ne cherche volontairement pas à faire)*
- Pas de score ni de classement — la progression par vagues sert de jauge.
- Pas de nouveaux types d'ennemis.
- Pas d'effets de particules ni de polish visuel poussé.

## Boucle de gameplay

**Boucle principale (macro-loop)**
> Nettoyer une vague → Nouvelle vague (n + 1 astéroïdes) → Répéter

**Boucle instantanée (micro-loop)**
> S'orienter → Avancer → Viser → Tirer → Esquiver

**En cas d'échec**
> Touché → Perte d'une vie → Respawn (invincibilité temporaire) / Game Over

Chaque astéroïde détruit se fragmente, ce qui augmente momentanément le nombre de cibles à l'écran : nettoyer une vague crée donc son propre pic d'intensité.

## Mécaniques principales

### Déplacement (inertiel)
Le vaisseau pivote sur lui-même et accélère par poussée. Il conserve son inertie et est progressivement ralenti par un amortissement. Aux bords de l'écran, il réapparaît du côté opposé (*Monde toroïdale*).
> _Le joueur divise sa vitesse de moitié au rythme de_ $t^½ = ln(2) / 1.2 ≈ 0.578 secs$.

### Tir
Le vaisseau tire des projectiles dans la direction où il pointe. Chaque projectile disparaît après une durée de vie limitée.
> _Cadence de tir: 0.2 sec — Durée de vie du projectile: 1.5 sec._

### Astéroïdes & fragmentation
Trois tailles : grand, moyen, petit. À l'impact d'un projectile, un astéroïde se scinde :
> Grand → 2 moyens · chaque moyen → 2 petits · chaque petit → disparition

Chaque astéroïde se déplace et tourne avec une vitesse et une orientation aléatoires.

### Vagues & difficulté
Une vague est terminée lorsque tous les astéroïdes (fragments compris) sont détruits ; la vague suivante démarre alors, plus dense. Le nombre d'astéroïdes augmente à chaque niveau.
> _Soit N le niveau actuel, la vague d'astéroide est composée de N + 3 Grands astéroides._

### Vies, mort & respawn
Le joueur démarre avec 3 vies. Toute collision entre le vaisseau et un astéroïde coûte une vie. Le vaisseau réapparaît alors avec une brève invincibilité ; à 0 vie, c'est le Game Over.
> _Durée de l'invicibilté : 2.0 secs._

### États de jeu
Menu principal → Jeu → (Pause) → Transition de vague → Game Over. La pause gèle la simulation physique sans détruire le monde. L'écran de Game Over permet de relancer une partie ou de revenir au menu.

## Itération v1 → v2

Avant la refonte, Stellar Drift était un prototype jouable mais incomplet. La refonte l'a transformé en jeu fini et publié.

| | v1 (prototype) | v2 (version publiée) |
|---|---|---|
| Boucle | pas de fin de partie résolue | boucle complète : Game Over + relance |
| États | gestion ad hoc | machine à états dédiée (menu / jeu / pause / vagues / game over) |
| Vies | réinitialisées à chaque vague | persistantes via une ressource de run |
| Rendu | colliders de debug visibles | rendu propre (debug retiré) |
| Diffusion | local uniquement | build WASM publié sur itch.io |
| Bugs notables | reset des vies, crash sur despawn multiple | corrigés |

### Le travail derrière
Certains systèmes se sont imposés à moi au fur et à mesure du projet.
- La machine à états : Afin d'avoir une sensation de progression fiable et une interface intéressante, j'ai introduit plusieurs états (MainMenu, Paused, LevelTransition...). Passant d'une simple boucle de jeu à un jeu complet.
- Déploiment WASM : Premier portage d'un jeu WASM. Il était plus intéressant de pouvoir jouer à ce petit jeu sur mon portfolio, j'ai donc dû créer un portage avec Trunk pour y parvenir.

## Ce que je referais autrement / limites

### Structure claire
Ce projet étant mon premier jeu fait de A à Z, j'ai parfois manqué de supervision sur l'importance de certains éléments en architecture ECS et Game Design.

Je me suis rendu compte de l'importance d'implémenter une machine à états efficace dès le début, elle me permet de structurer le projet en différentes parties.
Je ferais aussi attention à diviser efficacement les composants (! NOTE A MOI-MEME: CHAQUE PETIT ELEMENT DOIT ETRE UN COMPOSANT !).

### Direction artistique
Durant la conception, j'ai été tenté d'aborder certains éléments intéressant comme :
- des feedbacks visuels (particules)
- une diversité dans les feedbacks sonores (plusieurs sons d'explosions)
- une musique de fond.

J'ai identifié ces éléments comme non-essentiel au principe du jeu voulu initialement, à savoir une apprentissage d'une architecture ECS et de Bevy.
>_Je me réserve le droit d'apporter de légère modifcations si l'envie m'en prend._

## Stack & architecture

- **Langage** : Rust (édition 2021)
- **Moteur** : Bevy 0.12.1 (ECS) — version contrainte par l'environnement de développement
- **Physique** : bevy_rapier2d
- **Cible web** : WebAssembly via trunk

Le code suit un découpage modulaire par responsabilité :

```text
src/
├── main.rs      # Setup de l'app, plugins, ordonnancement des systèmes
├── states.rs    # Machine à états (menu / jeu / pause / transitions / game over)
├── player.rs    # Déplacement, tir, mort et respawn
├── asteroid.rs  # Spawn, déplacement et fragmentation
├── game.rs      # État du run (vies, niveau), fin de partie, wrap d'écran
├── hud.rs       # Interface (menus, vies, prompts)
├── audio.rs     # Intégration sonore
├── camera.rs    # Caméra 2D
└── entity.rs    # Composants partagés et nettoyage des entités
```

*(Pour les instructions de build et de lancement, voir le [README](/README.md) du dépôt.)*
