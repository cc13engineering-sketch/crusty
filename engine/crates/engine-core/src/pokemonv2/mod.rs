// AI-INSTRUCTIONS: pokemonv2/mod.rs — Top-level PokemonV2Sim. Wires all submodules.
// This is the second Pokemon Crystal rewrite, sprint-driven from first principles.
// Reference old pokemon/ module for patterns but follow new sprint architecture.
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
            GamePhase::Battle | GamePhase::Menu => {} // stubs
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
        let result = step_overworld(
            &mut self.player,
            &mut self.camera,
            &self.current_map,
            &mut self.npc_states,
            &self.event_flags,
            &self.scene_state,
            engine,
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
        }
    }

    fn step_script_phase(&mut self, engine: &Engine) {
        if let Some(ref mut script) = self.script {
            let still_running = step_script(
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
            if !still_running {
                self.script = None;
                self.phase = GamePhase::Overworld;
                // Refresh NPC visibility after script changes flags
                self.refresh_npc_visibility();
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

        let running = step_script(&mut script, &mut player, &mut npc_states,
            &mut flags, &mut scenes, MapId::NewBarkTown,
            &mut party, &mut bag, false, false, false, false);
        assert!(running);
        assert!(script.text_buffer.is_some());

        let _running = step_script(&mut script, &mut player, &mut npc_states,
            &mut flags, &mut scenes, MapId::NewBarkTown,
            &mut party, &mut bag, true, false, false, false);

        let running = step_script(&mut script, &mut player, &mut npc_states,
            &mut flags, &mut scenes, MapId::NewBarkTown,
            &mut party, &mut bag, false, false, false, false);
        assert!(!running, "Script should end after End step");
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
}
