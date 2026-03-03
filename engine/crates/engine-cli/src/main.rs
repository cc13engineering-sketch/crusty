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
            eprintln!("The S-League demo game has been removed.");
            eprintln!("Use the headless runner API with your own setup/update/render functions.");
        }
        _ => {
            eprintln!("Usage: engine-cli <command> [args...]");
            eprintln!("Commands:");
            eprintln!("  schema              Generate engine JSON schema");
            eprintln!("  simulate            (removed — provide your own game)");
        }
    }
}
