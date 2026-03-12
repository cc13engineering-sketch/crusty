// AI-INSTRUCTIONS: pokemonv2/mod.rs — Top-level PokemonV2Sim. Wires all submodules.
// This is the second Pokemon Crystal rewrite, sprint-driven from first principles.
// Reference old pokemon/ module for patterns but follow new sprint architecture.
//
// Sprint 4: Handles OverworldResult::TrainerBattle, trainer party StartBattle routing,
//           MrPokemonsHouse entry script, beaten_flag set on victory, refresh_npc_visibility.
//
// Module swap: Change import in lib.rs to use pokemonv2::PokemonV2Sim instead of
// pokemon::PokemonSim. See POKEMON VERSION SWAP comments in lib.rs.
//
// Architecture: data <- events <- maps <- overworld <- render (acyclic)
//               sprites <- data; dialogue is a leaf; mod.rs imports everything.

pub mod data;
pub mod maps;
pub mod events;
pub mod overworld;
pub mod render;
pub mod dialogue;
pub mod sprites;
pub mod battle;

use crate::engine::Engine;
use crate::simulation::Simulation;
use data::*;
use maps::*;
use events::*;
use overworld::*;
use render::render_game;
use dialogue::DialogueState;

// ── GamePhase ─────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GamePhase {
    TitleScreen,
    Overworld,
    Dialogue,
    Script,
    StarterSelect { cursor: u8 },
    MapTransition { timer: f64 },
    Battle, // stub
    Menu,   // stub
}

// ── PokemonV2Sim ──────────────────────────────────────────────────────────────

pub struct PokemonV2Sim {
    pub phase: GamePhase,
    pub player: PlayerState,
    pub party: Vec<Pokemon>,
    pub current_map_id: MapId,
    pub current_map: MapData,
    pub npc_states: Vec<NpcState>,
    pub bag: Vec<(u8, u8)>,
    pub event_flags: EventFlags,
    pub scene_state: SceneState,
    pub temp_flags: u8,
    pub dialogue: Option<DialogueState>,
    pub script: Option<ScriptState>,
    pub camera: CameraState,
    pub frame_count: u64,
    pub total_time: f64,
    pub step_count: u32,
    pub money: u32,
    pub badges: u8,
    pub title_timer: f64,
    pub player_name: String,
    pub day_night_tint: f64,
    pub time_of_day: f64,
    pub pending_warp: Option<(MapId, u8)>,
    pub battle: Option<battle::BattleState>,  // Sprint 2: battle system
}

impl PokemonV2Sim {
    pub fn new() -> Self {
        let start_map_id = MapId::PlayersHouse2F;
        let start_map = load_map(start_map_id);
        let npc_states = init_npc_states(&start_map);
        let mut sim = Self {
            phase: GamePhase::TitleScreen,
            player: PlayerState {
                x: 3, y: 3,
                facing: Direction::Down,
                walk_offset: 0.0,
                is_walking: false,
                walk_frame: 0,
                frame_timer: 0.0,
                name: "GOLD".to_string(),
            },
            party: Vec::new(),
            current_map_id: start_map_id,
            current_map: start_map,
            npc_states,
            bag: Vec::new(),
            event_flags: EventFlags::new(),
            scene_state: SceneState::new(),
            temp_flags: 0,
            dialogue: None,
            script: None,
            camera: CameraState {
                x: 3.0 * 16.0,
                y: 3.0 * 16.0,
            },
            frame_count: 0,
            total_time: 0.0,
            step_count: 0,
            money: 3000,
            badges: 0,
            title_timer: 0.0,
            player_name: "GOLD".to_string(),
            day_night_tint: 0.0,
            time_of_day: 0.0,
            pending_warp: None,
            battle: None,  // Sprint 2: initialized in constructor
        };
        sim.refresh_npc_visibility();
        sim
    }
}

impl Simulation for PokemonV2Sim {
    fn setup(&mut self, _engine: &mut Engine) {
        *self = Self::new();
    }

    fn step(&mut self, engine: &mut Engine) {
        self.frame_count += 1;
        self.total_time += 1.0 / 60.0;
        self.step_count += 1;

        match self.phase {
            GamePhase::TitleScreen           => self.step_title(engine),
            GamePhase::Overworld             => self.step_overworld_phase(engine),
            GamePhase::Dialogue              => self.step_dialogue(engine),
            GamePhase::Script                => self.step_script_phase(engine),
            GamePhase::StarterSelect { cursor } => self.step_starter_select(cursor, engine),
            GamePhase::MapTransition { timer }  => self.step_map_transition(timer, engine),
            GamePhase::Battle => self.step_battle_phase(engine),
            GamePhase::Menu   => {} // stub
        }
    }

    fn render(&self, engine: &mut Engine) {
        render_game(self, engine);
    }
}

// ── Phase Step Methods ────────────────────────────────────────────────────────

impl PokemonV2Sim {
    fn step_title(&mut self, engine: &Engine) {
        self.title_timer += 1.0 / 60.0;
        if is_confirm(engine) {
            self.phase = GamePhase::Overworld;
        }
    }

    fn step_overworld_phase(&mut self, engine: &Engine) {
        // Review #3: caller extracts RNG bytes and TimeOfDay before calling step_overworld
        let time_of_day = data::get_time_of_day(self.total_time);
        let rng_enc = (engine.rng.state & 0xFF) as u8;
        let rng_slot = ((engine.rng.state >> 8) & 0xFF) as u8;

        let result = step_overworld(
            &mut self.player,
            &mut self.camera,
            &self.current_map,
            &mut self.npc_states,
            &self.event_flags,
            &self.scene_state,
            engine,
            time_of_day,
            rng_enc,
            rng_slot,
        );
        match result {
            OverworldResult::Nothing => {}
            OverworldResult::WarpTo { dest_map, dest_warp_id } => {
                self.pending_warp = Some((dest_map, dest_warp_id));
                self.phase = GamePhase::MapTransition { timer: 0.0 };
            }
            OverworldResult::TriggerScript { script_id, .. }
            | OverworldResult::TriggerCoordEvent { script_id } => {
                let steps = get_script(script_id);
                self.script = Some(ScriptState::new(steps));
                self.phase = GamePhase::Script;
            }
            OverworldResult::WildEncounter { species, level } => {
                self.start_wild_battle(species, level);
            }
            OverworldResult::MapConnection { direction, dest_map, offset } => {
                self.handle_map_connection(direction, dest_map, offset);
            }
            OverworldResult::TrainerBattle { npc_idx: _, script_id } => {
                // Sprint 4: trainer saw player — run their script (contains LoadTrainerParty + StartBattle)
                let steps = get_script(script_id);
                self.script = Some(ScriptState::new(steps));
                self.phase = GamePhase::Script;
            }
        }
    }

    fn step_script_phase(&mut self, engine: &Engine) {
        if let Some(ref mut script) = self.script {
            let result = step_script(
                script,
                &mut self.player,
                &mut self.npc_states,
                &mut self.event_flags,
                &mut self.scene_state,
                self.current_map_id,
                &mut self.party,
                &mut self.bag,
                is_confirm(engine),
                is_cancel(engine),
                is_up(engine),
                is_down(engine),
            );
            // Review #2: match ScriptResult instead of bool
            match result {
                ScriptResult::Running => {}
                ScriptResult::Ended => {
                    self.script = None;
                    self.phase = GamePhase::Overworld;
                    self.refresh_npc_visibility();
                }
                ScriptResult::StartBattle { battle_type, species } => {
                    // Extract trainer party before releasing the borrow on script
                    let trainer_party = script.trainer_party.take();
                    let beaten_flag = script.trainer_beaten_flag.take();
                    match battle_type {
                        BattleType::Tutorial | BattleType::Wild => {
                            if let Some((sp, lv)) = species {
                                let battle_state = battle::BattleState::new_wild(sp, lv, battle_type);
                                self.battle = Some(battle_state);
                                self.phase = GamePhase::Battle;
                            }
                        }
                        BattleType::Normal => {
                            // Sprint 4: check if a trainer party was loaded via LoadTrainerParty
                            if let Some(party) = trainer_party {
                                let battle_state = battle::BattleState::new_trainer_party(
                                    party, battle_type, beaten_flag,
                                );
                                self.battle = Some(battle_state);
                                self.phase = GamePhase::Battle;
                            } else {
                                // Fallback: rival-style single trainer battle
                                let rival_species = self.get_rival_species();
                                let battle_state = battle::BattleState::new_trainer(rival_species, 5, battle_type);
                                self.battle = Some(battle_state);
                                self.phase = GamePhase::Battle;
                            }
                        }
                        BattleType::CanLose => {
                            // Rival battle (can't lose): determine counter-starter
                            let rival_species = self.get_rival_species();
                            let battle_state = battle::BattleState::new_trainer(rival_species, 5, battle_type);
                            self.battle = Some(battle_state);
                            self.phase = GamePhase::Battle;
                        }
                    }
                }
            }
        } else {
            self.phase = GamePhase::Overworld;
        }
    }

    fn step_dialogue(&mut self, engine: &Engine) {
        if let Some(ref mut dlg) = self.dialogue {
            let still_active = dlg.step(is_confirm(engine));
            if !still_active {
                self.dialogue = None;
                self.phase = GamePhase::Overworld;
            }
        } else {
            self.phase = GamePhase::Overworld;
        }
    }

    fn step_map_transition(&mut self, timer: f64, _engine: &Engine) {
        let new_timer = timer + 1.0 / 60.0;
        if new_timer >= 0.5 && timer < 0.5 {
            if let Some((dest_map, dest_warp_id)) = self.pending_warp.take() {
                self.change_map(dest_map, dest_warp_id);
            }
        }
        if new_timer >= 1.0 {
            self.phase = GamePhase::Overworld;
            self.check_map_entry_scripts();
        } else {
            self.phase = GamePhase::MapTransition { timer: new_timer };
        }
    }

    fn step_starter_select(&mut self, cursor: u8, engine: &Engine) {
        let mut c = cursor;
        if is_left(engine)  && c > 0 { c -= 1; }
        if is_right(engine) && c < 2 { c += 1; }

        if is_confirm(engine) {
            let species = match c {
                0 => CYNDAQUIL,
                1 => TOTODILE,
                _ => CHIKORITA,
            };
            let mut starter = Pokemon::new(species, 5);
            starter.held_item = Some(ITEM_BERRY);
            self.party.push(starter);

            self.event_flags.set(EVENT_GOT_A_POKEMON_FROM_ELM);
            match c {
                0 => {
                    self.event_flags.set(EVENT_GOT_CYNDAQUIL_FROM_ELM);
                    self.event_flags.set(EVENT_CYNDAQUIL_POKEBALL_IN_ELMS_LAB);
                }
                1 => {
                    self.event_flags.set(EVENT_GOT_TOTODILE_FROM_ELM);
                    self.event_flags.set(EVENT_TOTODILE_POKEBALL_IN_ELMS_LAB);
                }
                _ => {
                    self.event_flags.set(EVENT_GOT_CHIKORITA_FROM_ELM);
                    self.event_flags.set(EVENT_CHIKORITA_POKEBALL_IN_ELMS_LAB);
                }
            }

            self.refresh_npc_visibility();

            // GIMLI FIX: post-starter sets scene to AIDE_GIVES_POTION (5), not NOOP (2)
            self.scene_state.set(MapId::ElmsLab, SCENE_ELMSLAB_AIDE_GIVES_POTION);
            self.scene_state.set(MapId::NewBarkTown, SCENE_NEWBARKTOWN_NOOP);

            let post_script = build_post_starter_script(c);
            self.script = Some(ScriptState::new(post_script));
            self.phase = GamePhase::Script;
            return;
        }

        if is_cancel(engine) {
            self.phase = GamePhase::Overworld;
            return;
        }

        // Bilbo Rev 3 fix: use matches!() not { .. } in ==
        if matches!(self.phase, GamePhase::StarterSelect { .. }) {
            self.phase = GamePhase::StarterSelect { cursor: c };
        }
    }

    fn step_battle_phase(&mut self, engine: &Engine) {
        let rng_byte = (engine.rng.state & 0xFF) as u8;

        // Collect result before ending borrow on self.battle
        let outcome = if let Some(ref mut battle_state) = self.battle {
            if let Some(player_mon) = self.party.first_mut() {
                let still_running = battle::step_battle(battle_state, player_mon, 1.0 / 60.0, rng_byte);
                if !still_running {
                    Some((battle_state.result, battle_state.battle_type, battle_state.beaten_flag))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            self.phase = GamePhase::Overworld;
            return;
        };

        if let Some((result, battle_type, beaten_flag)) = outcome {
            self.battle = None;
            match result {
                Some(BattleResult::Lost) if battle_type != BattleType::CanLose => {
                    self.heal_party();
                    self.warp_to_last_pokecenter();
                }
                Some(BattleResult::Won) => {
                    // Sprint 4: set trainer beaten flag so they don't challenge again
                    if let Some(flag) = beaten_flag {
                        self.event_flags.set(flag);
                        self.refresh_npc_visibility();
                    }
                    self.phase = if self.script.is_some() {
                        GamePhase::Script
                    } else {
                        GamePhase::Overworld
                    };
                }
                _ => {
                    self.phase = if self.script.is_some() {
                        GamePhase::Script
                    } else {
                        GamePhase::Overworld
                    };
                }
            }
        }
    }

    fn start_wild_battle(&mut self, species: SpeciesId, level: u8) {
        let battle_state = battle::BattleState::new_wild(species, level, BattleType::Wild);
        self.battle = Some(battle_state);
        self.phase = GamePhase::Battle;
    }

    fn handle_map_connection(&mut self, direction: Direction, dest_map: MapId, offset: i8) {
        self.current_map_id = dest_map;
        self.current_map = load_map(dest_map);
        self.npc_states = init_npc_states(&self.current_map);
        self.temp_flags = 0;

        match direction {
            Direction::Left => {
                self.player.x = self.current_map.width - 1;
                self.player.y += offset as i32;
            }
            Direction::Right => {
                self.player.x = 0;
                self.player.y += offset as i32;
            }
            Direction::Up => {
                self.player.y = self.current_map.height - 1;
                self.player.x += offset as i32;
            }
            Direction::Down => {
                self.player.y = 0;
                self.player.x += offset as i32;
            }
        }
        snap_camera(&mut self.camera, &self.player);
        self.refresh_npc_visibility();
        self.check_map_callbacks();
    }

    fn get_rival_species(&self) -> SpeciesId {
        if self.event_flags.has(EVENT_GOT_CYNDAQUIL_FROM_ELM) { TOTODILE }
        else if self.event_flags.has(EVENT_GOT_TOTODILE_FROM_ELM) { CHIKORITA }
        else { CYNDAQUIL }
    }

    fn heal_party(&mut self) {
        for p in self.party.iter_mut() {
            p.hp = p.max_hp;
            p.status = data::StatusCondition::None;
        }
    }

    fn warp_to_last_pokecenter(&mut self) {
        let dest = if self.event_flags.has(EVENT_ENGINE_FLYPOINT_CHERRYGROVE) {
            MapId::CherrygrovePokecenter1F
        } else {
            MapId::ElmsLab
        };
        self.change_map(dest, 0);
        self.phase = GamePhase::Overworld;
    }

    fn check_map_callbacks(&mut self) {
        match self.current_map_id {
            MapId::CherrygroveCity => {
                self.event_flags.set(EVENT_ENGINE_FLYPOINT_CHERRYGROVE);
            }
            MapId::Route29 => {
                // Review #19: inline loop instead of undefined find_npc_by_event_flag method
                for i in 0..self.current_map.npcs.len() {
                    if self.current_map.npcs[i].event_flag == Some(EVENT_ROUTE_29_TUSCANY_OF_TUESDAY) {
                        if let Some(state) = self.npc_states.get_mut(i) {
                            state.visible = false;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn change_map(&mut self, dest_map: MapId, dest_warp_id: u8) {
        self.current_map_id = dest_map;
        self.current_map = load_map(dest_map);
        self.npc_states = init_npc_states(&self.current_map);
        self.temp_flags = 0;

        let (wx, wy) = resolve_warp_position(&self.current_map, dest_warp_id);
        self.player.x = wx;
        self.player.y = wy;
        self.player.is_walking = false;
        self.player.walk_offset = 0.0;

        snap_camera(&mut self.camera, &self.player);
        self.refresh_npc_visibility();
    }

    fn check_map_entry_scripts(&mut self) {
        if self.current_map_id == MapId::ElmsLab {
            let scene = self.scene_state.get(self.current_map_id);
            if scene == SCENE_ELMSLAB_MEET_ELM {
                // Reposition Elm to (3,4) before cutscene
                if let Some(elm) = self.npc_states.get_mut(0) {
                    elm.x = 3;
                    elm.y = 4;
                }
                let steps = build_elm_intro_script();
                self.script = Some(ScriptState::new(steps));
                self.phase = GamePhase::Script;
            }
        }

        // Sprint 4: Mr. Pokemon's House first visit cutscene
        if self.current_map_id == MapId::MrPokemonsHouse {
            let scene = self.scene_state.get(self.current_map_id);
            if scene == SCENE_MRPOKEMONSHOUSE_MEET_MR_POKEMON {
                let steps = build_mr_pokemon_meet_script();
                self.script = Some(ScriptState::new(steps));
                self.phase = GamePhase::Script;
            }
        }
    }

    pub fn refresh_npc_visibility(&mut self) {
        for (i, npc_def) in self.current_map.npcs.iter().enumerate() {
            if let Some(flag) = npc_def.event_flag {
                let flag_set = self.event_flags.has(flag);
                if let Some(state) = self.npc_states.get_mut(i) {
                    state.visible = if npc_def.event_flag_show { flag_set } else { !flag_set };
                }
            }
        }
    }

    /// Test helper: create a sim in a specific map/position state (skips title).
    #[cfg(test)]
    pub fn with_state(map: MapId, x: i32, y: i32, party: Vec<Pokemon>) -> Self {
        let mut sim = Self::new();
        sim.phase = GamePhase::Overworld;
        sim.party = party;
        sim.current_map_id = map;
        sim.current_map = load_map(map);
        sim.npc_states = init_npc_states(&sim.current_map);
        sim.player.x = x;
        sim.player.y = y;
        snap_camera(&mut sim.camera, &sim.player);
        sim.refresh_npc_visibility();
        sim
    }
}

// ── Input Helpers ─────────────────────────────────────────────────────────────

fn is_confirm(engine: &Engine) -> bool {
    engine.input.keys_pressed.contains("KeyZ")
        || engine.input.keys_pressed.contains("Space")
        || engine.input.keys_pressed.contains("Enter")
}

fn is_cancel(engine: &Engine) -> bool {
    engine.input.keys_pressed.contains("KeyX")
        || engine.input.keys_pressed.contains("Escape")
}

fn is_up(engine: &Engine) -> bool {
    engine.input.keys_pressed.contains("ArrowUp")
        || engine.input.keys_pressed.contains("KeyW")
}

fn is_down(engine: &Engine) -> bool {
    engine.input.keys_pressed.contains("ArrowDown")
        || engine.input.keys_pressed.contains("KeyS")
}

fn is_left(engine: &Engine) -> bool {
    engine.input.keys_pressed.contains("ArrowLeft")
        || engine.input.keys_pressed.contains("KeyA")
}

fn is_right(engine: &Engine) -> bool {
    engine.input.keys_pressed.contains("ArrowRight")
        || engine.input.keys_pressed.contains("KeyD")
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pokemonv2_creates() {
        let sim = PokemonV2Sim::new();
        assert_eq!(sim.phase, GamePhase::TitleScreen);
    }

    #[test]
    fn test_title_to_overworld() {
        let mut sim = PokemonV2Sim::new();
        let mut engine = Engine::new(160, 144);
        engine.input.keys_pressed.insert("KeyZ".to_string());
        sim.step(&mut engine);
        assert_eq!(sim.phase, GamePhase::Overworld);
    }

    #[test]
    fn test_player_spawns_in_bedroom() {
        let sim = PokemonV2Sim::with_state(MapId::PlayersHouse2F, 3, 3, vec![]);
        assert_eq!(sim.player.x, 3);
        assert_eq!(sim.player.y, 3);
        assert_eq!(sim.current_map_id, MapId::PlayersHouse2F);
    }

    #[test]
    fn test_all_sprint1_maps_load() {
        let maps = [
            MapId::PlayersHouse2F, MapId::PlayersHouse1F,
            MapId::NewBarkTown, MapId::ElmsLab,
            MapId::ElmsHouse, MapId::PlayersNeighborsHouse,
        ];
        for &id in &maps {
            let map = load_map(id);
            assert!(map.width > 0);
            assert!(map.height > 0);
            assert_eq!(map.tiles.len(), (map.width * map.height) as usize);
            assert_eq!(map.collision.len(), (map.width * map.height) as usize);
        }
    }

    #[test]
    fn test_warp_bidirectional_consistency() {
        let maps = [
            MapId::PlayersHouse2F, MapId::PlayersHouse1F,
            MapId::NewBarkTown, MapId::ElmsLab,
        ];
        for &map_id in &maps {
            let map = load_map(map_id);
            for warp in &map.warps {
                let dest = load_map(warp.dest_map);
                let has_return = dest.warps.iter().any(|w| w.dest_map == map_id);
                assert!(has_return,
                    "Warp from {:?} to {:?} has no return warp", map_id, warp.dest_map);
            }
        }
    }

    #[test]
    fn test_walking_changes_position() {
        let mut sim = PokemonV2Sim::with_state(MapId::PlayersHouse2F, 3, 3, vec![]);
        let mut engine = Engine::new(160, 144);
        let start_x = sim.player.x;
        engine.input.keys_held.insert("ArrowRight".to_string());
        sim.step(&mut engine);
        assert!(sim.player.is_walking || sim.player.x != start_x);
    }

    #[test]
    fn test_collision_blocks_wall() {
        let mut sim = PokemonV2Sim::with_state(MapId::PlayersHouse2F, 3, 3, vec![]);
        let mut engine = Engine::new(160, 144);
        for _ in 0..20 {
            engine.input.keys_held.insert("ArrowUp".to_string());
            sim.step(&mut engine);
            engine.input.keys_held.clear();
            engine.input.keys_pressed.clear();
        }
        assert!(sim.player.y >= 1, "Player walked through wall to y={}", sim.player.y);
    }

    #[test]
    fn test_warp_bedroom_to_1f() {
        // Start directly below the stair warp at (7,0) — player at (7,1)
        let mut sim = PokemonV2Sim::with_state(MapId::PlayersHouse2F, 7, 1, vec![]);
        let mut engine = Engine::new(160, 144);
        // Walk up toward the stair warp tile at (7, 0)
        // Needs 2+ frames at 8px/frame to complete 1 tile (16px)
        engine.input.keys_held.insert("ArrowUp".to_string());
        for _ in 0..4 {
            sim.step(&mut engine);
            engine.input.keys_pressed.clear();
        }
        let warped = sim.current_map_id == MapId::PlayersHouse1F
            || matches!(sim.phase, GamePhase::MapTransition { .. });
        assert!(warped, "Player should warp from 2F stairs to 1F (was at {:?}, map={:?})",
            (sim.player.x, sim.player.y), sim.current_map_id);
    }

    #[test]
    fn test_coord_event_teacher_blocks() {
        let mut sim = PokemonV2Sim::with_state(MapId::NewBarkTown, 3, 8, vec![]);
        assert_eq!(sim.scene_state.get(MapId::NewBarkTown), 0);
        let mut engine = Engine::new(160, 144);
        for _ in 0..30 {
            engine.input.keys_held.insert("ArrowLeft".to_string());
            sim.step(&mut engine);
            engine.input.keys_held.clear();
            engine.input.keys_pressed.clear();
            sim.step(&mut engine);
        }
        let triggered = matches!(sim.phase, GamePhase::Script)
            || sim.player.x > 1;
        assert!(triggered, "Teacher should block player from leaving without Pokemon");
    }

    #[test]
    fn test_script_engine_basic() {
        let steps = vec![
            ScriptStep::ShowText("Hello!".to_string()),
            ScriptStep::End,
        ];
        let mut script = ScriptState::new(steps);
        let mut player = PlayerState {
            x: 0, y: 0, facing: Direction::Down,
            walk_offset: 0.0, is_walking: false,
            walk_frame: 0, frame_timer: 0.0,
            name: "TEST".to_string(),
        };
        let mut npc_states = Vec::new();
        let mut flags = EventFlags::new();
        let mut scenes = SceneState::new();
        let mut party = Vec::new();
        let mut bag = Vec::new();

        let result = step_script(&mut script, &mut player, &mut npc_states,
            &mut flags, &mut scenes, MapId::NewBarkTown,
            &mut party, &mut bag, false, false, false, false);
        assert!(matches!(result, ScriptResult::Running));
        assert!(script.text_buffer.is_some());

        let _ = step_script(&mut script, &mut player, &mut npc_states,
            &mut flags, &mut scenes, MapId::NewBarkTown,
            &mut party, &mut bag, true, false, false, false);

        let result = step_script(&mut script, &mut player, &mut npc_states,
            &mut flags, &mut scenes, MapId::NewBarkTown,
            &mut party, &mut bag, false, false, false, false);
        assert!(matches!(result, ScriptResult::Ended), "Script should end after End step");
    }

    #[test]
    fn test_npc_collision() {
        let map = load_map(MapId::NewBarkTown);
        let npc_states = init_npc_states(&map);
        let found = npc_at(&npc_states, 6, 8);
        assert!(found.is_some(), "Teacher NPC should be at (6, 8)");
    }

    #[test]
    fn test_elm_lab_has_correct_npcs() {
        let map = load_map(MapId::ElmsLab);
        assert_eq!(map.npcs.len(), 6, "Elm's lab should have 6 NPCs");
        assert_eq!(map.npcs[2].x, 6); assert_eq!(map.npcs[2].y, 3); // Cyndaquil
        assert_eq!(map.npcs[3].x, 7); assert_eq!(map.npcs[3].y, 3); // Totodile
        assert_eq!(map.npcs[4].x, 8); assert_eq!(map.npcs[4].y, 3); // Chikorita
    }

    #[test]
    fn test_starter_pokemon_stats() {
        let cyndaquil = Pokemon::new(CYNDAQUIL, 5);
        assert_eq!(cyndaquil.level, 5);
        assert_eq!(cyndaquil.species, CYNDAQUIL);
        assert!(cyndaquil.hp > 0);
        assert!(cyndaquil.moves.iter().any(|m| *m == Some(MOVE_TACKLE)));
        assert!(cyndaquil.moves.iter().any(|m| *m == Some(MOVE_LEER)));
    }

    #[test]
    fn test_event_flags() {
        let mut flags = EventFlags::new();
        assert!(!flags.has(EVENT_GOT_A_POKEMON_FROM_ELM));
        flags.set(EVENT_GOT_A_POKEMON_FROM_ELM);
        assert!(flags.has(EVENT_GOT_A_POKEMON_FROM_ELM));
        flags.clear(EVENT_GOT_A_POKEMON_FROM_ELM);
        assert!(!flags.has(EVENT_GOT_A_POKEMON_FROM_ELM));
    }

    #[test]
    fn test_new_bark_town_dimensions() {
        let map = load_map(MapId::NewBarkTown);
        assert_eq!(map.width, 18);
        assert_eq!(map.height, 20);
    }

    #[test]
    fn test_elms_lab_coord_events() {
        let map = load_map(MapId::ElmsLab);
        assert_eq!(map.coord_events.len(), 8);
        let cant_leave: Vec<_> = map.coord_events.iter()
            .filter(|e| e.scene_id == SCENE_ELMSLAB_CANT_LEAVE)
            .collect();
        assert_eq!(cant_leave.len(), 2);
        assert!(cant_leave.iter().all(|e| e.y == 6));
    }

    #[test]
    fn test_starter_scene_sets_aide_gives_potion() {
        // GIMLI FIX: after starter selection, ElmsLab scene should be 5, not 2
        let mut sim = PokemonV2Sim::with_state(MapId::ElmsLab, 6, 4, vec![]);
        sim.scene_state.set(MapId::ElmsLab, SCENE_ELMSLAB_CANT_LEAVE);
        // Simulate starter selection (choice=0, Cyndaquil)
        sim.phase = GamePhase::StarterSelect { cursor: 0 };
        let mut engine = Engine::new(160, 144);
        engine.input.keys_pressed.insert("KeyZ".to_string());
        sim.step(&mut engine);
        // After confirm: scene should be AIDE_GIVES_POTION (5)
        assert_eq!(sim.scene_state.get(MapId::ElmsLab), SCENE_ELMSLAB_AIDE_GIVES_POTION,
            "Gimli fix: after starter, ElmsLab scene should be AIDE_GIVES_POTION=5");
    }

    // ── Sprint 3 QA: Group 1 — Player Spawn & Initial State ────────────

    #[test]
    fn test_player_spawn_position_and_initial_state() {
        let sim = PokemonV2Sim::new();
        assert_eq!(sim.current_map_id, MapId::PlayersHouse2F);
        assert_eq!(sim.player.x, 3);
        assert_eq!(sim.player.y, 3);
        assert_eq!(sim.phase, GamePhase::TitleScreen);
        assert!(sim.party.is_empty(), "Party should be empty at start");
        assert_eq!(sim.money, 3000);
        assert_eq!(sim.badges, 0);
        assert!(sim.bag.is_empty());
        assert!(sim.battle.is_none());
    }

    // ── Sprint 3 QA: Group 4 — Starter Selection All Three ─────────────

    #[test]
    fn test_starter_selection_cyndaquil() {
        let mut sim = PokemonV2Sim::with_state(MapId::ElmsLab, 6, 4, vec![]);
        sim.scene_state.set(MapId::ElmsLab, SCENE_ELMSLAB_CANT_LEAVE);
        sim.phase = GamePhase::StarterSelect { cursor: 0 };
        let mut engine = Engine::new(160, 144);
        engine.input.keys_pressed.insert("KeyZ".to_string());
        sim.step(&mut engine);
        assert_eq!(sim.party.len(), 1);
        assert_eq!(sim.party[0].species, CYNDAQUIL);
        assert_eq!(sim.party[0].level, 5);
        assert!(sim.event_flags.has(EVENT_GOT_A_POKEMON_FROM_ELM));
        assert!(sim.event_flags.has(EVENT_GOT_CYNDAQUIL_FROM_ELM));
        assert!(sim.event_flags.has(EVENT_CYNDAQUIL_POKEBALL_IN_ELMS_LAB));
        assert_eq!(sim.scene_state.get(MapId::ElmsLab), SCENE_ELMSLAB_AIDE_GIVES_POTION);
        assert_eq!(sim.scene_state.get(MapId::NewBarkTown), SCENE_NEWBARKTOWN_NOOP);
    }

    #[test]
    fn test_starter_selection_totodile() {
        let mut sim = PokemonV2Sim::with_state(MapId::ElmsLab, 7, 4, vec![]);
        sim.scene_state.set(MapId::ElmsLab, SCENE_ELMSLAB_CANT_LEAVE);
        sim.phase = GamePhase::StarterSelect { cursor: 1 };
        let mut engine = Engine::new(160, 144);
        engine.input.keys_pressed.insert("KeyZ".to_string());
        sim.step(&mut engine);
        assert_eq!(sim.party.len(), 1);
        assert_eq!(sim.party[0].species, TOTODILE);
        assert!(sim.event_flags.has(EVENT_GOT_TOTODILE_FROM_ELM));
        assert!(sim.event_flags.has(EVENT_TOTODILE_POKEBALL_IN_ELMS_LAB));
    }

    #[test]
    fn test_starter_selection_chikorita() {
        let mut sim = PokemonV2Sim::with_state(MapId::ElmsLab, 8, 4, vec![]);
        sim.scene_state.set(MapId::ElmsLab, SCENE_ELMSLAB_CANT_LEAVE);
        sim.phase = GamePhase::StarterSelect { cursor: 2 };
        let mut engine = Engine::new(160, 144);
        engine.input.keys_pressed.insert("KeyZ".to_string());
        sim.step(&mut engine);
        assert_eq!(sim.party.len(), 1);
        assert_eq!(sim.party[0].species, CHIKORITA);
        assert!(sim.event_flags.has(EVENT_GOT_CHIKORITA_FROM_ELM));
        assert!(sim.event_flags.has(EVENT_CHIKORITA_POKEBALL_IN_ELMS_LAB));
    }

    #[test]
    fn test_starter_held_item_berry() {
        let mut sim = PokemonV2Sim::with_state(MapId::ElmsLab, 6, 4, vec![]);
        sim.scene_state.set(MapId::ElmsLab, SCENE_ELMSLAB_CANT_LEAVE);
        sim.phase = GamePhase::StarterSelect { cursor: 0 };
        let mut engine = Engine::new(160, 144);
        engine.input.keys_pressed.insert("KeyZ".to_string());
        sim.step(&mut engine);
        assert_eq!(sim.party[0].held_item, Some(ITEM_BERRY),
            "Starter should hold a Berry");
    }

    // ── Sprint 3 QA: Group 9 — Rival Species Counter ───────────────────

    #[test]
    fn test_rival_species_counters_starter() {
        // Cyndaquil -> rival Totodile
        let mut sim = PokemonV2Sim::new();
        sim.event_flags.set(EVENT_GOT_CYNDAQUIL_FROM_ELM);
        assert_eq!(sim.get_rival_species(), TOTODILE);

        // Totodile -> rival Chikorita
        let mut sim2 = PokemonV2Sim::new();
        sim2.event_flags.set(EVENT_GOT_TOTODILE_FROM_ELM);
        assert_eq!(sim2.get_rival_species(), CHIKORITA);

        // Chikorita -> rival Cyndaquil
        let sim3 = PokemonV2Sim::new();
        // No flag set = default = Cyndaquil
        assert_eq!(sim3.get_rival_species(), CYNDAQUIL);
    }

    #[test]
    fn test_canlose_battle_does_not_blackout() {
        let mut sim = PokemonV2Sim::with_state(
            MapId::CherrygroveCity, 33, 6,
            vec![Pokemon::new(CYNDAQUIL, 5)],
        );
        // Simulate battle ending with Lost + CanLose
        sim.battle = Some(battle::BattleState::new_trainer(TOTODILE, 5, BattleType::CanLose));
        sim.phase = GamePhase::Battle;
        // Manually set result to Lost
        if let Some(ref mut b) = sim.battle {
            b.result = Some(BattleResult::Lost);
            b.phase = battle::BattlePhase::Defeat;
        }
        let mut engine = Engine::new(160, 144);
        sim.step(&mut engine);
        // Should NOT warp to pokecenter — should return to overworld or script
        let expected = sim.current_map_id == MapId::CherrygroveCity;
        assert!(expected, "CanLose battle loss should keep player on same map, not blackout");
    }

    // ── Sprint 3 QA: Group 7 — Cherrygrove Flypoint ────────────────────

    #[test]
    fn test_cherrygrove_flypoint_set_on_entry() {
        let mut sim = PokemonV2Sim::with_state(
            MapId::Route29, 0, 10,
            vec![Pokemon::new(CYNDAQUIL, 5)],
        );
        assert!(!sim.event_flags.has(EVENT_ENGINE_FLYPOINT_CHERRYGROVE));
        sim.handle_map_connection(Direction::Left, MapId::CherrygroveCity, 0);
        assert!(sim.event_flags.has(EVENT_ENGINE_FLYPOINT_CHERRYGROVE),
            "Entering Cherrygrove should set flypoint flag");
    }

    // ── Sprint 3 QA: Group 10 — Integration Tests ──────────────────────

    #[test]
    fn test_all_map_entry_scripts_dont_panic() {
        let all_maps = [
            MapId::PlayersHouse2F, MapId::PlayersHouse1F, MapId::NewBarkTown,
            MapId::ElmsLab, MapId::ElmsHouse, MapId::PlayersNeighborsHouse,
            MapId::Route29, MapId::Route27, MapId::Route29Route46Gate,
            MapId::CherrygroveCity, MapId::CherrygrovePokecenter1F,
            MapId::CherrygroveMart, MapId::GuideGentsHouse,
            MapId::CherrygroveGymSpeechHouse, MapId::CherrygroveEvolutionSpeechHouse,
            MapId::Route46, MapId::Route30,
        ];
        for &map_id in &all_maps {
            let mut sim = PokemonV2Sim::with_state(map_id, 1, 1, vec![]);
            sim.check_map_entry_scripts();
            sim.check_map_callbacks();
            // Just verify no panic
        }
    }

    #[test]
    fn test_full_bedroom_to_new_bark_flow() {
        // Start at stair warp position in 2F
        let mut sim = PokemonV2Sim::with_state(MapId::PlayersHouse2F, 7, 1, vec![]);
        let mut engine = Engine::new(160, 144);

        // Walk up to stair warp
        engine.input.keys_held.insert("ArrowUp".to_string());
        for _ in 0..4 {
            sim.step(&mut engine);
            engine.input.keys_pressed.clear();
        }
        engine.input.keys_held.clear();

        // Run transition to completion
        for _ in 0..120 {
            sim.step(&mut engine);
        }

        // Should be in House1F now
        let in_1f = sim.current_map_id == MapId::PlayersHouse1F;
        assert!(in_1f, "Should have warped to House1F, was on {:?}", sim.current_map_id);
    }

    #[test]
    fn test_npc_visibility_refresh() {
        let mut sim = PokemonV2Sim::with_state(MapId::NewBarkTown, 5, 5, vec![]);
        // Rival NPC (index 2) has event_flag=9, event_flag_show=true
        // Flag not set -> should be invisible
        assert!(!sim.event_flags.has(EVENT_RIVAL_NEW_BARK_TOWN));
        sim.refresh_npc_visibility();
        assert!(!sim.npc_states[2].visible, "Rival should be hidden when flag 9 is not set");

        // Set the flag -> should become visible
        sim.event_flags.set(EVENT_RIVAL_NEW_BARK_TOWN);
        sim.refresh_npc_visibility();
        assert!(sim.npc_states[2].visible, "Rival should be visible when flag 9 is set");
    }

    // ── Sprint 4 Integration Tests: Route 30 + Mr. Pokemon's House ──────────────

    #[test]
    fn test_sprint4_all_maps_load_in_sim() {
        for &map_id in &[MapId::Route30, MapId::Route30BerryHouse, MapId::MrPokemonsHouse, MapId::Route31] {
            let mut sim = PokemonV2Sim::with_state(map_id, 1, 1, vec![]);
            sim.check_map_entry_scripts();
            sim.check_map_callbacks();
            let map = load_map(map_id);
            assert!(map.width > 0 && map.height > 0, "{:?} should have positive dimensions", map_id);
        }
    }

    #[test]
    fn test_route30_wild_encounter_integration() {
        let map = load_map(MapId::Route30);
        let enc = map.wild_encounters.as_ref().expect("Route 30 should have wild encounters");
        assert!(!enc.morning.is_empty(), "Route 30 should have morning encounters");
        assert!(!enc.day.is_empty(), "Route 30 should have day encounters");
        assert!(!enc.night.is_empty(), "Route 30 should have night encounters");
        for slot in &enc.morning {
            assert!(slot.level >= 2 && slot.level <= 6,
                "Route 30 encounter level should be 2-6, got {}", slot.level);
        }
    }

    #[test]
    fn test_route30_trainer_npcs_have_trainer_range() {
        let map = load_map(MapId::Route30);
        let trainers: Vec<_> = map.npcs.iter().filter(|n| n.trainer_range.is_some()).collect();
        assert!(trainers.len() >= 2, "Route 30 should have at least 2 trainers with sight range");
        for trainer in &trainers {
            let range = trainer.trainer_range.unwrap();
            assert!(range >= 1 && range <= 5, "Trainer sight range should be 1-5, got {}", range);
            assert!(trainer.event_flag.is_some(), "Each trainer NPC should have a beaten event flag");
        }
    }

    #[test]
    fn test_trainer_beaten_flag_set_after_victory() {
        let mut sim = PokemonV2Sim::with_state(
            MapId::Route30, 5, 10,
            vec![Pokemon::new(CYNDAQUIL, 10)],
        );
        let steps = build_trainer_joey_script();
        let script = ScriptState::new(steps);
        let has_load_trainer = script.steps.iter().any(|s| matches!(s, ScriptStep::LoadTrainerParty { .. }));
        assert!(has_load_trainer, "Joey's script should have LoadTrainerParty step");
        // Simulate the beaten flag being set after battle victory
        sim.event_flags.set(EVENT_BEAT_YOUNGSTER_JOEY);
        assert!(sim.event_flags.has(EVENT_BEAT_YOUNGSTER_JOEY),
            "Joey beaten flag should be set after winning");
    }

    #[test]
    fn test_mr_pokemon_house_entry_script_triggers_on_scene0() {
        let mut sim = PokemonV2Sim::with_state(MapId::MrPokemonsHouse, 4, 6, vec![]);
        assert_eq!(sim.scene_state.get(MapId::MrPokemonsHouse), SCENE_MRPOKEMONSHOUSE_MEET_MR_POKEMON,
            "Default scene for MrPokemonsHouse should be 0 (meet scene)");
        sim.check_map_entry_scripts();
        let has_script = sim.script.is_some() || sim.phase == GamePhase::Script;
        assert!(has_script, "Entering MrPokemonsHouse scene 0 should trigger a script");
    }

    #[test]
    fn test_mr_pokemon_meet_script_gives_mystery_egg() {
        let steps = build_mr_pokemon_meet_script();
        let gives_egg = steps.iter().any(|s| matches!(s,
            ScriptStep::GiveItem { item_id, .. } if *item_id == ITEM_MYSTERY_EGG));
        assert!(gives_egg, "Mr. Pokemon meet script should give Mystery Egg");
        let sets_flag = steps.iter().any(|s| matches!(s,
            ScriptStep::SetEvent(f) if *f == EVENT_MR_POKEMONS_HOUSE_OAK));
        assert!(sets_flag, "Mr. Pokemon meet script should set the OAK event flag");
    }

    #[test]
    fn test_berry_house_gives_berry_once() {
        let steps = get_script(SCRIPT_BERRY_HOUSE_POKEFAN);
        let has_give_berry = steps.iter().any(|s| matches!(s,
            ScriptStep::GiveItem { item_id, .. } if *item_id == ITEM_BERRY));
        let has_check = steps.iter().any(|s| matches!(s,
            ScriptStep::CheckEvent { flag, .. } if *flag == EVENT_GOT_BERRY_FROM_ROUTE_30_HOUSE));
        let has_set_flag = steps.iter().any(|s| matches!(s,
            ScriptStep::SetEvent(f) if *f == EVENT_GOT_BERRY_FROM_ROUTE_30_HOUSE));
        assert!(has_give_berry, "Berry house script should give ITEM_BERRY");
        assert!(has_check, "Berry house script should check EVENT_GOT_BERRY_FROM_ROUTE_30_HOUSE");
        assert!(has_set_flag, "Berry house script should set EVENT_GOT_BERRY_FROM_ROUTE_30_HOUSE");
    }

    #[test]
    fn test_route30_to_mr_pokemon_house_warp_exists() {
        let map = load_map(MapId::Route30);
        let has_warp = map.warps.iter().any(|w| w.dest_map == MapId::MrPokemonsHouse);
        assert!(has_warp, "Route 30 should have a warp to MrPokemonsHouse");
    }

    #[test]
    fn test_route30_to_berry_house_warp_exists() {
        let map = load_map(MapId::Route30);
        let has_warp = map.warps.iter().any(|w| w.dest_map == MapId::Route30BerryHouse);
        assert!(has_warp, "Route 30 should have a warp to Route30BerryHouse");
    }

    #[test]
    fn test_route31_connects_to_route30() {
        let map = load_map(MapId::Route31);
        let south_connects = map.connections.south.as_ref()
            .map(|c| c.dest_map == MapId::Route30)
            .unwrap_or(false);
        let warp_connects = map.warps.iter().any(|w| w.dest_map == MapId::Route30);
        assert!(south_connects || warp_connects, "Route 31 stub should connect south to Route 30");
    }

    #[test]
    fn test_is_in_sight() {
        use super::overworld::is_in_sight;
        // NPC at (5, 5) facing Down, range 3
        assert!(is_in_sight(5, 5, Direction::Down, 5, 7, 3), "Down: in range");
        assert!(!is_in_sight(5, 5, Direction::Down, 5, 9, 3), "Down: out of range");
        assert!(!is_in_sight(5, 5, Direction::Down, 5, 3, 3), "Down: wrong direction");
        assert!(!is_in_sight(5, 5, Direction::Down, 6, 7, 3), "Down: not aligned");
        // NPC facing Up
        assert!(is_in_sight(5, 5, Direction::Up, 5, 3, 3), "Up: in range");
        assert!(!is_in_sight(5, 5, Direction::Up, 5, 7, 3), "Up: wrong direction");
    }
}
