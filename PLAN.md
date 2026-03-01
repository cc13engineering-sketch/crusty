# Crusty Engine — Master Plan

Based on the 4 items from your previous session that hit the usage limit.

---

## Phase 0: Pre-Work Codebase Audit (New Innovation Game Step)

**Goal**: Before any new innovation rounds, multi-disciplinary agents review the entire codebase and fix rot.

### What gets reviewed:
1. **Rust Engine Expert** — Reviews all engine source (`engine/crates/engine-core/src/`) for:
   - Dead code, unused imports, stale modules
   - API mismatches between REVIEW.md findings and actual code (e.g., `mod scripting` visibility, despawn/NameMap bug — are these fixed?)
   - Components/systems that were proposed but never implemented (World Snapshot was "deferred" in Round 3)
   - Test gaps — 850 tests claimed, verify count and coverage

2. **Game Developer / Demo Reviewer** — Reviews all demos and web files:
   - Do `site/game-1`, `site/game-2`, `site/game-3`, `innovations-1` actually compile and run against the current WASM build?
   - Are the `.world` files using current grammar (rounds added new grammar features)?
   - Does `innovations-1/game.js` reference WASM exports that actually exist?

3. **Documentation / Prompt Accuracy Reviewer** — Reviews all `.md` files for accuracy:
   - `engine/CLAUDE.md` — Are Innovation Games rounds 4-7 documented? (Currently only rounds 1-3 are listed)
   - `PROCESS.md` — API pitfalls section: verify every code example compiles against current source
   - `REVIEW.md` — Original review findings: which are fixed, which are still open?
   - `CHANGELOG.md` — Does it match actual commit history and features?
   - `engine/ARCHITECTURE.md` — Is it current with 32 components, 21 systems?

### Deliverables:
- Fix all identified issues
- Update all `.md` files to reflect current engine state
- Add rounds 4-7 to `engine/CLAUDE.md` Innovation Games section
- Confirm all demos build and run
- Commit: "pre-work: codebase audit and documentation sync"

---

## Phase 1: Innovation Rounds 1-2 — Mobile Game Design Automation

**Theme**: Innovations that allow Claude Code to design high-quality immersive mobile games with minimal human input.

**Focus areas to explore** (agents should think about ALL of these):
- Declarative game definition — can we extend `.world` DSL so an entire mobile game can be specified without custom Rust?
- Auto-layout and responsive design — engine-level mobile screen adaptation
- Touch gesture vocabulary — beyond tap: swipe, pinch, drag, long-press, multi-touch
- Procedural content pipelines — seeded level generation, enemy placement, difficulty curves
- Visual polish automation — auto-particles, juice effects, screen shake, transitions triggered by game events
- Audio system — sound effects, music, spatial audio (currently missing entirely)
- Game template system — "start a new game of type X" that scaffolds everything
- AI-assisted art — procedural sprites, palettes, tile generation
- Playtesting automation — bot that plays the game, measures fun metrics
- One-command deploy — `cargo run -- deploy` that builds WASM, assembles site, pushes to GH Pages

**Format**: 4 agents each propose 3-5 engine innovations. Cross-review. Best ideas get implemented with tests.

---

## Phase 2: Innovation Rounds 3-4 — Game Concept Design Competition

**Theme**: Large map tile-based RPG, Pokémon-style, but instead of fighting trainers, players encounter traps where they must use minigolf-like mechanics to fight/solve their way through an encounter.

**Round 3 — Initial Proposals**: Each agent produces a rich, ready-to-build game concept doc covering:
- World design (map regions, biomes, progression gates)
- Core minigolf-trap mechanics (how encounters work, physics tuning, variety)
- RPG systems (character progression, inventory, abilities that modify golf physics)
- NPC/story elements
- Visual style and UI/UX
- Technical architecture (which engine features are needed, which are missing)

**Round 4 — Refinement**: Agents review all Round 3 proposals, then each produces a refined "final spec" that cherry-picks the best ideas across all proposals into one cohesive design. Vote on final concept.

**Deliverable**: A single agreed-upon game design document committed as `game-concept/DESIGN.md`.

---

## Phase 3: Innovation Rounds 5-6 — Engine Improvements for the Game

**Theme**: Code and engine improvements specifically to make building the agreed-upon RPG/minigolf game feasible.

Likely gaps to address (based on current engine state):
- Large scrolling tile maps with multiple layers (current TileMap is single-layer)
- NPC system / dialogue trees (current DialogueQueue is basic)
- Inventory / equipment system
- Save/load for RPG state (basic Save/Load exists but may need extension)
- Mini-game subsystem (the minigolf encounters need isolated physics contexts)
- Map editor / map format for large worlds
- Sprite animation states tied to character movement
- Sound system
- Menu / UI system

**Format**: 4 agents each propose engine changes. Cross-review. Implement winning proposals with tests.

---

## Phase 4: Ongoing Innovation Rounds (7+) — Feature Competition

**Theme**: Open feature ideas, continuing indefinitely until told to stop.

Each round:
1. 4 agents propose feature ideas (new components, systems, tools, demos)
2. Cross-review and vote
3. Implement winners with tests
4. Build/update demos as appropriate
5. Publish changelog (see below)

---

## Recurring Step: Changelog Publishing (Every Round)

**Added to every innovation round as a mandatory step.**

After each round's code is merged:
1. Update `CHANGELOG.md` with the round's additions
2. Generate/update a public changelog page at `site/changelog/index.html` (or similar) that renders nicely on GitHub Pages
3. Include "How to Run" instructions for all completed demos
4. Ensure the GitHub Actions workflow (`deploy.yml`) includes the changelog page and any new demo directories
5. Commit and push so GH Pages auto-deploys

The changelog page should be browsable at the project's GitHub Pages URL alongside the demo games.

---

## Changes to CLAUDE.md

The following will be added to `engine/CLAUDE.md`:

### Innovation Games Process (Updated)

```
## Innovation Games Process

### Pre-Work: Codebase Audit (runs before each innovation round)
Before starting proposals, spawn agents with different technical backgrounds to:
1. Review engine code for outdated patterns, dead code, API drift
2. Review all demos — verify they compile and run against current WASM build
3. Review all .md files (CLAUDE.md, PROCESS.md, REVIEW.md, CHANGELOG.md, ARCHITECTURE.md) for accuracy
4. Fix issues found, update docs, commit as "pre-work: codebase audit"

### Innovation Round Steps
1. **Pre-work audit** (see above)
2. **Spawn competing agents** — each proposes features/ideas independently
3. **Theme-driven ideation** — proposals validated against round's theme
4. **Cross-pollinate** — review agents select best ideas across competitors
5. **Integrate** — winning features implemented with tests
6. **Demo** — build/update demo games showcasing new features
7. **Changelog publish** — update CHANGELOG.md, publish to GitHub Pages site with "how to run" info for demos

### Round Schedule
- Rounds 1-2: Mobile game design automation (minimal human input)
- Rounds 3-4: Game concept competition (tile-based RPG + minigolf traps)
- Rounds 5-6: Engine improvements for the agreed game concept
- Rounds 7+: Open feature competition (ongoing until told to stop)
```

---

## Execution Order

1. Create/update `engine/CLAUDE.md` with the new Innovation Games process
2. Run Phase 0 (pre-work audit) — fix all issues
3. Start Phase 1 (innovation rounds 1-2)
4. Continue through phases sequentially
5. Each round: audit → propose → review → implement → test → publish changelog

---

## Questions for You

- **GitHub Pages URL**: What is your repo's GH Pages URL? (Needed to verify changelog publishing works)
- **Round pacing**: Do you want me to complete all rounds in one session, or pause between phases for your review?
- **Game concept input**: For Phase 2, do you have any additional preferences for the RPG/minigolf game beyond what you described?
