# design-acceleration.md — Features That Make the Hard Parts Faster

## Purpose

The hardest part of building a game isn't engineering. It's design iteration: deciding whether a mechanic is fun, whether the difficulty feels right, whether a change helped or hurt. These decisions require human taste, but taste operates on information — and the faster you get the right information, the faster your taste can work.

This document describes engine features designed to compress the feedback loop between "I wonder if this works" and "now I can see whether it works." None of these replace the designer. All of them give the designer better information, faster.

These features build on the existing infrastructure: deterministic simulation, seeded RNG, headless execution, sweeps, replays, and the Simulation trait. They are listed roughly in order of implementation complexity, simplest first.

-----

## Feature 1: Variant Branching

### The Problem

Testing two versions of a mechanic — "enemies chase you" vs "enemies patrol and ambush" — currently requires implementing one, playing it, forming an opinion, implementing the other, playing it, and comparing from memory. Memory is unreliable. The comparison is slow.

### The Feature

The Simulation trait gains an optional method:

```rust
fn variants(&self) -> Vec<(&str, ParamSet)> { vec![] }
```

A `ParamSet` is a named bundle of game parameters (physics constants, spawn rates, AI behaviors, anything the game exposes as tunable). The engine can hot-swap between variants during play with a keystroke. Because the simulation is deterministic, it can also rewind to any frame and replay forward with a different variant, letting you compare the same moment under different parameters.

### How It Helps

You play your game. You hit a moment that feels off — maybe the enemies are too aggressive. You press a key and instantly you're playing the same scenario with the "passive enemies" variant. The comparison is side-by-side in time, not separated by twenty minutes of reimplementing and replaying. Your taste has the information it needs immediately.

### Extension

Variants can also be swept. "Run 10,000 games with variant A and 10,000 with variant B, compare survival rates." This connects the feel-testing (play both variants) with the data-testing (sweep both variants) and lets you confirm that what feels better also measures better.

-----

## Feature 2: Death Classification

### The Problem

Sweeps produce statistics: "30% survival rate." But the same survival rate can feel completely different depending on *how* players die. Deaths where the player was making progress and narrowly failed feel fair. Deaths where the player never had a chance feel cheap. The number alone doesn't tell you which kind you have.

### The Feature

The engine automatically classifies terminal states (deaths, losses, game-overs) based on the trajectory leading up to them:

- **Close call**: score or progress was increasing until the final frames. The player was engaged and almost succeeded.
- **Blowout**: score or progress was flat or zero for a sustained period before death. The player was stuck or overwhelmed from the start.
- **Cliff**: progress was high and then dropped suddenly. Something broke the player's run unexpectedly.
- **Attrition**: progress was slowly declining over time. The player was losing ground gradually.

Classification is based on simple trajectory analysis of tracked metrics (score, health, distance, whatever the game reports) over the last N frames before the terminal state. No ML. Just slope and variance calculations.

### How It Helps

Instead of "30% survival rate," you see "30% survival rate: 60% close calls, 20% blowouts, 15% cliffs, 5% attrition." A game with mostly close calls feels fair and tense. A game with mostly blowouts has an early-game problem. A game with lots of cliffs has a surprise-kill problem. The classification tells you what kind of difficulty problem you have, not just that you have one.

### What The Designer Does With It

"Rooms 6-8 have 70% blowout deaths" is actionable. Play the blowout replays for those rooms. See what's happening. Fix it. Re-sweep. Confirm the blowout rate dropped. This turns a vague feeling ("the mid-game feels unfair") into a specific diagnosis ("room 7 spawns too many enemies before the player has a weapon upgrade").

-----

## Feature 3: Divergence Replay

### The Problem

After changing your game, the sweep tells you something got worse — survival rate dropped, game length changed, a metric shifted. But you don't know *why*. You know the before and the after. You don't know the moment where they diverge.

### The Feature

The engine can diff two runs (or two sweeps) and find the first frame where behavior meaningfully diverges. Given two playthroughs of the same seed — one from the old version, one from the new — it walks forward frame by frame comparing state hashes until they differ, then presents that frame as a replay with both versions side by side.

For sweeps, it finds the seeds where the outcome changed the most (e.g., seeds that survived in version A but died in version B) and presents representative divergence replays for those seeds.

### How It Helps

You changed the enemy spawn rate and survival dropped. The divergence replay shows you: on seed 4217, at frame 340, the old version had two enemies on screen and the new version had four. The player couldn't handle four. Now you know exactly what happened, on a specific seed, at a specific frame. You don't need to guess why the metric moved.

### Implementation Note

This is cheap because state hashing already exists. Comparing two replays frame-by-frame is just comparing two sequences of u64 hashes. The first mismatch is the divergence point. The replay infrastructure already supports jumping to any frame.

-----

## Feature 4: Feel Presets

### The Problem

Getting a game mechanic to feel right means tweaking physics constants — acceleration, drag, gravity, bounce, turn rate, projectile speed. This is done by feel, one parameter at a time, playing after each change. Starting from default values means a long search through a large parameter space.

### The Feature

The engine ships with a library of named feel profiles. Each profile is a bundle of physics constants that are known to produce a specific feel:

- "Tight platformer" — high acceleration, high drag, fast direction changes
- "Floaty astronaut" — low drag, slow acceleration, momentum carries
- "Heavy tank" — high mass, slow turn rate, high max speed
- "Snappy cursor" — instant acceleration, no inertia, precise positioning
- "Underwater" — high drag, slow everything, floaty jumps

Profiles are stored as TOML files. A game can load a profile as a starting point and override individual values. Over time, every game you build can export its tuned constants back as a new profile.

### How It Helps

Instead of spending an hour finding good physics constants from scratch, you pick the closest preset and spend fifteen minutes adjusting. Your tenth game starts from a better place than your first because your personal preset library has grown.

### Extension

Presets can be swept. "Try all 12 presets on this game, run 1000 games each, show me which ones produce the longest average game length and highest decision frequency." This lets you empirically find the best starting point rather than guessing.

-----

## Feature 5: Interesting Moment Detection

### The Problem

When playtesting, you're watching for moments where something interesting happens — a close call, a surprising outcome, a decision that mattered. But you can only play one run at a time. Most of the time you're playing through boring parts waiting for something to happen.

### The Feature

The engine runs a batch of games and flags frames where "interesting" things happened. "Interesting" is defined by configurable detectors:

- **Near miss**: a tracked metric (health, score, distance to goal) crossed close to a critical threshold without hitting it.
- **Reversal**: a metric that was declining sharply reversed direction. Something saved the run.
- **Rare event**: the RNG produced an outcome in the bottom or top 5% of its distribution.
- **Decision point**: the policy's choice at this frame had unusually high variance in outcomes across other runs with different choices.
- **State novelty**: the game state at this frame is unusually different from the typical state at this frame number.

The engine presents flagged moments as short replay clips — a few seconds before and after the interesting frame.

### How It Helps

Instead of playing for thirty minutes hoping to encounter something interesting, you watch a ninety-second highlight reel of curated moments from a thousand games. This directly answers the question "is the core mechanic producing interesting decisions?" without requiring you to find those decisions by hand.

### What The Designer Does With It

If the highlight reel is boring, the mechanic is boring. If the highlights are all the same type (e.g., always near-misses, never reversals), the mechanic is one-dimensional. If interesting moments are clustered in the early game and absent in the late game, the late game needs work. The highlights are a diagnostic tool for the overall health of the design.

-----

## Feature 6: Mechanic Ablation

### The Problem

A game has multiple mechanics — movement, combat, resource management, level generation, enemies. Usually you don't know which mechanic is carrying the fun until the game is fully built. If you knew earlier, you could invest more in the important mechanics and simplify or cut the unimportant ones.

### The Feature

The engine supports running a game with individual mechanics disabled or neutralized:

- Turn off combat (enemies don't deal damage) and sweep. How does engagement change?
- Turn off resource management (infinite resources) and sweep. Does the game get better or worse?
- Simplify level generation (all identical rooms) and sweep. How much does variety matter?

The Simulation trait gains an optional method:

```rust
fn ablations(&self) -> Vec<(&str, AblationConfig)> { vec![] }
```

Each ablation describes what to disable. The engine runs full sweeps with and without each mechanic and reports the delta on key metrics.

### How It Helps

The mechanic whose removal tanks engagement (measured by game length variance, decision frequency, state-space coverage) the most is the one carrying the game. The mechanic whose removal changes nothing is dead weight you should cut or simplify. This is how researchers identify active ingredients in complex systems. Applied to game design, it tells you where to invest your limited time.

### When To Use It

Mid-development, when the game has multiple systems working and you're deciding what to polish and what to cut. Also useful when a game feels bloated — ablation tells you what you can remove without losing what makes it fun.

-----

## Feature 7: Continuous Playtesting Dashboard

### The Problem

All these features produce information — sweep results, death classifications, divergence replays, interesting moments, ablation reports. Currently this information is scattered across JSONL files and CLI output. The designer has to remember to run each analysis and interpret the raw output.

### The Feature

A browser-based dashboard (served locally or from a static build) that auto-updates as you develop. It watches for changes to the game binary, re-runs a standard analysis suite (a short sweep, death classification, interesting moment detection), and presents results in a persistent view.

The dashboard shows:

- Current sweep summary (survival rate, game length distribution, score distribution)
- Death classification breakdown with example replays
- Interesting moment highlight reel (auto-updated on each build)
- Trend over time (how metrics have changed across recent builds)
- Golden test status (pass/fail)

### How It Helps

You make a change, save, and glance at the dashboard. The sweep re-ran automatically. Survival rate went up, blowout deaths went down, a new interesting moment appeared. You don't need to remember to run the analysis or interpret raw output. The information is just there, continuously, like a test suite that runs on save but for design quality instead of code correctness.

### Implementation Note

This is the most complex feature on this list and should be built last. Everything above it produces value independently through the CLI. The dashboard is the integration layer that makes them frictionless.

-----

## Implementation Priority

|Feature                        |Effort|Design Impact                                             |Depends On                                   |
|-------------------------------|------|----------------------------------------------------------|---------------------------------------------|
|1. Variant Branching           |Medium|High — directly compresses the core design loop           |Simulation trait, parameterizable games      |
|2. Death Classification        |Low   |High — transforms sweep output from numbers into diagnosis|Sweep infrastructure, metric tracking        |
|3. Divergence Replay           |Low   |Medium — essential for understanding regressions          |Replay infrastructure, state hashing         |
|4. Feel Presets                |Low   |Medium — saves time on every new game                     |Physics constants, TOML loading              |
|5. Interesting Moment Detection|Medium|High — answers the hardest design question                |Sweep infrastructure, replay, metric tracking|
|6. Mechanic Ablation           |Medium|Medium — most useful mid-development, not always needed   |Simulation trait, sweep infrastructure       |
|7. Continuous Dashboard        |High  |High — but only as integration of above features          |All of the above                             |

Build them in this order. Each one is useful on its own. The dashboard ties them together but isn't needed until the individual features are proven.
