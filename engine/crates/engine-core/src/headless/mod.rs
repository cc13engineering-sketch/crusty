/// Headless engine simulation for AI-driven testing and game analysis.
///
/// This module provides the infrastructure for running the engine without a
/// browser or display. Claude Code (and CI) can construct scenarios, inject
/// input sequences, run N frames, and inspect the results — all via `cargo test`
/// or the CLI `simulate` subcommand.
///
/// # Architecture
///
/// ```text
///  ShotBuilder ─┐
///               ├─► GameScenario ─► HeadlessRunner ─► SimResult
///  ScheduledAction ┘                     │
///                                  framebuffer_hash()
/// ```

mod runner;
pub mod scenario;
mod shot_builder;
mod fb_hash;
mod sweep;
mod timeline;
mod fitness;
mod regression;
mod snapshot;

pub use runner::{HeadlessRunner, SimResult};
pub use scenario::{GameScenario, ScheduledAction, Assertion, ScenarioResult, ScenarioBuilder, dispatch_noop};
pub use shot_builder::ShotBuilder;
pub use fb_hash::framebuffer_hash;
pub use sweep::{run_sweep, SweepConfig, SweepResult, SweepReport};
pub use timeline::{record_timeline, record_timeline_with_actions, StateTimeline};
pub use fitness::{
    FitnessEvaluator, FitnessResult, CriterionResult,
    score_distance, score_state_match, score_ratio,
};
pub use regression::{
    RegressionSuite, RegressionBaseline, DiffReport, DiffEntry, DiffStatus,
    classify_any_change, classify_lower_is_better,
};
pub use snapshot::{run_with_snapshots, FrameSnapshot, SnapshotResult};

#[cfg(test)]
mod tests;
