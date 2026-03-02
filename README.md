Crusty Engine

Experimental Rust game engine focused on fast iteration, deterministic simulation, and AI-assisted development.

⸻

What This Project Is

Crusty is a modular Rust game engine designed to:
	•	Run deterministic simulations
	•	Support AI-driven iteration and testing
	•	Keep the core engine small and composable
	•	Enable both headless simulation and visual rendering
	•	Make experimentation cheap and fast

This repository contains the engine core, CLI tooling, documentation, and a small web demo.

⸻

Who This Is For

You will get value from this project if you are:
	•	New to the codebase and want orientation
	•	Interested in ECS-based engines
	•	Experimenting with AI-assisted game development
	•	Building simulation-heavy or deterministic games
	•	Comfortable (or learning) Rust

If you are looking for a beginner game engine to ship a commercial title quickly, this project is still early-stage.

⸻

High-Level Architecture

engine/
  crates/
    engine-core/   ← main engine logic
    engine-cli/    ← command-line runner
docs/              ← deep technical docs
site/              ← static docs site + demo
game-concept/      ← design notes

Core Principles
	•	ECS-first architecture
	•	Deterministic simulation
	•	Headless-friendly
	•	AI iteration loops
	•	Data-oriented design

⸻

Key Concepts (Newcomer Friendly)

ECS (Entity Component System)

Crusty uses an ECS model:
	•	Entity → just an ID
	•	Component → data attached to entities
	•	System → logic that processes components

Core ECS implementation lives in:

engine-core/src/ecs/

Start with:
	•	world.rs
	•	entity.rs
	•	component_store.rs

⸻

Engine Core

The heart of the runtime is:

engine-core/src/engine.rs

This coordinates:
	•	Systems
	•	Physics
	•	Rendering
	•	Game state

If you want to understand the frame loop, start here.

⸻

Systems Layer

Gameplay behavior is implemented as systems:

engine-core/src/systems/

Examples:
	•	collision.rs
	•	gameplay.rs
	•	force_accumulator.rs
	•	behavior.rs

Think of systems as the “game logic pipeline.”

⸻

Physics

Physics utilities live in:

engine-core/src/physics/

Includes:
	•	Spatial grid
	•	Math helpers
	•	Continuous collision detection (CCD)

⸻

Rendering

Rendering is intentionally modular:

engine-core/src/rendering/

Includes:
	•	Framebuffer
	•	Particles
	•	Post FX
	•	Starfield
	•	Sprite and shapes

The renderer is designed to evolve independently from simulation.

⸻

Headless + AI Iteration (Important)

One of the more distinctive parts of this engine is:

engine-core/src/headless/

This supports:
	•	Automated experiments
	•	Fitness evaluation
	•	Hill climbing
	•	Golden comparisons
	•	Action generation

If your interest is AI-driven gameplay iteration, start here.

⸻

Getting Started

1. Prerequisites

You need:
	•	Rust (stable recommended)
	•	Cargo

Verify:

rustc --version
cargo --version


⸻

2. Build the Engine

From the repo root:

cd engine
cargo build


⸻

3. Run the CLI

cargo run -p engine-cli

This will execute the default engine entry point.

⸻

4. Explore the Docs

Recommended reading order:
	1.	docs/getting-started.md
	2.	docs/architecture.md
	3.	docs/engine.md
	4.	docs/ai-iteration.md
	5.	docs/api-reference.md

⸻

Web Demo

There is a small browser demo under:

site/

Open locally:

cd site
python -m http.server 8000

Then visit:

http://localhost:8000


⸻

Project Status

Reality check: this is an experimental engine.

What is relatively solid:
	•	ECS foundation
	•	Deterministic simulation direction
	•	Headless experimentation tooling
	•	Modular structure

What is still evolving:
	•	Public API stability
	•	Renderer maturity
	•	Tooling polish
	•	Cross-platform packaging
	•	Production ergonomics

Treat this as a research engine, not a finished product.

⸻

Where New Contributors Should Start

If you are trying to ramp up quickly:

First pass (30–60 min)
	•	docs/architecture.md
	•	engine-core/src/ecs/world.rs
	•	engine-core/src/engine.rs

Second pass
	•	systems/
	•	physics/
	•	headless/

Advanced
	•	Rendering pipeline
	•	AI iteration loops
	•	CLI integration

⸻

Design Goals

The engine is optimizing for:
	•	Fast iteration cycles
	•	AI-assisted development workflows
	•	Deterministic reproducibility
	•	Clear system boundaries
	•	Minimal hidden magic

It is not currently optimizing for:
	•	AAA graphics
	•	Editor tooling
	•	Beginner friendliness
	•	Large asset pipelines

⸻

Contributing

Before making large changes:
	1.	Read ENGINE_BOUNDARIES.md
	2.	Read ARCHITECTURE.md
	3.	Prefer small, composable additions
	4.	Preserve determinism where possible

⸻

Roadmap (Likely Directions)

Based on current structure, high-leverage future work includes:
	•	Platform abstraction layer
	•	Asset pipeline
	•	Save/load snapshots
	•	Better debug tooling
	•	Renderer decoupling
	•	Multiplayer determinism support

⸻

License

Check the repository root for license details.

⸻

If you are trying to extend the engine and want a targeted reading path, say what area you care about (rendering, ECS, AI iteration, etc.) and I will give you a focused onboarding map.
