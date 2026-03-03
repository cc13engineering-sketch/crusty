# vision.md — Crusty Engine

## What This Is

Crusty is the runtime substrate for AI-native game creation. It is a deterministic simulation engine designed so that AI agents can build, test, and iterate on games at machine speed.

The product vision: a person describes a game they want. The system builds it, validates it against thousands of simulations, and delivers a playable result. The person plays, gives feedback, and the system iterates. The engine is the part that makes this possible — not by understanding natural language or generating code, but by executing game logic deterministically, running simulations at high throughput, and reporting what happened with precision.

## What The Engine Is Responsible For

- Deterministic simulation. Same seed + same inputs = identical state, always.
- High-throughput headless execution. Thousands of runs in seconds.
- Structured observation. The engine reports what happened in a format agents can reason about.
- Replay fidelity. Any run can be recorded and reproduced exactly.
- A clean simulation boundary. Games plug into the engine through a trait. The engine doesn’t know what game it’s running.

## What The Engine Is Not Responsible For

- Understanding user intent. That’s the agent layer above.
- Generating game code or definitions. That’s the agent layer above.
- Deciding whether a game is good. That’s the validation layer, informed by the agent layer.
- Rendering to GPU, managing windows, or playing audio. The engine renders to a software framebuffer and queues sound commands. Hosts handle the rest.

## The Moat

The competitive advantage is not AI code generation. Everyone will have that. The advantage is the **validation loop**: generate a game, simulate it 50,000 times, prove it works, and deliver it with confidence. This requires determinism, throughput, and structured observation built into the engine’s DNA — not bolted on. Competitors starting from Unity or Godot would need to rebuild their runtime to achieve this. We started here.

The system gets better as models improve, without engine changes. Faster models = faster design iteration. Smarter models = better game designs fed into the same engine. Larger context windows = richer design briefs. The engine is the track. The models are the train. Better trains make the track more valuable, not less.

## Guiding Principles

### Determinism Is Non-Negotiable

If the simulation isn’t deterministic, replays are meaningless, sweeps are noise, and agents can’t learn from outcomes. Every engine feature must preserve determinism. No wall-clock dependencies. No unseeded randomness. No iteration-order dependence. No floating-point ambiguity in game logic.

When in doubt, choose the deterministic option even if it’s slower.

### The Engine Is A Platform, Not A Game

The engine knows about entities, components, systems, physics, rendering, and input. It does not know about enemies, health bars, levels, or score. Game concepts live in the game layer — either in Rust code implementing the Simulation trait, or in a declarative game definition format.

Never add game-specific logic to engine-core. If you’re writing code that mentions “player,” “enemy,” “bullet,” or “health” in engine-core, you’re in the wrong layer.

### Headless First, Visual Second

Every feature must work in headless mode. If it can’t run without a browser, it can’t be swept, and if it can’t be swept, it can’t be validated. Visual rendering is important for human playtesting but it is never the primary execution path.

When building a new system, make it work headless first. Add rendering as a separate pass.

### Declare, Don’t Hardcode

The engine should trend toward data-driven game definitions over time. Behavior rules, entity templates, state machines, game flow, visual themes — these are already partially declarative. The more game logic that can be expressed as data rather than code, the easier it is for AI agents to generate and modify games.

This doesn’t mean abandoning the Simulation trait. Rust code remains the escape hatch for novel mechanics that declarative systems can’t express. But common patterns should be declarable.

### Observations Over Pixels

AI agents need structured data, not screenshots. The observation layer should expose game state, entity positions, metrics, and events in a format that’s cheap to produce and easy to reason about. Framebuffer access should be optional, not the primary observation channel.

When adding game state or events, ask: “Can an agent understand what happened from the observation alone, without seeing the pixels?”

### Composable Over Configurable

Prefer small, orthogonal building blocks that combine in interesting ways over large configurable systems with many parameters. A force field + a trigger zone + a behavior rule can express “wind that pushes the player when they enter a region” without a dedicated wind system. The fewer specialized systems the engine has, the more games it can express.

### Allocation-Aware On Hot Paths

The simulation loop runs millions of times during sweeps. Avoid heap allocation in per-frame code. Pre-allocate buffers. Reuse collections. Profile before optimizing, but design for allocation-free hot paths from the start.

## Architecture Layers

```
┌─────────────────────────────────────────────┐
│  Agent Layer (not in this repo)             │
│  - Understands user intent                  │
│  - Generates game definitions               │
│  - Proposes changes, reviews results        │
│  - Reads sweep data, diagnoses issues       │
└──────────────────┬──────────────────────────┘
                   │ game.toml / Simulation trait
┌──────────────────▼──────────────────────────┐
│  Game Layer                                 │
│  - Game definitions (declarative)           │
│  - Custom Simulation impls (Rust)           │
│  - Entity templates, behavior rules         │
│  - Visual themes, difficulty parameters     │
└──────────────────┬──────────────────────────┘
                   │ Engine API
┌──────────────────▼──────────────────────────┐
│  Engine Core                                │
│  - ECS (World, Components, Systems)         │
│  - Physics (fixed timestep, CCD)            │
│  - Rendering (framebuffer, shapes, text)    │
│  - Input (InputFrame application)           │
│  - RNG (seeded, engine-owned)               │
│  - State hashing, diagnostics, metrics      │
└──────────────────┬──────────────────────────┘
                   │ Headless API
┌──────────────────▼──────────────────────────┐
│  Headless / Tooling Layer                   │
│  - HeadlessRunner (Simulation + Policy)     │
│  - Replay recording and verification        │
│  - Sweep / batch execution                  │
│  - Golden tests, regression suites          │
│  - Fitness evaluation, anomaly detection    │
│  - CLI (record, replay, batch, sweep, etc.) │
└──────────────────┬──────────────────────────┘
                   │ JSONL / Observation
┌──────────────────▼──────────────────────────┐
│  Validation Layer                           │
│  - Metric targets (from design brief)       │
│  - Coherence checks (reachability, etc.)    │
│  - Difficulty curve analysis                │
│  - Comparison (before/after)                │
│  - Promotion / rejection decisions          │
└─────────────────────────────────────────────┘
```

## Key Abstractions

### Simulation Trait

The contract between a game and the engine. Games implement `setup`, `step`, and `render`. The engine handles timing, input application, determinism, and observation. A game can be 50 lines or 5,000 lines. The engine doesn’t care.

### InputFrame

The canonical representation of one frame of player input. Keys pressed, keys held, pointer position, pointer events. Serializable. This is what replays store and what policies produce.

### Observation

A lightweight, zero-allocation view into engine state after each frame. Frame number, state hash, game state reference, entity count, optional framebuffer reference. This is what policies consume and what agents reason about.

### Policy

A pluggable input generator. Takes an Observation, produces an InputFrame. Random policies explore. Scripted policies replay. Smart policies (future) optimize. The engine records what the policy does, so every policy-driven run is replayable.

### Design Brief (Future)

A structured document expressing the user’s intent: genre, tone, session length, difficulty philosophy, success metrics, aesthetic preferences. The generation agents read this. The validation layer checks against it. The sweep metrics are derived from it.

### Game Definition (Future)

A declarative file format that describes a complete game: entity types, behaviors, rules, generation parameters, win/lose conditions, visual theme, difficulty curve. The engine interprets this without compilation. AI agents read and write it. This is the compilation target for the product.

The engine already has the building blocks for this:

- `EntityTemplate` for entity prefabs
- `BehaviorRules` with condition → action logic (collision, trigger, timer, state check)
- `GameFlow` for lifecycle state machine (Title → Playing → GameOver)
- `StateMachine` for per-entity FSM
- `ColorPalette` for visual identity
- `SchemaInfo` trait for component introspection

These need to be unified into a single coherent format.

## Current State

### What’s Solid

- ECS with 32 component types, clean separation
- 21 systems in fixed execution order
- Fixed-timestep physics at 60Hz
- Software renderer (framebuffer, shapes, text, particles, post-fx)
- Headless infrastructure: 18 modules covering replay, golden tests, sweeps, anomaly detection, fitness evaluation, hill climbing, regression suites, strategies, test harnesses
- Declarative building blocks: behavior rules, templates, state machines, game flow
- No external runtime dependencies beyond wasm-bindgen and serde
- Clean platform boundaries (ENGINE_BOUNDARIES.md)

### What’s In Progress (Main Implementation Plan)

- Engine-owned seeded RNG (replacing scattered implementations)
- State hashing (simulation state, not just pixels)
- Fixed dt for all simulation systems
- Simulation trait and InputFrame
- HeadlessRunner rewrite against Simulation/Policy
- Replay serialization to JSONL
- Golden test CI gate
- Turbo mode (skip rendering)
- Batch runner and policy-driven sweeps
- CLI buildout

### What’s Next (After Main Plan)

See the product trajectory section below.

## Product Trajectory

This is the sequence of capabilities that takes the engine from developer tool to product substrate. Each step is valuable on its own. Each step makes the next one possible.

### Stage 1: Simulation Platform (Main Implementation Plan)

The engine runs games deterministically, headless, at high throughput. Games are Rust code. The CLI drives everything. Sweeps produce JSONL. Golden tests gate CI. This is the foundation.

**You are here.**

### Stage 2: Validation Games

Build 3–5 genuinely different games on the engine. A physics puzzle, a navigation game, a tower defense, a roguelike, something with resource management. These are research, not product. They reveal where the Simulation trait creaks, where the observation layer is too thin, where the engine’s component vocabulary is missing concepts.

Each game should have:

- A Simulation implementation
- Golden tests
- A random and heuristic policy
- Sweep-derived difficulty curves
- At least one metric target that the sweep validates

This stage answers the question: “Is the engine actually general enough?”

### Stage 3: Game Definition Format

Extract the patterns from Stage 2 into a declarative format. Entity types defined in data. Behavior rules in data. Win/lose conditions in data. Generation parameters in data. Visual theme in data.

The format doesn’t need to express every possible game. It needs to express the common 80%. The Simulation trait remains the escape hatch for the other 20%.

The engine includes an interpreter that loads a game definition and runs it as a Simulation. No Rust compilation needed. AI agents can now generate games by producing data files.

### Stage 4: Design Brief and Metric Targets

Define a structured format for user intent. Genre, tone, difficulty, session length, core mechanics. The design brief maps to metric targets: survival rate, completion rate, session length, decision frequency, state-space coverage.

Generation agents read the brief and produce game definitions. The validation layer checks the game against the brief’s metric targets. A game “passes” when the sweep confirms it meets all targets.

### Stage 5: Coherence Validation

Before running expensive sweeps, fast structural checks on the game definition. Can the player reach the goal? Do enemies and player interact? Is there at least one input that changes state? Is the win condition achievable?

These are graph checks on the game definition, not simulation. They catch nonsense in milliseconds rather than burning compute.

### Stage 6: Generation Pipeline

The full loop: user prompt → design brief → game definition → validation → playable game. An agent expands the prompt into a brief. Another agent generates a game definition from the brief. The engine validates it. The user plays it and gives feedback. The system iterates.

This is the product.

### Stage 7: Autonomous Iteration

The system proposes improvements to existing games. Parameter tuning runs unattended (optimize within declared metric targets). Feature proposals are generated, built, swept, and queued for human review. Nightly evolution runs explore the design space.

The human’s role shifts from builder to curator: approving or rejecting changes, setting new constraints, steering the creative direction.

## Things We Believe

- The declarative game definition format is the most important product decision we’ll make. It determines what games can be generated, how agents interact with game logic, and how games are versioned and diffed. Get this right and everything downstream works. Get it wrong and no amount of AI sophistication helps.
- The validation loop is more valuable than the generation loop. A system that can reliably tell you “this game is broken” is more useful than a system that can generate games but can’t check them. Build validation first.
- Visual identity should be constrained, not generated. A product with a recognizable, consistent aesthetic (like Pico-8 or Playdate) is better than one that attempts to generate arbitrary art. Constraints are a feature. The current software renderer with ColorPalette, shapes, particles, and post-fx can produce attractive games if the vocabulary is rich enough and consistently applied.
- The engine should never be smart. It should be fast, deterministic, and honest. Intelligence belongs in the agent layer. The engine is a truth machine. It runs what you give it and tells you what happened. The smarter the agents get, the better the games they produce, but the engine’s contract never changes.
- HashMap iteration order will bite us. Component stores use HashMap. If any system’s behavior depends on iteration order, determinism is compromised. This should be addressed structurally (BTreeMap or sorted iteration) rather than chased as intermittent bugs.
- Cross-platform floating-point determinism is a real risk but not a blocker. Native-to-native determinism is the priority. WASM parity is a nice-to-have. The AI workflow runs headless and native. Browser builds are for human playtesting.
- We should resist building the agent layer prematurely. The engine and tooling layer need to be solid before building orchestration on top. Every time we’re tempted to build “engine-cli agent propose-fixes,” we should instead do that workflow manually and see what information actually flows between steps.

## Explicit Non-Goals

These are things we will not build, at least not in the near term:

- **GPU rendering.** The software renderer is the right choice for determinism, headless execution, and WASM portability. It’s also the right choice for a constrained visual identity.
- **Editor tooling.** The engine is operated through the CLI and through code/data. A visual editor is a separate product.
- **Network multiplayer.** Deterministic lockstep multiplayer is interesting but orthogonal to the product vision.
- **Asset pipelines.** Generated games use the engine’s built-in visual vocabulary (shapes, particles, palettes, post-fx), not imported sprites or 3D models.
- **Plugin architecture.** Games are Simulations or game definitions. There’s no plugin loading, no scripting language, no dynamic linking.
- **ECS archetype rewrite.** The HashMap-based component stores work. If performance becomes a problem (unlikely at our entity counts), upgrade individual stores to Vec-based slots. Don’t rewrite the ECS.

## How To Use This Document

This is the north star for development decisions. When evaluating a feature, change, or design:

1. Does it preserve determinism? If no, don’t do it.
1. Does it work headless? If no, make it work headless first.
1. Does it strengthen the validation loop? If yes, prioritize it.
1. Does it make the engine smarter or the agent layer’s job easier? Prefer the latter.
1. Does it get better as models improve? If yes, it adds to the moat. If it gets worse or becomes irrelevant, reconsider.
1. Can it be expressed as data rather than code? If yes, lean toward data.
1. Does the engine need to know about it, or can it live in the game layer? Keep the engine thin.

When in doubt, build the smallest thing that works, validate it with a sweep, and iterate.
