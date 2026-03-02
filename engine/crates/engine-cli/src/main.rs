fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("schema") => {
            std::fs::create_dir_all("schema").expect("Failed to create schema/");
            let schema = engine_core::schema::generate_schema();
            std::fs::write("schema/engine.schema.json", schema)
                .expect("Failed to write schema");
            println!("Schema written to schema/engine.schema.json");
        }
        Some("simulate") => {
            let frames: u64 = args.get(2)
                .and_then(|s| s.parse().ok())
                .unwrap_or(300);
            let json_output = args.iter().any(|a| a == "--json");

            let mut runner = engine_core::headless::HeadlessRunner::new(480, 720);
            let result = runner.run(
                engine_core::sleague::setup_fight_only,
                engine_core::sleague::update,
                engine_core::sleague::render,
                frames,
            );

            if json_output {
                let phase = result.game_state.get("tl_phase")
                    .and_then(|v| v.as_f64()).unwrap_or(0.0);
                let strokes = result.game_state.get("strokes")
                    .and_then(|v| v.as_f64()).unwrap_or(0.0);
                let ball_x = result.game_state.get("ball_x")
                    .and_then(|v| v.as_f64()).unwrap_or(0.0);
                let ball_y = result.game_state.get("ball_y")
                    .and_then(|v| v.as_f64()).unwrap_or(0.0);
                let hole_x = result.game_state.get("hole_x")
                    .and_then(|v| v.as_f64()).unwrap_or(0.0);
                let hole_y = result.game_state.get("hole_y")
                    .and_then(|v| v.as_f64()).unwrap_or(0.0);
                let dx = ball_x - hole_x;
                let dy = ball_y - hole_y;
                let dist_to_hole = (dx * dx + dy * dy).sqrt();

                println!(
                    "{{\"frames\":{},\"phase\":{},\"strokes\":{},\"ball\":[{:.1},{:.1}],\"hole\":[{:.1},{:.1}],\"dist_to_hole\":{:.1},\"fb_hash\":\"{:#x}\",\"sim_time\":{:.3}}}",
                    result.frames_run, phase, strokes,
                    ball_x, ball_y, hole_x, hole_y,
                    dist_to_hole, result.framebuffer_hash,
                    result.elapsed_sim_time,
                );
            } else {
                println!("S-League Headless Simulation");
                println!("  Frames: {}", result.frames_run);
                println!("  Sim time: {:.2}s", result.elapsed_sim_time);
                println!("  FB hash: {:#x}", result.framebuffer_hash);
                println!("  Game state:");
                let mut keys: Vec<_> = result.game_state.keys().collect();
                keys.sort();
                for key in keys {
                    let val = &result.game_state[key];
                    println!("    {}: {:?}", key, val);
                }
            }
        }
        _ => {
            eprintln!("Usage: engine-cli <command> [args...]");
            eprintln!("Commands:");
            eprintln!("  schema              Generate engine JSON schema");
            eprintln!("  simulate [frames]   Run headless S-League simulation");
            eprintln!("    --json            Output machine-readable JSON");
        }
    }
}
