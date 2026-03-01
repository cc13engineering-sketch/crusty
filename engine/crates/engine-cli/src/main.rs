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
        Some("validate") => {
            let path = args.get(2).expect("Usage: engine-cli validate <file.world>");
            let source = std::fs::read_to_string(path).expect("Failed to read file");
            match engine_core::scripting::parser::parse_world(&source) {
                Ok(wf) => println!("✅ Valid. {} entities.", wf.entities.len()),
                Err(e) => { eprintln!("❌ {}", e); std::process::exit(1); }
            }
        }
        Some("test") => {
            let path = args.get(2).expect("Usage: engine-cli test <file.world>");
            let source = std::fs::read_to_string(path).expect("Failed to read file");
            let mut eng = engine_core::engine::Engine::new(1, 1);
            match engine_core::scripting::parser::parse_world(&source) {
                Ok(wf) => {
                    engine_core::scripting::loader::load_world_file(&wf, &mut eng.world, &mut eng.config);
                }
                Err(e) => { eprintln!("❌ Parse error: {}", e); std::process::exit(1); }
            }
            println!("Loaded {} entities.", eng.world.entity_count());

            let dt = 1.0 / 60.0;
            for _ in 0..60 {
                eng.physics_step(dt);
            }
            println!("After 60 ticks (1.0s simulated):");

            let mut any_nan = false;
            let mut any_escaped = false;
            let (bw, bh) = eng.config.bounds;

            for (entity, t) in eng.world.transforms.iter() {
                let name = eng.world.names.get_name(entity).unwrap_or("?");
                let vel = eng.world.rigidbodies.get(entity)
                    .map(|r| format!("vel=({:.1}, {:.1})", r.vx, r.vy))
                    .unwrap_or_else(|| "static".to_string());
                println!("  {}: pos=({:.1}, {:.1}) {}", name, t.x, t.y, vel);

                if t.x.is_nan() || t.y.is_nan() || t.x.is_infinite() || t.y.is_infinite() {
                    any_nan = true;
                }
                if t.x < -100.0 || t.x > bw + 100.0 || t.y < -100.0 || t.y > bh + 100.0 {
                    any_escaped = true;
                }
            }

            println!("Checks:");
            println!("  [{}] No NaN or Infinity values", if any_nan { "FAIL" } else { "OK" });
            println!("  [{}] No entities escaped bounds ({:.0}×{:.0})", if any_escaped { "FAIL" } else { "OK" }, bw, bh);
        }
        _ => {
            eprintln!("Usage: engine-cli <schema|validate|test> [args...]");
        }
    }
}
