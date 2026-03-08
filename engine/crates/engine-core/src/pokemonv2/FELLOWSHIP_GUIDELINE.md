# Fellowship of the Sprint — Process Guide

> Living document. Update with process refinements as they emerge.

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

## Process Refinements Log

### 2026-03-08: Worktree Commit Rule Added
**Problem**: Mary/Pippin/Sam's worktree changes were lost because they never committed. Branches existed but had zero new commits. Bilbo had to re-implement from scratch instead of reviewing three implementations.
**Fix**: Added mandatory `git add -A && git commit` step to all worktree agent prompts. Agents must commit before reporting completion and include their branch name in the message.
**Impact**: Bilbo can now `git diff` and cherry-pick across all three implementations.
