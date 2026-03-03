/// Headless engine simulation for AI-driven testing and game analysis.
///
/// This module provides the infrastructure for running the engine without a
/// browser or display. Claude Code (and CI) can construct scenarios, inject
/// input sequences, run N frames, and inspect the results — all via `cargo test`
/// or the CLI `simulate` subcommand.
///
/// # Architecture (18 modules across 5 layers)
///
/// ```text
/// Core:          HeadlessRunner, SimResult, GameScenario, ScenarioBuilder,
///                ShotBuilder, ScheduledAction, framebuffer_hash
/// Analysis:      run_sweep, StateTimeline, FitnessEvaluator, RegressionSuite
/// Snapshot:      run_with_snapshots, Replay, compare_replays, AnomalyDetector
/// Optimization:  Experiment, HillClimber, action_gen (grid/random/tap/drag)
/// Orchestration: Strategy, TestHarness, GoldenTest
/// ```
///
/// All modules are game-agnostic: games integrate by providing function
/// pointers for setup, update, render, and action dispatch.

mod runner;
pub mod scenario;
mod shot_builder;
mod fb_hash;
mod sweep;
mod timeline;
mod fitness;
mod regression;
mod snapshot;
mod experiment;
mod hill_climb;
pub mod action_gen;
mod replay;
mod compare;
mod anomaly;
mod strategy;
mod harness;
mod golden;

pub use runner::{HeadlessRunner, SimResult, RunConfig};
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
pub use experiment::{Experiment, ExperimentResult};
pub use hill_climb::{HillClimber, ParamRange, ClimbResult, Candidate};
pub use replay::{Replay, ReplayFrame, record_replay};
pub use compare::{compare_replays, Comparison, KeyDiff};
pub use anomaly::{AnomalyDetector, Anomaly, AnomalyKind};
pub use strategy::{Strategy, StrategyResult, StepOutcome, StatePredicate};
pub use harness::{TestHarness, HarnessReport, HarnessEntry};
pub use golden::{GoldenTest, GoldenResult};

#[cfg(test)]
mod tests;
