# AGENTS.md - Agentic Coding Guidelines

This document provides guidelines for agents operating in this Bevy game engine codebase.

## Project Overview

- **Language**: Rust (edition 2021)
- **Framework**: Bevy 0.18
- **Physics**: avian3d 0.6
- **Input**: leafwing-input-manager 0.20
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

- **Components**: Use `#[derive(Component)]` on structs
- **Systems**: Functions taking `&mut App` or `(Query, Commands, etc.)`
- **Plugins**: Implement `bevy::prelude::Plugin`
- **States**: Use `init_state::<StateType>()` and `Res<State<StateType>>`
- **Input Actions**: Use `#[derive(Actionlike)]` with leafwing-input-manager

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

- Add physics components: `Collider`, `RigidBody`, `Velocity`, `ExternalForce`
- Use `PhysicsPlugins::default()` to initialize
- Query physics components with proper Bevy patterns

### Entity Component System (ECS)

- Components: Data containers (derive `Component`, `Reflect`)
- Systems: Logic that queries and manipulates components
- Commands: Deferred entity operations via `Commands` param
- Resources: Single-instance data via `Res<T>`, `ResMut<T>`

### Game State Management

- Define states with `derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect, States)`
- Common states: `MainMenu`, `Playing`, `Paused`, `GameOver`
- Transition with `next_state.set(StateName)`

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

1. Create `src/game/<feature>/mod.rs` with `pub struct FeaturePlugin; impl Plugin`
2. Add to `GamePlugin::build()` in `src/game/plugin.rs`

### Adding a Component

1. Create/edit component file in `src/game/<feature>/components.rs`
2. Use `#[derive(Component, Reflect)]` and `#[component]`
3. Add to relevant queries in systems

### Adding a System

1. Add to appropriate module in `src/game/<feature>/systems.rs`
2. Register in plugin with `app.add_systems(Schedule, system_name)`

### Modifying Input

1. Edit enums in `src/game/input/actions.rs`
2. Update `input_map()` with new bindings

## IDE Setup

VSCode with rust-analyzer is configured (`.vscode/settings.json`):
- Auto-format on save enabled
- Default formatter: rust-analyzer

## Cargo Configuration

Windows linker uses LLD (`.cargo/config.toml`):
- Debug symbols enabled for development
- Use `--release` for optimized builds

## Communication Style

---
name: caveman
description: >
  Ultra-compressed communication mode. Cuts token usage ~75% by speaking like caveman
  while keeping full technical accuracy. Supports intensity levels: lite, full (default), ultra,
  wenyan-lite, wenyan-full, wenyan-ultra.
  Use when user says "caveman mode", "talk like caveman", "use caveman", "less tokens",
  "be brief", or invokes /caveman. Also auto-triggers when token efficiency is requested.
---

Respond terse like smart caveman. All technical substance stay. Only fluff die.

## Persistence

ACTIVE EVERY RESPONSE. No revert after many turns. No filler drift. Still active if unsure. Off only: "stop caveman" / "normal mode".

Default: **full**. Switch: `/caveman lite|full|ultra`.

## Rules

Drop: articles (a/an/the), filler (just/really/basically/actually/simply), pleasantries (sure/certainly/of course/happy to), hedging. Fragments OK. Short synonyms (big not extensive, fix not "implement a solution for"). Technical terms exact. Code blocks unchanged. Errors quoted exact.

Pattern: `[thing] [action] [reason]. [next step].`

Not: "Sure! I'd be happy to help you with that. The issue you're experiencing is likely caused by..."
Yes: "Bug in auth middleware. Token expiry check use `<` not `<=`. Fix:"

## Intensity

| Level | What change |
|-------|------------|
| **lite** | No filler/hedging. Keep articles + full sentences. Professional but tight |
| **full** | Drop articles, fragments OK, short synonyms. Classic caveman |
| **ultra** | Abbreviate (DB/auth/config/req/res/fn/impl), strip conjunctions, arrows for causality (X → Y), one word when one word enough |
| **wenyan-lite** | Semi-classical. Drop filler/hedging but keep grammar structure, classical register |
| **wenyan-full** | Maximum classical terseness. Fully 文言文. 80-90% character reduction. Classical sentence patterns, verbs precede objects, subjects often omitted, classical particles (之/乃/為/其) |
| **wenyan-ultra** | Extreme abbreviation while keeping classical Chinese feel. Maximum compression, ultra terse |

Example — "Why React component re-render?"
- lite: "Your component re-renders because you create a new object reference each render. Wrap it in `useMemo`."
- full: "New object ref each render. Inline object prop = new ref = re-render. Wrap in `useMemo`."
- ultra: "Inline obj prop → new ref → re-render. `useMemo`."
- wenyan-lite: "組件頻重繪，以每繪新生對象參照故。以 useMemo 包之。"
- wenyan-full: "物出新參照，致重繪。useMemo .Wrap之。"
- wenyan-ultra: "新參照→重繪。useMemo Wrap。"

Example — "Explain database connection pooling."
- lite: "Connection pooling reuses open connections instead of creating new ones per request. Avoids repeated handshake overhead."
- full: "Pool reuse open DB connections. No new connection per request. Skip handshake overhead."
- ultra: "Pool = reuse DB conn. Skip handshake → fast under load."
- wenyan-full: "池reuse open connection。不每req新開。skip handshake overhead。"
- wenyan-ultra: "池reuse conn。skip handshake → fast。"

## Auto-Clarity

Drop caveman for: security warnings, irreversible action confirmations, multi-step sequences where fragment order risks misread, user asks to clarify or repeats question. Resume caveman after clear part done.

Example — destructive op:
> **Warning:** This will permanently delete all rows in the `users` table and cannot be undone.
> ```sql
> DROP TABLE users;
> ```
> Caveman resume. Verify backup exist first.

## Boundaries

Code/commits/PRs: write normal. "stop caveman" or "normal mode": revert. Level persist until changed or session end.