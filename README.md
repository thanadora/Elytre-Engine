# game_engine

Un moteur de jeu 2D écrit from scratch en Rust. Projet d'apprentissage visant à comprendre les mécanismes internes d'un moteur : architecture ECS, physique, rendu bas niveau avec wgpu.

---

## Ce que j'ai appris et construit

### Architecture ECS (Entity-Component-System)

C'est le cœur du projet, et la partie la plus aboutie.

**Entités avec générations (`src/ecs/entity.rs`)**

Les entités sont représentées par un `id` et une `generation`. Quand une entité est détruite, son ID est recyclé mais sa génération est incrémentée. Cela permet de détecter les références "zombies" vers des entités mortes — si quelqu'un garde une `Entity { id: 3, generation: 0 }` après que l'entité a été détruite et son slot réutilisé par `{ id: 3, generation: 1 }`, toutes les opérations sur l'ancienne référence échouent proprement.

```rust
let e1 = manager.create(); // { id: 0, generation: 0 }
manager.destroy(e1);
let e2 = manager.create(); // { id: 0, generation: 1 } — même slot, nouvelle vie
```

**SparseSet (`src/ecs/sparse_set.rs`)**

Structure de données centrale du stockage de composants. Elle combine :
- un tableau `sparse` indexé par l'ID d'entité qui pointe vers un index dense
- un tableau `dense` contenant les entités actives
- un tableau `data` contenant les composants correspondants

Avantages : itération très rapide (données contiguës en mémoire), insertion/suppression en O(1) amorti. La suppression utilise `swap_remove` pour rester dense sans décalage.

**Change tracking (`src/ecs/change_time.rs`)**

Chaque composant enregistre l'heure à laquelle il a été ajouté (`added`) et la dernière fois qu'il a été modifié (`changed`). On peut ensuite interroger le monde :

```rust
world.is_component_changed::<Position>(entity, last_frame_time)
world.is_component_added::<Health>(entity, last_frame_time)
```

Cela permet aux systèmes de ne traiter que les entités dont les données ont changé — une optimisation clé dans les moteurs de jeu modernes (Bevy l'appelle `Changed<T>` et `Added<T>`).

**World (`src/ecs/world.rs`)**

Le registre central. Il contient l'`EntityManager` et une `HashMap<TypeId, Box<dyn ComponentStorage>>` — chaque type de composant a son propre `SparseSet` typé, accessible par réflexion via `TypeId`. L'API publique :

```rust
let entity = world.spawn();
world.add_component(entity, Position { x: 0.0, y: 0.0 });
world.get_component::<Position>(entity);
world.get_component_mut::<Position>(entity);
world.iter_components::<Position>();
world.despawn(entity);
```

**Macro `get_storages_mut!`**

Le problème classique des ECS en Rust : comment obtenir des références mutables vers deux composants différents d'une même entité sans que le borrow checker se plaigne ? La solution est une macro qui vérifie à la compilation que les types sont distincts, puis utilise des pointeurs bruts (`unsafe`) pour accéder aux storages indépendamment.

```rust
if let Some((pos, vel)) = get_storages_mut!(world, entity, Position, Velocity) {
    pos.x += vel.x;
}
```

**Systèmes et Schedule (`src/ecs/system.rs`)**

Les systèmes sont simplement des closures `FnMut(&mut World)`. Le `Schedule` en stocke une liste et les exécute dans l'ordre à chaque frame.

```rust
schedule.add_system(|world: &mut World| {
    // logique ici
});
schedule.run(&mut world);
```

---

### Physique 2D

**Composants de base**

- `Vec2` : vecteur 2D avec add, sub, mul scalaire, produit scalaire, longueur au carré.
- `Transform2D` : position 2D d'une entité.
- `Velocity2D` : vitesse 2D.
- `Collider2D` : enum avec deux formes — `AABB { half_size }` (boîte alignée aux axes) et `Circle { radius }`.

**Systèmes**

- `movement_system` : applique la vélocité à la position à chaque frame.
- `gravity_system` : décrémente `vel.y` d'une constante à chaque frame.

**Détection de collisions (`src/physique/systems/Collision.rs`)**

Trois cas couverts avec leurs formules mathématiques :
- Cercle vs cercle : distance² < (r1 + r2)²
- AABB vs AABB : overlap sur les deux axes
- Cercle vs AABB : trouver le point du rectangle le plus proche du centre du cercle, comparer la distance au rayon

**Spatial hashing (`src/physique/spatial_grid.rs`)**

Grille de hash spatiale pour éviter les comparaisons O(n²) entre tous les colliders. Le monde est découpé en cellules de taille fixe. Chaque entité est insérée dans les cellules que son AABB ou bounding circle recouvre. La détection ne teste alors que les paires dans la même cellule.

---

### Rendu graphique (amorce)

**Fenêtre avec winit (`src/graphique/gameWindow.rs`)**

`winit` gère la création de fenêtre et la boucle d'événements de façon cross-platform (Windows, macOS, Linux). La `GameWindow` encapsule l'`EventLoop` et la `Window`.

**Contexte GPU avec wgpu (`src/graphique/graphicsContext.rs`)**

`wgpu` est une abstraction cross-platform au-dessus de Vulkan, Metal, DirectX 12 et WebGPU. L'initialisation suit toujours le même schéma :

1. Créer une `Instance` wgpu
2. Créer une `Surface` liée à la fenêtre
3. Choisir un `Adapter` (GPU physique)
4. Demander un `Device` (GPU logique) et une `Queue` (file de commandes)
5. Configurer la surface (format, taille, vsync)

**`GameEngine` (`src/gameEngine.rs`)**

Agrège la fenêtre et le contexte graphique. La boucle principale écoute les événements (`CloseRequested`, `RedrawRequested`) et appellera les systèmes de rendu.

---

### Désérialisation de scènes

`src/deserialize.rs` définit des structs pour charger des objets de jeu depuis du JSON avec `serde` :

```json
{
  "name": "Player",
  "position": { "x": 0.0, "y": 0.0 },
  "rotation": 0.0,
  "scale": { "x": 1.0, "y": 1.0 },
  "colliders": [],
  "scripts": [{ "type": "Health", "max_hp": 100 }],
  "sprite": "player.png"
}
```

---

## Ce qu'il reste à faire

### 🔴 Corrections urgentes (le projet ne compile pas en l'état)

- **Ajouter `wgpu` dans `Cargo.toml`** — le module graphique l'utilise mais il n'est pas déclaré comme dépendance.
- **Mettre à jour le code graphique vers l'API winit 0.30** — `winit 0.30` a cassé la compatibilité : `EventLoop::new()` retourne maintenant un `Result`, `Window::new()` disparaît au profit de `event_loop.create_window()`, et `ControlFlow::Exit` devient `EventLoopWindowTarget::exit()`. Voir le [guide de migration winit 0.30](https://github.com/rust-windowing/winit/blob/master/CHANGELOG.md).
- **Corriger les imports cassés dans la physique** — `spatial_grid.rs`, `Collision.rs`, `movement_system`, `gravity_system` référencent `crate::components::`, `crate::math::vec2::`, `crate::get_components_mut!` qui n'existent pas dans la structure actuelle.
- **Unifier les noms de champs** — `Velocity2D` expose un champ `vel` mais `gravity_system` et `movement_system` accèdent à `velocity.velocity.x`. Choisir une convention et s'y tenir.
- **Renommer les fichiers en snake_case** — `Collider2D.rs` → `collider_2d.rs`, `Collision.rs` → `collision.rs`, `Transform2D.rs` → `transform_2d.rs`, `Vec2.rs` → `vec2.rs`. Rust l'exige.

### 🟡 Fonctionnalités à compléter

- **Brancher l'ECS sur la physique** — les systèmes physiques doivent utiliser `world.iter_components::<Transform2D>()` (pas `world.query::<>()` qui n'existe pas).
- **Brancher la physique sur la boucle principale** — `GameEngine::run()` doit appeler `movement_system`, `gravity_system`, `spatial_hash_system` et `collision_system` à chaque frame.
- **Implémenter le rendu** — pour l'instant `RedrawRequested` est vide. Il faut créer un pipeline wgpu (shaders WGSL, vertex buffer, draw calls) pour afficher quelque chose à l'écran.
- **Brancher `deserialize.rs`** — créer une fonction qui lit un fichier JSON et spawn les entités avec leurs composants dans le `World`.
- **Décommenter et finir `Health.rs`** — définir le trait `Script` et l'API `GameObject` si cette direction est maintenue.

### 🟢 Améliorations futures

- **Réponse aux collisions** — pour l'instant `collision_system` détecte les collisions et `println!` le résultat. Il faut calculer le vecteur de séparation (MTV) et déplacer les entités.
- **Delta time réel** — `delta_time` est hardcodé à `0.016`. Il faut le mesurer avec `std::time::Instant` dans la boucle.
- **Système de rendu de sprites** — charger des images PNG et les afficher à la position du `Transform2D`.
- **Tests d'intégration** — le fichier `tests/Health.rs` montre l'intention. Écrire des tests qui font tourner plusieurs frames et vérifient le comportement physique.
- **Profiling** — une fois le moteur fonctionnel, mesurer les hotspots (probablement le spatial hash et les allocations dans les systèmes).

---

## Structure du projet

```
src/
├── main.rs               # Point d'entrée, démo de l'ECS
├── gameEngine.rs         # Boucle principale (fenêtre + systèmes)
├── deserialize.rs        # Chargement de scènes JSON
├── ecs/
│   ├── mod.rs
│   ├── entity.rs         # Entity + EntityManager (générations)
│   ├── sparse_set.rs     # Stockage de composants
│   ├── world.rs          # Registre central + macro get_storages_mut!
│   ├── system.rs         # Trait System + Schedule
│   └── change_time.rs    # Tracking des modifications de composants
├── physique/
│   ├── Vec2.rs           # Vecteur 2D
│   ├── spatial_grid.rs   # Hash spatial pour les collisions
│   ├── composant/
│   │   ├── Transform2D.rs
│   │   ├── Velocity2D.rs
│   │   └── Collider2D.rs
│   └── systems/
│       ├── movement.rs
│       ├── gravity.rs
│       └── Collision.rs
└── graphique/
    ├── gameWindow.rs     # Fenêtre winit
    └── graphicsContext.rs # Surface + Device wgpu

tests/
└── Health.rs             # Ébauche d'un système de scripts
```

## Dépendances

| Crate | Rôle |
|---|---|
| `winit 0.30` | Fenêtre et événements (clavier, souris, resize) |
| `wgpu` | Rendu GPU cross-platform (Vulkan / Metal / DX12 / WebGPU) — à ajouter dans Cargo.toml |
| `serde` + `serde_json` | Sérialisation / désérialisation JSON |
| `log` + `env_logger` | Logging structuré |
