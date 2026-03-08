// AI-INSTRUCTIONS: Pokemon Generation 2 recreation v2 — clean rewrite implementing the Simulation trait.
// This is the second attempt at a 1:1 Pokemon Crystal clone, built from first principles using
// sprint-driven development. Reference the original pokemon/ module for patterns but prefer
// new architecture decisions guided by sprint specifications.
//
// Module swap: This module replaces pokemon::PokemonSim in lib.rs via setup_test_pokemon().
// To swap versions, change the import and constructor in lib.rs (see POKEMON VERSION SWAP comments).
//
// Architecture (to be built by sprints):
//   - State machine: TitleScreen -> NewGame -> Overworld <-> Battle <-> Menu <-> Dialogue
//   - Grid-based overworld with tile maps from pokecrystal-canonical data
//   - Turn-based battle system with Gen 2 mechanics
//   - Story progression via event flags (u64 bitfield)
//   - Scene/script system for cutscenes and NPC events

use crate::engine::Engine;
use crate::simulation::Simulation;
use crate::rendering::color::Color;

/// Top-level game state machine for Pokemon v2.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GamePhase {
    TitleScreen,
}

/// Pokemon Generation 2 simulation — v2 clean rewrite.
pub struct PokemonV2Sim {
    phase: GamePhase,
    title_timer: f64,
}

impl PokemonV2Sim {
    pub fn new() -> Self {
        Self {
            phase: GamePhase::TitleScreen,
            title_timer: 0.0,
        }
    }
}

impl Simulation for PokemonV2Sim {
    fn setup(&mut self, _engine: &mut Engine) {
        self.phase = GamePhase::TitleScreen;
        self.title_timer = 0.0;
    }

    fn step(&mut self, _engine: &mut Engine) {
        match self.phase {
            GamePhase::TitleScreen => {
                self.title_timer += 1.0 / 60.0;
            }
        }
    }

    fn render(&self, engine: &mut Engine) {
        let w = engine.framebuffer.width as i32;
        let h = engine.framebuffer.height as i32;

        // Clear to black
        for y in 0..h {
            for x in 0..w {
                engine.framebuffer.set_pixel(x, y, Color::from_rgba(0, 0, 0, 255));
            }
        }

        // Simple title text indicator — pulsing white bar
        let pulse = ((self.title_timer * 2.0).sin() * 0.5 + 0.5) as u8;
        let brightness = 100 + pulse;
        let bar_y = h / 2;
        let bar_w = w / 3;
        let bar_x = (w - bar_w) / 2;
        for x in bar_x..(bar_x + bar_w) {
            for dy in 0..4i32 {
                if bar_y + dy < h {
                    engine.framebuffer.set_pixel(x, bar_y + dy, Color::from_rgba(brightness, brightness, brightness, 255));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pokemonv2_creates() {
        let sim = PokemonV2Sim::new();
        assert_eq!(sim.phase, GamePhase::TitleScreen);
    }
}
