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

pub use runner::{HeadlessRunner, SimResult};
pub use scenario::{GameScenario, ScheduledAction, Assertion, ScenarioResult};
pub use shot_builder::ShotBuilder;
pub use fb_hash::framebuffer_hash;
pub use sweep::{run_sweep, SweepConfig, SweepResult, SweepReport};

#[cfg(test)]
mod tests;
