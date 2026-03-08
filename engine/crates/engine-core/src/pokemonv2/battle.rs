// AI-INSTRUCTIONS: pokemonv2/battle.rs — Battle system. Imports data.rs ONLY.
// BattleState, BattlePhase, auto-battle loop, Gen 2 damage formula.
// Review #4: BattleType and BattleResult live in data.rs, imported here.
// Review #17: MOVE_STRUGGLE lives in data.rs, imported here.
// Import graph: battle.rs <- data.rs ONLY

use super::data::{
    Pokemon, SpeciesId, MoveId,
    BattleType, BattleResult,
    species_data, move_data, type_effectiveness,
    MOVE_STRUGGLE,
};

// ── Constants ─────────────────────────────────────────────────────────────────

const MESSAGE_TIME: f64 = 1.5;
const AUTO_FLEE_TURNS: u8 = 10;
const TUTORIAL_CATCH_TURN: u8 = 3;

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BattlePhase {
    Intro,
    PlayerTurn,
    EnemyTurn,
    Message,
    Victory,
    Defeat,
    Caught,
    Flee,
}

#[derive(Clone, Debug)]
pub struct BattleState {
    pub enemy: Pokemon,
    pub battle_type: BattleType,
    pub turn_count: u8,
    pub phase: BattlePhase,
    pub message: Option<String>,
    pub message_timer: f64,
    pub result: Option<BattleResult>,
}

impl BattleState {
    pub fn new_wild(species: SpeciesId, level: u8, battle_type: BattleType) -> Self {
        Self {
            enemy: Pokemon::new(species, level),
            battle_type,
            turn_count: 0,
            phase: BattlePhase::Intro,
            message: None,
            message_timer: 0.0,
            result: None,
        }
    }

    pub fn new_trainer(species: SpeciesId, level: u8, battle_type: BattleType) -> Self {
        Self {
            enemy: Pokemon::new(species, level),
            battle_type,
            turn_count: 0,
            phase: BattlePhase::Intro,
            message: None,
            message_timer: 0.0,
            result: None,
        }
    }
}

// ── Damage Calculation ────────────────────────────────────────────────────────

/// Gen 2 damage formula.
/// damage = (((2 * level / 5 + 2) * power * atk / def) / 50 + 2) * effectiveness * stab * random
pub fn calc_damage(
    attacker: &Pokemon,
    defender: &Pokemon,
    move_id: MoveId,
    rng_byte: u8,
) -> u16 {
    let md = move_data(move_id);
    if md.power == 0 {
        return 0;
    }

    let att_data = species_data(attacker.species);
    let def_data = species_data(defender.species);

    let (atk, def) = if md.is_special {
        (attacker.sp_attack as f64, defender.sp_defense as f64)
    } else {
        (attacker.attack as f64, defender.defense as f64)
    };

    let level = attacker.level as f64;
    let power = md.power as f64;

    let base = ((2.0 * level / 5.0 + 2.0) * power * atk / def).floor() / 50.0 + 2.0;

    // STAB bonus
    let stab = if md.move_type == att_data.type1 || md.move_type == att_data.type2 {
        1.5
    } else {
        1.0
    };

    // Type effectiveness (both enemy types)
    let eff1 = type_effectiveness(md.move_type, def_data.type1);
    let eff2 = type_effectiveness(md.move_type, def_data.type2);

    // Random factor: 85-100% (using rng_byte)
    let rand_factor = (85.0 + (rng_byte as f64 % 16.0)) / 100.0;

    let damage = (base * stab * eff1 * eff2 * rand_factor).floor() as u16;
    damage.max(1)
}

// ── Move Selection ────────────────────────────────────────────────────────────

/// Pick the best damaging move for the enemy AI.
/// Returns MOVE_STRUGGLE if no damaging move is available.
pub fn pick_damaging_move(pokemon: &Pokemon) -> MoveId {
    let best = pokemon.moves.iter().filter_map(|&m| m).filter(|&m| {
        let md = move_data(m);
        md.power > 0 && pokemon.move_pp[pokemon.moves.iter().position(|&slot| slot == Some(m)).unwrap_or(0)] > 0
    }).max_by_key(|&m| move_data(m).power);

    best.unwrap_or(MOVE_STRUGGLE)
}

// ── Battle Step ───────────────────────────────────────────────────────────────

/// Advance the auto-battle by one frame. Returns true if battle is still running.
/// dt is the delta time in seconds (typically 1/60).
pub fn step_battle(
    battle: &mut BattleState,
    player_mon: &mut Pokemon,
    dt: f64,
    rng_byte: u8,
) -> bool {
    match battle.phase {
        BattlePhase::Intro => {
            let enemy_data = species_data(battle.enemy.species);
            battle.message = Some(format!("Wild {} appeared!", enemy_data.name));
            battle.message_timer = MESSAGE_TIME;
            battle.phase = BattlePhase::Message;
        }

        BattlePhase::Message => {
            battle.message_timer -= dt;
            if battle.message_timer <= 0.0 {
                battle.message = None;
                // Determine next phase based on current state
                if battle.result.is_some() {
                    // Battle ended: go to final phase
                    if let Some(result) = battle.result {
                        battle.phase = match result {
                            BattleResult::Won => BattlePhase::Victory,
                            BattleResult::Lost => BattlePhase::Defeat,
                            BattleResult::Caught => BattlePhase::Caught,
                            BattleResult::Fled => BattlePhase::Flee,
                        };
                    }
                } else {
                    battle.phase = BattlePhase::PlayerTurn;
                }
            }
        }

        BattlePhase::PlayerTurn => {
            battle.turn_count += 1;

            // Tutorial: auto-catch on TUTORIAL_CATCH_TURN
            if battle.battle_type == BattleType::Tutorial && battle.turn_count >= TUTORIAL_CATCH_TURN {
                let enemy_data = species_data(battle.enemy.species);
                battle.message = Some(format!("Threw a POKe BALL! {} was caught!", enemy_data.name));
                battle.message_timer = MESSAGE_TIME;
                battle.result = Some(BattleResult::Caught);
                battle.phase = BattlePhase::Message;
                return true;
            }

            // Auto-flee after too many turns
            if battle.turn_count >= AUTO_FLEE_TURNS {
                battle.message = Some("Got away safely!".to_string());
                battle.message_timer = MESSAGE_TIME;
                battle.result = Some(BattleResult::Fled);
                battle.phase = BattlePhase::Message;
                return true;
            }

            // Player attacks with best move
            let player_move = pick_damaging_move(player_mon);
            let dmg = calc_damage(player_mon, &battle.enemy, player_move, rng_byte);

            let md = move_data(player_move);
            battle.enemy.hp = battle.enemy.hp.saturating_sub(dmg);

            battle.message = Some(format!("{} used {}!",
                species_data(player_mon.species).name,
                md.name));
            battle.message_timer = MESSAGE_TIME * 0.5;
            battle.phase = BattlePhase::Message;

            if battle.enemy.hp == 0 {
                battle.result = Some(BattleResult::Won);
            } else {
                // After message: go to enemy turn
                battle.phase = BattlePhase::Message;
                // Use a tiny message then chain to enemy turn
                // (simplified: set to EnemyTurn directly after message clears)
                let _ = (); // enemy turn happens after message
            }

            // If no result yet, enemy attacks next
            if battle.result.is_none() {
                // We'll chain: Message -> EnemyTurn logic below
                // For simplicity, compute enemy attack inline and set one combined message
                let enemy_move = pick_damaging_move(&battle.enemy);
                let enemy_dmg = calc_damage(&battle.enemy, player_mon, enemy_move, rng_byte.wrapping_add(77));
                let enemy_md = move_data(enemy_move);

                player_mon.hp = player_mon.hp.saturating_sub(enemy_dmg);

                if player_mon.hp == 0 {
                    battle.result = Some(BattleResult::Lost);
                    let player_name = species_data(player_mon.species).name;
                    let enemy_name = species_data(battle.enemy.species).name;
                    battle.message = Some(format!(
                        "{} used {}! {} fainted!",
                        enemy_name, enemy_md.name, player_name
                    ));
                } else {
                    let player_name = species_data(player_mon.species).name;
                    let enemy_name = species_data(battle.enemy.species).name;
                    battle.message = Some(format!(
                        "{} used {}! {}'s HP: {}/{}",
                        enemy_name, enemy_md.name, player_name,
                        player_mon.hp, player_mon.max_hp
                    ));
                }
                battle.message_timer = MESSAGE_TIME;
                battle.phase = BattlePhase::Message;
            }
        }

        BattlePhase::EnemyTurn => {
            // This phase is rarely hit directly since we inline enemy turn in PlayerTurn.
            // Kept for correctness.
            battle.phase = BattlePhase::PlayerTurn;
        }

        BattlePhase::Victory => {
            // Battle won -- signal completion
            return false;
        }

        BattlePhase::Defeat => {
            // Handle CanLose: return false but result is Lost
            // (mod.rs checks the result field to decide whiteout vs. resume)
            return false;
        }

        BattlePhase::Caught | BattlePhase::Flee => {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::data::{PIDGEY, CYNDAQUIL, HOPPIP};

    #[test]
    fn test_new_wild_battle() {
        let battle = BattleState::new_wild(PIDGEY, 2, BattleType::Wild);
        assert_eq!(battle.enemy.species, PIDGEY);
        assert_eq!(battle.enemy.level, 2);
        assert_eq!(battle.battle_type, BattleType::Wild);
        assert_eq!(battle.phase, BattlePhase::Intro);
        assert!(battle.result.is_none());
    }

    #[test]
    fn test_new_trainer_battle() {
        let battle = BattleState::new_trainer(CYNDAQUIL, 5, BattleType::CanLose);
        assert_eq!(battle.enemy.species, CYNDAQUIL);
        assert_eq!(battle.battle_type, BattleType::CanLose);
    }

    #[test]
    fn test_hoppip_uses_struggle() {
        use super::super::data::Pokemon;
        let hoppip = Pokemon::new(HOPPIP, 2); // level 2: only knows Splash (power=0)
        let chosen = pick_damaging_move(&hoppip);
        assert_eq!(chosen, MOVE_STRUGGLE, "Hoppip at level 2 should use Struggle (no damaging moves)");
    }

    #[test]
    fn test_calc_damage_nonzero() {
        use super::super::data::Pokemon;
        let attacker = Pokemon::new(CYNDAQUIL, 5);
        let defender = Pokemon::new(PIDGEY, 2);
        let dmg = calc_damage(&attacker, &defender, super::super::data::MOVE_TACKLE, 128);
        assert!(dmg > 0, "Tackle should deal > 0 damage");
    }

    #[test]
    fn test_tutorial_auto_catches() {
        use super::super::data::Pokemon;
        let mut battle = BattleState::new_wild(HOPPIP, 3, BattleType::Tutorial);
        let mut player = Pokemon::new(CYNDAQUIL, 5);

        // Step enough turns for tutorial auto-catch (TUTORIAL_CATCH_TURN=3)
        let mut still_running = true;
        for _ in 0..50 {
            if !still_running { break; }
            still_running = step_battle(&mut battle, &mut player, 1.0 / 60.0, 42);
        }

        // Eventually: result should be Caught
        // (may take many frames due to MESSAGE_TIME delays, so run more frames)
        for _ in 0..600 {
            if !still_running { break; }
            still_running = step_battle(&mut battle, &mut player, 1.0 / 60.0, 42);
        }

        assert!(!still_running, "Tutorial battle should complete");
        assert!(matches!(battle.result, Some(BattleResult::Caught)),
            "Tutorial battle should result in Caught, got {:?}", battle.result);
    }

    #[test]
    fn test_battle_wild_runs_auto() {
        use super::super::data::Pokemon;
        let mut battle = BattleState::new_wild(PIDGEY, 2, BattleType::Wild);
        let mut player = Pokemon::new(CYNDAQUIL, 5);

        // Run for many frames -- should auto-flee after AUTO_FLEE_TURNS
        let mut still_running = true;
        for _ in 0..3600 { // 60 seconds at 60fps
            if !still_running { break; }
            still_running = step_battle(&mut battle, &mut player, 1.0 / 60.0, 100);
        }

        assert!(!still_running, "Battle should eventually complete");
        assert!(battle.result.is_some(), "Battle should have a result");
    }

    // ── Sprint 3 QA: Group 9 — Rival Battle ─────────────────────────────

    #[test]
    fn test_rival_battle_completes() {
        use super::super::data::{Pokemon, TOTODILE};
        // Rival has Totodile (counter to player's Cyndaquil)
        let mut battle = BattleState::new_trainer(TOTODILE, 5, BattleType::CanLose);
        let mut player = Pokemon::new(CYNDAQUIL, 5);

        let mut still_running = true;
        for _ in 0..3600 {
            if !still_running { break; }
            still_running = step_battle(&mut battle, &mut player, 1.0 / 60.0, 50);
        }
        assert!(!still_running, "Rival battle should complete");
        assert!(battle.result.is_some());
        // CanLose: result should be Won, Lost, or Fled -- never Caught
        assert!(!matches!(battle.result, Some(BattleResult::Caught)),
            "Trainer battle should never result in Caught");
    }

    #[test]
    fn test_starter_vs_rival_damage_nonzero() {
        use super::super::data::{Pokemon, TOTODILE, CHIKORITA};
        // Cyndaquil vs Totodile
        let attacker = Pokemon::new(CYNDAQUIL, 5);
        let defender = Pokemon::new(TOTODILE, 5);
        let dmg = calc_damage(&attacker, &defender, super::super::data::MOVE_TACKLE, 128);
        assert!(dmg > 0, "Cyndaquil Tackle vs Totodile should deal > 0");

        // Totodile vs Chikorita
        let attacker2 = Pokemon::new(TOTODILE, 5);
        let defender2 = Pokemon::new(CHIKORITA, 5);
        let dmg2 = calc_damage(&attacker2, &defender2, super::super::data::MOVE_SCRATCH, 128);
        assert!(dmg2 > 0, "Totodile Scratch vs Chikorita should deal > 0");

        // Chikorita vs Cyndaquil
        let attacker3 = Pokemon::new(CHIKORITA, 5);
        let defender3 = Pokemon::new(CYNDAQUIL, 5);
        let dmg3 = calc_damage(&attacker3, &defender3, super::super::data::MOVE_TACKLE, 128);
        assert!(dmg3 > 0, "Chikorita Tackle vs Cyndaquil should deal > 0");
    }

    #[test]
    fn test_canlose_battle_result_is_lost() {
        use super::super::data::Pokemon;
        // Intentionally give player very weak Pokemon that will lose
        let mut battle = BattleState::new_trainer(CYNDAQUIL, 50, BattleType::CanLose);
        let mut player = Pokemon::new(PIDGEY, 2); // Level 2 Pidgey vs Level 50

        let mut still_running = true;
        for _ in 0..3600 {
            if !still_running { break; }
            still_running = step_battle(&mut battle, &mut player, 1.0 / 60.0, 100);
        }
        assert!(!still_running, "Battle should complete");
        // Player should lose (level 2 vs level 50)
        assert!(matches!(battle.result, Some(BattleResult::Lost)),
            "Level 2 Pidgey vs Level 50 Cyndaquil should result in Lost, got {:?}", battle.result);
    }
}
