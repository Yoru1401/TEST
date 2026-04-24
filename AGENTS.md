# AGENTS.md - Agentic Coding Guidelines

This document provides guidelines for agents operating in this Bevy game engine codebase.

## Project Overview

- **Language**: Rust (edition 2021)
- **Framework**: Bevy 0.18
- **Physics**: Custom system built on avian3d 0.6 (spatial queries), NOT rigid body simulation
- **Input**: leafwing-input-manager 0.20 (action states as components)
- **UI**: univis_ui 0.2.0-alpha.1 (separate from Bevy UI)
- **Build Target**: x86_64-pc-windows-msvc

## Build Commands

```bash
cargo build        # Build the project
cargo build --release   # Build with optimizations
cargo run           # Build and run
cargo run --release # Run release build
```

## Testing

```bash
cargo test              # Run all tests
cargo test <test_name> # Run single test
cargo test -- --nocapture  # Show print output
```

To run a specific test, use the exact function name:
```bash
cargo test test_player_movement
```

## Linting & Formatting

```bash
cargo clippy          # Run linter
cargo clippy -- -D warnings  # Treat warnings as errors
cargo fmt            # Format code
cargo fmt -- --check  # Check formatting without changes
```

## Code Style Guidelines

### Imports

- Use `bevy::prelude::*` for common Bevy types via the crate prelude
- Use absolute paths from crate root: `crate::game::plugin::GamePlugin`
- External crates: `use bevy::prelude::*;`, `use leafwing_input_manager::prelude::*;`
- Group imports: standard library, external crates, local modules

### Formatting

-rust-analyzer auto-formats on save (enabled in `.vscode/settings.json`)
- Run `cargo fmt` before committing

### Naming Conventions

- **Types/Structs**: `PascalCase` (e.g., `PlayerMarker`, `GameState`)
- **Enums**: `PascalCase` for enum and variants (e.g., `PlayerAction::Jump`)
- **Functions/Methods**: `snake_case` (e.g., `setup_playground`, `enter_playground`)
- **Variables**: `snake_case` (e.g., `current_state`, `next_state`)
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Files**: `snake_case.rs` (e.g., `game_plugin.rs`, `components.rs`)

### Bevy-Specific Patterns

- **Components**: Use `#[derive(Component, Reflect)]` on structs; always add `#[component]` for reflect-based serializable components
- **Marker Components**: Empty structs like `PlayerMarker`, `CameraMarker` for tagging entities (no data, just identity)
- **Systems**: Functions taking system params. All game logic lives in systems, not in plugins
- **Plugins**: Implement `bevy::prelude::Plugin` to group related setup and systems
- **States**: Use `init_state::<GameState>()` in plugins; `#[derive(States)]` on enums for state transitions
- **Input Actions**: Use `#[derive(Actionlike)]` + `InputManagerPlugin` + `ActionState<T>` component on entities

### Custom Physics Gotchas (CRITICAL)

This project uses a **custom physics system**, NOT avian3d's built-in simulation:

- Uses avian3d for `Collider`, `RigidBody`, `SpatialQuery`, `MoveAndSlide` — but NOT RigidBody dynamics
- Entities have `Collider::sphere/cuboid(...)` and `CustomPositionIntegration` (not default integration)
- Physics is manually calculated in `physics/systems.rs`: `apply_forces` → `accumulate_forces` → `resolve_collisions`
- Key types: `PhysicsVelocity`, `ForceApplier`, `GroundState`, `Contacts`, `PhysicsConfig`
- To add player movement: Query `ForceApplier` and `GroundState`, call `force_app.add_force()` or `.add_impulse()`
- Drag calculation uses quadratic velocity scaling in `accumulate_forces`: `force -= vel_dir * drag * speed * speed`
- Ground detection uses `spatial_query.cast_shape()` with sphere collider facing down, NOT collision events

### Error Handling

- Use `Result<T, E>` for fallible operations
- Use `?` operator for propagating errors
- Use `expect()` or `unwrap()` only for truly irrecoverable errors
- Log errors appropriately: `log::error!("message: {:?}", e)`

### Comments

- Avoid unnecessary comments; code should be self-documenting
- Use doc comments (`///`) for public APIs
- Use inline comments (`//`) sparingly for non-obvious logic

### Types

- Use `f32` for game math (Bevy standard)
- Use `i32` for grid/index positions
- Prefer Bevy's built-in types: `Vec3`, `Vec2`, `Transform`, `Quat`
- Use `bevy::prelude::*` for common types

### Bevy Systems

- Register systems with `app.add_systems(Schedule, system_fn)`
- Common schedules: `OnEnter`, `OnExit`, `Update`, `PostUpdate`, `PreUpdate`
- Use `Query` for component access, `Commands` for entity manipulation
- System params must be registered in function signature order

### Physics (avian3d)

- Add physics components: `Collider`, `RigidBody`, `PhysicsVelocity` (custom, NOT avian's Velocity)
- Use `PhysicsPlugins::default()` to initialize (registers spatial queries)
- Query physics components with proper Bevy patterns
- Useavian3d prelude for: `Collider`, `RigidBody`, `SpatialQuery`, `ShapeCastConfig`, `SpatialQueryFilter`, `CustomPositionIntegration`, `MoveAndSlide`

### Entity Component System (ECS)

- **Components**: Data containers. Use `#[derive(Component, Reflect)]` for data, empty structs for markers
- **Marker Components**: Empty structs tag entities: `PlayerMarker`, `CameraMarker`, `GameWorldSpawned` (marker for "has this been spawned yet")
- **Custom Physics Components**: `PhysicsVelocity`, `ForceApplier`, `GroundState`, `Contacts` — these replace avian's built-in physics
- **Systems**: All game logic lives here. Take system params in specific order for scheduling
- **Commands**: Deferred entity operations via `Commands` param
- **Resources**: Single-instance data via `Res<T>`, `ResMut<T>` (e.g., `State<GameState>`)

### Game State Management

- Define states with `#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect, States)]`
- States: `MainMenu`, `Playing`, `Paused`
- Use `is_running()` from `crate::game::setup` or `crate::game::states` for `.run_if()` conditions
- **GOTCHA**: Two `is_running()` functions exist — `game/setup/functions.rs` and `game/states.rs`. Use either; both do same check.

### Input System (leafwing-input-manager)

- Define actions with `#[derive(Actionlike)]`: `PlayerAction`, `CameraAction`, `GlobalAction`
- Action enums can have `#[actionlike(DualAxis)]` for analog input (returns `Vec2`)
- Each action has `impl PlayerAction { pub fn input_map() -> InputMap<Self> }` that creates `InputMap::default()`
- Add input map to entities: `player_cmd.insert(PlayerAction::input_map())`
- Add action state component: `player_cmd.insert(ActionState::<PlayerAction>::default())`
- Query in systems: `Query<&ActionState<PlayerAction>>`

### Performance

- Prefer `PostUpdate` for physics and transform updates
- Use `Option<T>` instead of nullable pointers
- Mark structs as `#[derive(Clone, Copy)]` only if truly cheap to clone
- Use `bevy::utils::hashbrown` for collections in hot paths

### Testing Patterns

- Test systems in isolation with `World` and `Schedule`
- Use `SystemState` to populate system params
- Assert on `Query` results, `Resource` values, or entity counts

## Common Tasks

### Adding a New Plugin

1. Create `src/game/<feature>/mod.rs` with:
   ```rust
   pub struct FeaturePlugin;
   impl Plugin for FeaturePlugin { fn build(&self, app: &mut App) { ... } }
   ```
2. Export in `src/game/mod.rs`: `pub mod <feature>; pub use <feature>::FeaturePlugin;`
3. Add to `GamePlugin::build()` in `src/game/plugin.rs`: `app.add_plugins((..., FeaturePlugin))`

### Adding a Component

1. Create/edit `src/game/<feature>/components.rs`
2. Use `#[derive(Component, Reflect)]` for data or empty struct for marker
3. Add to relevant queries in systems

### Adding a System

1. Add to `src/game/<feature>/systems.rs`
2. Register in plugin: `app.add_systems(Schedule, system_name)`
3. Add `.run_if(is_running)` to run only when `GameState::Playing`

### Adding Input Actions

1. Edit or add enum in `src/game/input/actions.rs`:
   ```rust
   #[derive(Actionlike, Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
   pub enum NewAction { Jump, Move }
   impl NewAction {
       pub fn input_map() -> InputMap<Self> { ... }
   }
   ```
2. Insert on entity: `entity.insert(NewAction::input_map()); entity.insert(ActionState::<NewAction>::default());`
3. Query: `Query<&ActionState<NewAction>>` or `Query<(Entity, &ActionState<NewAction>)>`

### Adding Custom Physics Movement

1. Query `(With<Foo>, &mut ForceApplier, &GroundState, &PhysicsVelocity)`
2. Call `force_app.add_force(vector * speed)` or `.add_impulse(vector * impulse)`
3. Forces are accumulated in `PreUpdate`, applied in `Update`, resolved in `PostUpdate`

## Project Architecture

### Core Flow

1. `main.rs` → creates `App` → adds `GamePlugin`
2. `GamePlugin` registers all sub-plugins: `PhysicsPlugin`, `InputPlugin`, `PlayerPlugin`, `CameraPlugin`, `GrapplePlugin`, `UIPlugin`
3. On state `Playing` enter: `setup_playground` spawns player, camera, ground, walls, stairs, ramps, light
4. Player input system reads `ActionState<PlayerAction>` component, applies forces via `ForceApplier`
5. Physics systems: forces accumulate → collisions resolve → positions update

### Plugin Dependencies

- `PhysicsPlugin` must init before any physics-using plugin (adds `PhysicsPlugins::default()`)
- `InputPlugin` must init before `PlayerPlugin` (user input required for movement)
- All plugins use `.run_if(is_running)` (only active in `Playing` state)

### Game State Transitions

- `MainMenu` → (start) → `Playing` → (escape/pause) → `Paused` → (unpause) → `Playing`
- UI systems handle state transitions via button callbacks

## UI Library

- Uses `univis_ui` (NOT Bevy's built-in UI system)
- Register with: `app.add_plugins(univis_ui::prelude::UnivisUiPlugin)`
- UI elements live in `src/game/ui/` (main_menu.rs, pause_menu.rs)
- State transitions triggered by UI callbacks

## IDE Setup

VSCode with rust-analyzer is configured (`.vscode/settings.json`):
- Auto-format on save enabled
- Default formatter: rust-analyzer

## Cargo Configuration

Windows linker uses LLD (`.cargo/config.toml`):
- Debug symbols enabled for development
- Use `--release` for optimized builds
