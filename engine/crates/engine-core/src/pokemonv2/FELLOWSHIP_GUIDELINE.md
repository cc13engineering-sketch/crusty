# Fellowship of the Sprint — Process Guide

> Living document. Update with process refinements as they emerge.

---

## Continuous Operation — Autonomy Level 10

**Sprints run continuously without human intervention.** The user is AFK. Do not pause between sprints to ask for confirmation, approval, or feedback. When one sprint completes (commit + push + tracker update), immediately pick up the next sprint from the queue and start the pipeline.

The team lead is responsible for:
- Automatically advancing through all pipeline stages
- Spawning agents as needed without waiting for human input
- Handling all decisions autonomously (architecture, review verdicts, implementation choices)
- Committing, pushing, and updating the tracker between every sprint
- Continuing until all sprints in `queued.jsonl` are exhausted

**The only reasons to stop are:**
1. A compilation error that no agent can resolve after 3 attempts
2. All sprints are complete (queued.jsonl is empty, progress.jsonl is empty)
3. The session is forcibly terminated

---

## Sprint Pipeline

Each sprint follows this pipeline in order:

### 1. Gandalf + Aragorn: Architecture Proposal
- Gandalf picks up the next sprint (from progress.jsonl, or queued.jsonl if none in progress)
- Gandalf and Aragorn collaborate to propose architecture changes
- Output: `SPRINT{N}_ARCHITECTURE.md`

### 2. Frodo: Implementation Plan
- Translates the architecture into a concrete, phase-by-phase implementation plan
- Must be detailed enough for independent engineers to code from
- Output: `SPRINT{N}_IMPLEMENTATION_PLAN.md`

### 3. Bilbo: Plan Review (up to 3 rounds)
- Can ACCEPT, ACCEPT WITH MODIFICATIONS, or VETO
- If modifications or veto → back to Gandalf/Aragorn (max 3 iterations)
- Output: `BILBO_REVIEW_S{N}.md`

### 4. Bilbo: Incorporate Modifications
- Patches review feedback into the plan
- Output: `SPRINT{N}_REVISED_PLAN.md`

### 5. Gimli: Pokemon Accuracy Review (up to 3 rounds)
- Verifies all game data against pokecrystal-master .asm files
- Can ACCEPT, ACCEPT WITH MODIFICATIONS, or VETO
- If modifications or veto → back to Bilbo (max 3 iterations)
- Output: `GIMLI_REVIEW_S{N}.md`

### 6. Mary / Pippin / Sam: Parallel Implementation
- Each works independently in a **git worktree**
- Each attempts the full sprint implementation
- **CRITICAL: Must commit work before finishing** (see Worktree Rules below)

### 7. Bilbo: Review + Final Execution
- Reviews all three implementations (via their worktree branches)
- Picks the best aspects, updates the plan
- Executes the final implementation on the main tree
- Runs `cargo check`, `cargo test`, `cargo build`

### 8. Commit, Push, Update Tracker
- `git add`, `git commit`, `git push`
- Move sprint from progress.jsonl → done.jsonl
- Move next sprint from queued.jsonl → progress.jsonl

---

## Worktree Rules (CRITICAL)

### Problem Solved
Worktree agents that don't commit their work lose all changes when the worktree is cleaned up. The branches survive but have zero new commits, making Bilbo's review impossible.

### Rules for Mary / Pippin / Sam

**Every worktree agent MUST do the following before sending their completion message:**

1. **Stage all changes**: `git add -A` (in the worktree directory)
2. **Commit with a descriptive message**:
   ```bash
   git commit -m "Sprint {N}: {agent_name} implementation - {brief summary}"
   ```
3. **Report the branch name** in their completion message so Bilbo can find it
4. **Do NOT push** — the branch stays local for Bilbo to review

### Why This Matters
- Git worktrees share the repo's branch namespace
- Committed changes survive worktree cleanup
- Bilbo can then `git diff main..worktree-agent-{hash}` to review each implementation
- Bilbo can cherry-pick or diff specific files across implementations

### Agent Prompt Template Addition
Add this to every Mary/Pippin/Sam spawn prompt:

```
WORKTREE COMMIT RULE: Before sending your completion message, you MUST:
1. Run `git add -A` in your worktree directory
2. Run `git commit -m "Sprint {N}: {your_name} implementation - {brief summary}"`
3. Include your branch name in your completion message
Do NOT push. Bilbo will review your branch.
```

---

## Tracker Files

Located at `<crusty>/sprints/tracker/`:

| File | Purpose |
|------|---------|
| `queued.jsonl` | Sprints waiting to start |
| `progress.jsonl` | Currently active sprint (at most 1) |
| `done.jsonl` | Completed sprints with commit hashes |
| `progress-listener-{N}.jsonl` | Legolas's observational notes per sprint |

### Tracker JSONL Format

**queued.jsonl**: Raw sprint data from `all_sprints_ordered.jsonl`

**progress.jsonl**:
```json
{"sprint_index": N, "goal": "...", "started_at": "YYYY-MM-DD", "fellowship_pipeline_stage": "stage_name", "next_step": "description", "completed_stages": ["stage1", "stage2"]}
```

**done.jsonl**:
```json
{"sprint_index": N, "goal": "...", "completed_at": "YYYY-MM-DD", "commit": "hash", "summary": "what was built"}
```

---

## Agent Roster

| Agent | Role | Model | When Active |
|-------|------|-------|-------------|
| Gandalf | Expert Rust engineer, deep compiler knowledge | Opus | Pipeline stage 1 |
| Aragorn | Expert system architect | Opus | Pipeline stage 1 |
| Frodo | Technical writer | Opus | Pipeline stage 2 |
| Bilbo | Senior Sprint Engineer (Rust + Pokemon) | Opus | Stages 3, 4, 7 |
| Gimli | Pokemon Super Fan, data verifier | Opus | Stage 5 |
| Mary | Disciplined implementation engineer | Sonnet | Stage 6 (worktree) |
| Pippin | Creative implementation engineer | Sonnet | Stage 6 (worktree) |
| Sam | Clever implementation engineer | Sonnet | Stage 6 (worktree) |
| Legolas | Observational note-taker | Sonnet | Always (background) |

---

## Session Recovery — Resuming After Abrupt Cutoff

A new Claude Code session may start with no context about where the Fellowship left off. Here is how to recover:

### Step 1: Read the Tracker Files

```
Read: <crusty>/sprints/tracker/progress.jsonl
Read: <crusty>/sprints/tracker/done.jsonl
```

- `progress.jsonl` tells you **which sprint** is active and **which pipeline stage** it's at
- `done.jsonl` tells you what's already been completed and committed
- If `progress.jsonl` is empty and `done.jsonl` has entries, all sprints may be done — check `queued.jsonl`

### Step 2: Check the `fellowship_pipeline_stage` Field

The progress.jsonl entry has a `fellowship_pipeline_stage` field. Map it to where to resume:

| Stage Value | What Happened | What To Do Next |
|-------------|---------------|-----------------|
| `not_started` | Sprint was just moved from queue | Spawn Gandalf + Aragorn for architecture |
| `gandalf_aragorn_architecture` | Architecture in progress or done | Check if `SPRINT{N}_ARCHITECTURE.md` exists in pokemonv2/. If yes → spawn Frodo. If no → spawn Gandalf+Aragorn |
| `frodo_implementation_plan` | Plan in progress or done | Check if `SPRINT{N}_IMPLEMENTATION_PLAN.md` exists. If yes → spawn Bilbo for review. If no → spawn Frodo |
| `bilbo_review_round{1,2,3}` | Bilbo reviewing | Check if `BILBO_REVIEW_S{N}.md` exists. If yes → check verdict. ACCEPT → next stage. MODIFICATIONS → spawn Bilbo to incorporate. VETO → back to Gandalf |
| `bilbo_incorporates_modifications` | Bilbo patching plan | Check if `SPRINT{N}_REVISED_PLAN.md` exists. If yes → spawn Gimli. If no → spawn Bilbo |
| `gimli_review_round{1,2,3}` | Gimli reviewing | Check if `GIMLI_REVIEW_S{N}.md` exists. If yes → check verdict. ACCEPT → spawn Mary/Pippin/Sam. MODIFICATIONS → send fixes to Bilbo |
| `parallel_implementation` | Mary/Pippin/Sam coding | Check worktree branches for commits. If all three have commits → spawn Bilbo for review. If some are missing → re-spawn missing agents |
| `bilbo_final_execution` | Bilbo executing final code | Run `cargo check` — if it passes, run `cargo test`. If all pass → commit and push. If errors → spawn Bilbo to fix |
| `commit_push` | Committing | Check `git status` — if clean, the commit was made. Check `git log` for the sprint commit. If present → update tracker and move to next sprint |

### Step 3: Check for Artifact Files

The presence or absence of these files in `pokemonv2/` tells you exactly what's been done:

```
SPRINT{N}_ARCHITECTURE.md      → Architecture proposal exists
SPRINT{N}_IMPLEMENTATION_PLAN.md → Frodo's plan exists
BILBO_REVIEW_S{N}.md           → Bilbo reviewed (read verdict inside)
SPRINT{N}_REVISED_PLAN.md      → Bilbo incorporated modifications
GIMLI_REVIEW_S{N}.md           → Gimli reviewed (read verdict inside)
```

### Step 4: Check Git State

```bash
git log --oneline -5        # See if the sprint was already committed
git status                  # See if there are uncommitted changes (mid-implementation)
git branch | grep worktree  # See if Mary/Pippin/Sam branches exist with commits
cargo check                 # See if current code compiles
```

### Step 5: Update the Tracker

Once you've determined the actual state, update `progress.jsonl` with the correct `fellowship_pipeline_stage` and `completed_stages` before spawning any agents.

### Quick Recovery Checklist

1. Read `progress.jsonl` → get sprint index and stage
2. `ls pokemonv2/SPRINT*` → see which artifacts exist
3. `git log --oneline -3` → see if sprint was committed
4. `cargo check` → see if code compiles
5. Update tracker → set correct stage
6. Spawn the right agents → resume pipeline

### Common Recovery Scenarios

**Session cut during Mary/Pippin/Sam implementation:**
- Worktree changes may be lost (if agents didn't commit)
- Check `git branch | grep worktree` for branches with commits
- If no commits found, re-spawn all three with the WORKTREE COMMIT RULE
- If some committed, only re-spawn the ones that didn't

**Session cut during Bilbo final execution:**
- Run `cargo check` — if errors, the implementation is partial
- Read the current pokemonv2/*.rs files + the revised plan
- Spawn Bilbo to finish the implementation

**Session cut after commit but before push:**
- Run `git log --oneline -1` to confirm the commit exists
- Run `git push` to push it
- Update tracker files (done.jsonl, progress.jsonl, queued.jsonl)

**Session cut after push but before tracker update:**
- The code is safe in git. Just update the tracker files:
  - Move sprint entry from progress.jsonl → done.jsonl (add commit hash)
  - Move next sprint from queued.jsonl → progress.jsonl

---

## Process Refinements Log

### 2026-03-08: Continuous Operation Rule Added
**Problem**: Team lead was pausing between sprints to ask for user confirmation, wasting time when user is AFK.
**Fix**: Added "Continuous Operation — Autonomy Level 10" section. Sprints run back-to-back without human input. Team lead auto-advances the pipeline, spawns agents, commits/pushes, and picks up the next sprint immediately.
**Impact**: Full AFK operation. User can walk away and sprints keep running.

### 2026-03-08: Worktree Commit Rule Added
**Problem**: Mary/Pippin/Sam's worktree changes were lost because they never committed. Branches existed but had zero new commits. Bilbo had to re-implement from scratch instead of reviewing three implementations.
**Fix**: Added mandatory `git add -A && git commit` step to all worktree agent prompts. Agents must commit before reporting completion and include their branch name in the message.
**Impact**: Bilbo can now `git diff` and cherry-pick across all three implementations.
