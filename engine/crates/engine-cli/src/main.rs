use engine_core::demo_ball::DemoBall;
use engine_core::feel_preset::FeelPresetLibrary;
use engine_core::headless::playthrough::PlaythroughFile;
use engine_core::headless::{HeadlessRunner, RunConfig};
use engine_core::headless::{compare_hash_sequences, compare_sweep_outcomes, classify_batch, ClassifierConfig};
use engine_core::headless::{AblationConfig, Ablation, run_ablation_study};
use engine_core::headless::{sweep_variants, VariantSweepReport};
use engine_core::headless::{scan_for_highlights_report, HighlightConfig};
use engine_core::headless::{DashboardConfig, generate_dashboard_data};
use engine_core::input_frame::InputFrame;
use engine_core::policy::{NullPolicy, RandomPolicy};
use engine_core::simulation::Simulation;
use std::collections::{HashMap, BTreeMap};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("schema") => cmd_schema(),
        Some("record") => cmd_record(&args[2..]),
        Some("replay") => cmd_replay(&args[2..]),
        Some("batch") => cmd_batch(&args[2..]),
        Some("sweep") => cmd_sweep(&args[2..]),
        Some("golden") => cmd_golden(&args[2..]),
        Some("deaths") => cmd_deaths(&args[2..]),
        Some("divergence") => cmd_divergence(&args[2..]),
        Some("preset") => cmd_preset(&args[2..]),
        Some("ablation") => cmd_ablation(&args[2..]),
        Some("variants") => cmd_variants(),
        Some("variant-sweep") => cmd_variant_sweep(&args[2..]),
        Some("highlights") => cmd_highlights(&args[2..]),
        Some("dashboard-data") => cmd_dashboard_data(&args[2..]),
        Some("info") => cmd_info(),
        _ => print_usage(),
    }
}

fn print_usage() {
    eprintln!("Usage: engine-cli <command> [args...]");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  record    Record a playthrough to disk");
    eprintln!("  replay    Replay and verify a playthrough file");
    eprintln!("  batch     Run simulations across a seed range");
    eprintln!("  sweep     Run policy-driven simulations across seeds");
    eprintln!("  golden    Record or check golden baselines");
    eprintln!("  deaths    Classify terminal states from simulation runs");
    eprintln!("  divergence Compare two runs or sweeps to find divergence");
    eprintln!("  preset    Manage physics feel presets");
    eprintln!("  ablation  Run mechanic ablation study");
    eprintln!("  variants  List declared game variants");
    eprintln!("  variant-sweep  Sweep across variants and seeds");
    eprintln!("  highlights  Detect interesting moments in simulations");
    eprintln!("  dashboard-data  Generate dashboard JSON");
    eprintln!("  info      Print engine information");
    eprintln!("  schema    Generate engine JSON schema");
    eprintln!();
    eprintln!("Use 'engine-cli <command> --help' for command-specific help.");
}

// ─── Argument Parsing Helpers ───────────────────────────────────────

fn get_arg(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1).cloned())
}

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|a| a == flag)
}

fn parse_seed(args: &[String]) -> u64 {
    get_arg(args, "--seed")
        .and_then(|s| s.parse().ok())
        .unwrap_or(42)
}

fn parse_frames(args: &[String]) -> u64 {
    get_arg(args, "--frames")
        .and_then(|s| s.parse().ok())
        .unwrap_or(600)
}

fn parse_seed_range(args: &[String]) -> (u64, u64) {
    if let Some(range_str) = get_arg(args, "--seed-range") {
        if let Some((a, b)) = range_str.split_once("..") {
            let start = a.parse().unwrap_or(0);
            let end = b.parse().unwrap_or(100);
            return (start, end);
        }
    }
    (0, 100)
}

// ─── Commands ───────────────────────────────────────────────────────

fn cmd_schema() {
    std::fs::create_dir_all("schema").expect("Failed to create schema/");
    let schema = engine_core::schema::generate_schema();
    std::fs::write("schema/engine.schema.json", schema)
        .expect("Failed to write schema");
    println!("Schema written to schema/engine.schema.json");
}

fn cmd_record(args: &[String]) {
    if has_flag(args, "--help") {
        eprintln!("Usage: engine-cli record [--seed N] [--frames N] [--out FILE]");
        eprintln!();
        eprintln!("Record a demo_ball playthrough with NullPolicy (no input).");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --seed N      RNG seed (default: 42)");
        eprintln!("  --frames N    Frame count (default: 600)");
        eprintln!("  --out FILE    Output file (default: playthrough.json)");
        return;
    }

    let seed = parse_seed(args);
    let frames = parse_frames(args);
    let out = get_arg(args, "--out").unwrap_or_else(|| "playthrough.json".into());

    eprintln!("Recording {} frames with seed {}...", frames, seed);

    let inputs: Vec<InputFrame> = vec![InputFrame::default(); frames as usize];
    let mut game = DemoBall::new();
    let playthrough = PlaythroughFile::record(&mut game, seed, &inputs, frames, true);

    let json = playthrough.to_json_pretty().expect("Failed to serialize");
    std::fs::write(&out, &json).expect("Failed to write output file");

    eprintln!("Written to {}", out);
    eprintln!("  state_hash: {:#018x}", playthrough.final_state_hash);
    eprintln!("  fb_hash:    {:#018x}", playthrough.final_fb_hash);
}

fn cmd_replay(args: &[String]) {
    if has_flag(args, "--help") || args.is_empty() {
        eprintln!("Usage: engine-cli replay <FILE> [--verify] [--pretty]");
        eprintln!();
        eprintln!("Replay a recorded playthrough. Use --verify to check determinism.");
        return;
    }

    let file = &args[0];
    let verify = has_flag(args, "--verify");
    let pretty = has_flag(args, "--pretty");

    let json = std::fs::read_to_string(file).expect("Failed to read playthrough file");
    let playthrough = PlaythroughFile::from_json(&json).expect("Failed to parse playthrough");

    if verify {
        eprintln!("Verifying {} ({} frames, seed {})...",
            file, playthrough.frame_count, playthrough.seed);
        let mut game = DemoBall::new();
        match playthrough.verify(&mut game) {
            Ok(()) => {
                eprintln!("PASS: replay matches baseline");
            }
            Err(e) => {
                eprintln!("FAIL: {}", e);
                std::process::exit(1);
            }
        }
    } else if pretty {
        println!("{}", playthrough.to_json_pretty().unwrap());
    } else {
        println!("Playthrough: {} frames, seed {}", playthrough.frame_count, playthrough.seed);
        println!("  state_hash: {:#018x}", playthrough.final_state_hash);
        println!("  fb_hash:    {:#018x}", playthrough.final_fb_hash);
        if !playthrough.state_hashes.is_empty() {
            println!("  per-frame hashes: {}", playthrough.state_hashes.len());
        }
        if !playthrough.metadata.is_empty() {
            println!("  metadata: {:?}", playthrough.metadata);
        }
    }
}

fn cmd_batch(args: &[String]) {
    if has_flag(args, "--help") {
        eprintln!("Usage: engine-cli batch [--seed-range START..END] [--frames N] [--turbo] [--out FILE]");
        eprintln!();
        eprintln!("Run demo_ball across a range of seeds and collect results.");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --seed-range S..E   Seed range (default: 0..100)");
        eprintln!("  --frames N          Frames per run (default: 600)");
        eprintln!("  --turbo             Skip rendering for faster throughput");
        eprintln!("  --out FILE          Output JSONL file (default: stdout)");
        return;
    }

    let (seed_start, seed_end) = parse_seed_range(args);
    let frames = parse_frames(args);
    let turbo = has_flag(args, "--turbo");
    let out_file = get_arg(args, "--out");

    let config = RunConfig {
        turbo,
        capture_state_hashes: false,
    };

    let total = seed_end - seed_start;
    eprintln!("Batch: {} seeds ({}..{}), {} frames each{}",
        total, seed_start, seed_end, frames,
        if turbo { ", turbo" } else { "" });

    let mut results = Vec::new();
    for seed in seed_start..seed_end {
        let mut runner = HeadlessRunner::new(480, 270);
        let mut game = DemoBall::new();
        let inputs: Vec<InputFrame> = vec![InputFrame::default(); frames as usize];
        let result = runner.run_sim_frames(&mut game, seed, &inputs, frames, config.clone());

        let entry = serde_json::json!({
            "seed": seed,
            "frames": result.frames_run,
            "state_hash": format!("{:#018x}", result.state_hash),
            "fb_hash": format!("{:#018x}", result.framebuffer_hash),
            "score": result.game_state.get("score").and_then(|v| v.as_f64()).unwrap_or(0.0),
            "elapsed": result.elapsed_sim_time,
        });
        results.push(entry);
    }

    let output: String = results.iter()
        .map(|r| serde_json::to_string(r).unwrap())
        .collect::<Vec<_>>()
        .join("\n");

    if let Some(path) = out_file {
        std::fs::write(&path, &output).expect("Failed to write output");
        eprintln!("Written {} results to {}", results.len(), path);
    } else {
        println!("{}", output);
    }
}

fn cmd_sweep(args: &[String]) {
    if has_flag(args, "--help") {
        eprintln!("Usage: engine-cli sweep [--policy null|random] [--seed-range START..END] [--frames N] [--turbo] [--out FILE]");
        eprintln!();
        eprintln!("Run policy-driven simulations across a seed range.");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --policy P          Policy: null or random (default: null)");
        eprintln!("  --seed-range S..E   Seed range (default: 0..100)");
        eprintln!("  --frames N          Frames per run (default: 600)");
        eprintln!("  --turbo             Skip rendering");
        eprintln!("  --out FILE          Output JSONL file (default: stdout)");
        return;
    }

    let policy_name = get_arg(args, "--policy").unwrap_or_else(|| "null".into());
    let (seed_start, seed_end) = parse_seed_range(args);
    let frames = parse_frames(args);
    let turbo = has_flag(args, "--turbo");
    let out_file = get_arg(args, "--out");

    let config = RunConfig {
        turbo,
        capture_state_hashes: false,
    };

    let total = seed_end - seed_start;
    eprintln!("Sweep: {} seeds, {} frames, policy={}, {}",
        total, frames, policy_name,
        if turbo { "turbo" } else { "normal" });

    let mut results = Vec::new();
    for seed in seed_start..seed_end {
        let mut runner = HeadlessRunner::new(480, 270);
        let mut game = DemoBall::new();

        let result = match policy_name.as_str() {
            "random" => {
                let keys = vec!["Space".into(), "KeyA".into(), "KeyD".into()];
                let mut policy = RandomPolicy::new(seed.wrapping_mul(7919), keys);
                runner.run_with_policy(&mut game, &mut policy, seed, frames, config.clone())
            }
            _ => {
                let mut policy = NullPolicy;
                runner.run_with_policy(&mut game, &mut policy, seed, frames, config.clone())
            }
        };

        let entry = serde_json::json!({
            "seed": seed,
            "policy": policy_name,
            "frames": result.frames_run,
            "state_hash": format!("{:#018x}", result.state_hash),
            "score": result.game_state.get("score").and_then(|v| v.as_f64()).unwrap_or(0.0),
            "elapsed": result.elapsed_sim_time,
        });
        results.push(entry);
    }

    let output: String = results.iter()
        .map(|r| serde_json::to_string(r).unwrap())
        .collect::<Vec<_>>()
        .join("\n");

    if let Some(path) = out_file {
        std::fs::write(&path, &output).expect("Failed to write output");
        eprintln!("Written {} results to {}", results.len(), path);
    } else {
        println!("{}", output);
    }
}

fn cmd_golden(args: &[String]) {
    match args.first().map(|s| s.as_str()) {
        Some("record") => cmd_golden_record(&args[1..]),
        Some("check") => cmd_golden_check(&args[1..]),
        _ => {
            eprintln!("Usage: engine-cli golden <subcommand>");
            eprintln!();
            eprintln!("Subcommands:");
            eprintln!("  record [--seed N] [--frames N] [--out FILE]   Record a golden baseline");
            eprintln!("  check <FILE>                                  Verify against a golden baseline");
        }
    }
}

fn cmd_golden_record(args: &[String]) {
    if has_flag(args, "--help") {
        eprintln!("Usage: engine-cli golden record [--seed N] [--frames N] [--out FILE]");
        eprintln!();
        eprintln!("Record a golden baseline playthrough with per-frame hashes.");
        return;
    }

    let seed = parse_seed(args);
    let frames = parse_frames(args);
    let out = get_arg(args, "--out")
        .unwrap_or_else(|| "golden/baseline.json".into());

    eprintln!("Recording golden baseline: {} frames, seed {}...", frames, seed);

    let inputs: Vec<InputFrame> = vec![InputFrame::default(); frames as usize];
    let mut game = DemoBall::new();
    let mut playthrough = PlaythroughFile::record(&mut game, seed, &inputs, frames, true);
    playthrough.metadata.insert("type".into(), "golden".into());
    playthrough.metadata.insert("game".into(), "demo_ball".into());

    // Create parent directory if needed
    if let Some(parent) = std::path::Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).expect("Failed to create output directory");
        }
    }

    let json = playthrough.to_json_pretty().expect("Failed to serialize");
    std::fs::write(&out, &json).expect("Failed to write golden baseline");

    eprintln!("Golden baseline written to {}", out);
    eprintln!("  state_hash: {:#018x}", playthrough.final_state_hash);
    eprintln!("  per-frame hashes: {}", playthrough.state_hashes.len());
}

fn cmd_golden_check(args: &[String]) {
    if has_flag(args, "--help") || args.is_empty() {
        eprintln!("Usage: engine-cli golden check <FILE>");
        eprintln!();
        eprintln!("Verify a golden baseline by replaying and comparing hashes.");
        return;
    }

    let file = &args[0];
    let json = std::fs::read_to_string(file).expect("Failed to read golden baseline");
    let playthrough = PlaythroughFile::from_json(&json).expect("Failed to parse golden baseline");

    eprintln!("Checking golden baseline: {} ({} frames, seed {})...",
        file, playthrough.frame_count, playthrough.seed);

    let mut game = DemoBall::new();
    match playthrough.verify(&mut game) {
        Ok(()) => {
            eprintln!("PASS: golden baseline verified");
        }
        Err(e) => {
            eprintln!("FAIL: {}", e);
            std::process::exit(1);
        }
    }
}

fn cmd_info() {
    let version = env!("CARGO_PKG_VERSION");
    let mut info: HashMap<&str, serde_json::Value> = HashMap::new();
    info.insert("engine_version", serde_json::json!(version));
    info.insert("game", serde_json::json!("demo_ball"));
    info.insert("viewport", serde_json::json!({"width": 480, "height": 270}));
    info.insert("fixed_dt", serde_json::json!(1.0 / 60.0));
    info.insert("features", serde_json::json!([
        "deterministic_simulation",
        "state_hashing",
        "playthrough_replay",
        "policy_driven_runs",
        "turbo_mode",
        "golden_tests",
    ]));

    println!("{}", serde_json::to_string_pretty(&info).unwrap());
}

// ─── Deaths Command ─────────────────────────────────────────────────

fn cmd_deaths(args: &[String]) {
    if has_flag(args, "--help") {
        eprintln!("Usage: engine-cli deaths [--metric KEY] [--window N] [--seed-range S..E] [--frames N] [--out FILE] [--pretty]");
        eprintln!();
        eprintln!("Classify terminal states from demo_ball simulation runs.");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --metric KEY        Metric to track (default: score)");
        eprintln!("  --window N          Classification window in frames (default: 120)");
        eprintln!("  --seed-range S..E   Seed range (default: 0..100)");
        eprintln!("  --frames N          Frames per run (default: 600)");
        eprintln!("  --out FILE          Write JSON output to file");
        eprintln!("  --pretty            Pretty-print JSON output");
        return;
    }

    let metric_key = get_arg(args, "--metric").unwrap_or_else(|| "score".into());
    let window: usize = get_arg(args, "--window")
        .and_then(|s| s.parse().ok())
        .unwrap_or(120);
    let (seed_start, seed_end) = parse_seed_range(args);
    let frames = parse_frames(args);
    let out_file = get_arg(args, "--out");
    let pretty = has_flag(args, "--pretty");

    let config = ClassifierConfig::default()
        .with_metric_key(&metric_key)
        .with_window_size(window);

    let total = seed_end - seed_start;
    eprintln!("Deaths: {} seeds ({}..{}), {} frames each, tracking '{}'",
        total, seed_start, seed_end, frames, metric_key);

    let run_config = RunConfig {
        turbo: true,
        capture_state_hashes: false,
    };

    let mut runs: Vec<(u64, Vec<f64>)> = Vec::new();
    for seed in seed_start..seed_end {
        let mut runner = HeadlessRunner::new(480, 270);
        let mut game = DemoBall::new();
        let inputs: Vec<InputFrame> = vec![InputFrame::default(); frames as usize];
        let result = runner.run_sim_frames(&mut game, seed, &inputs, frames, run_config.clone());

        // Extract per-frame metric values from the final game state
        // For demo_ball, we track the score which changes over time
        let final_value = result.game_state.get(&metric_key)
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        // Create a simple trajectory: linear interpolation to final value
        let series: Vec<f64> = (0..frames)
            .map(|f| final_value * (f as f64 / frames as f64))
            .collect();
        runs.push((seed, series));
    }

    let report = classify_batch(&runs, &config);
    eprintln!("{}", report.summary());

    let json = if pretty {
        serde_json::to_string_pretty(&report).expect("Failed to serialize")
    } else {
        serde_json::to_string(&report).expect("Failed to serialize")
    };

    if let Some(path) = out_file {
        std::fs::write(&path, &json).expect("Failed to write output");
        eprintln!("Report written to {}", path);
    } else {
        println!("{}", json);
    }
}

// ─── Divergence Command ─────────────────────────────────────────────

fn cmd_divergence(args: &[String]) {
    match args.first().map(|s| s.as_str()) {
        Some("files") => cmd_divergence_files(&args[1..]),
        Some("sweep") => cmd_divergence_sweep(&args[1..]),
        _ => {
            eprintln!("Usage: engine-cli divergence <files|sweep> [args...]");
            eprintln!();
            eprintln!("Subcommands:");
            eprintln!("  files <a.json> <b.json> [--context N]");
            eprintln!("      Compare two playthrough files and find frame-level divergence.");
            eprintln!("  sweep <a.jsonl> <b.jsonl> [--key KEY]");
            eprintln!("      Compare two sweep result sets by matching seeds.");
        }
    }
}

fn cmd_divergence_files(args: &[String]) {
    if has_flag(args, "--help") || args.len() < 2 {
        eprintln!("Usage: engine-cli divergence files <a.json> <b.json> [--context N]");
        eprintln!();
        eprintln!("Compare two playthrough files and find frame-level divergence.");
        return;
    }

    let path_a = &args[0];
    let path_b = &args[1];
    let context_radius: usize = get_arg(args, "--context")
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);

    let src_a = std::fs::read_to_string(path_a)
        .unwrap_or_else(|e| { eprintln!("Failed to read {}: {}", path_a, e); std::process::exit(1); });
    let src_b = std::fs::read_to_string(path_b)
        .unwrap_or_else(|e| { eprintln!("Failed to read {}: {}", path_b, e); std::process::exit(1); });

    let play_a = PlaythroughFile::from_json(&src_a)
        .unwrap_or_else(|e| { eprintln!("Failed to parse {}: {}", path_a, e); std::process::exit(1); });
    let play_b = PlaythroughFile::from_json(&src_b)
        .unwrap_or_else(|e| { eprintln!("Failed to parse {}: {}", path_b, e); std::process::exit(1); });

    let report = compare_hash_sequences(
        &play_a.state_hashes,
        &play_b.state_hashes,
        path_a,
        path_b,
        context_radius,
    );

    print!("{}", report.summary());

    if let Some(ctx) = &report.context {
        println!("Context window (frames {}..{}):", ctx.start_frame, ctx.end_frame);
        for cf in &ctx.frames {
            let marker = if cf.matches { " " } else { "!" };
            println!(
                "  {} frame {:>6}: A={:016x} B={:016x}",
                marker, cf.frame, cf.hash_a, cf.hash_b
            );
        }
    }
}

fn cmd_divergence_sweep(args: &[String]) {
    if has_flag(args, "--help") || args.len() < 2 {
        eprintln!("Usage: engine-cli divergence sweep <a.jsonl> <b.jsonl> [--key KEY]");
        eprintln!();
        eprintln!("Compare two sweep result sets by matching seeds.");
        return;
    }

    let path_a = &args[0];
    let path_b = &args[1];
    let metric_key = get_arg(args, "--key").unwrap_or_else(|| "score".into());

    let results_a = load_sweep_jsonl(path_a, &metric_key);
    let results_b = load_sweep_jsonl(path_b, &metric_key);

    let report = compare_sweep_outcomes(&results_a, &results_b);
    print!("{}", report.summary());
}

fn load_sweep_jsonl(path: &str, metric_key: &str) -> Vec<(u64, f64)> {
    let content = std::fs::read_to_string(path)
        .unwrap_or_else(|e| { eprintln!("Failed to read {}: {}", path, e); std::process::exit(1); });
    let mut results = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        let obj: serde_json::Value = serde_json::from_str(line)
            .unwrap_or_else(|e| { eprintln!("Invalid JSON in {}: {}", path, e); std::process::exit(1); });
        let seed = obj.get("seed").and_then(|v| v.as_u64()).unwrap_or(0);
        let value = obj.get(metric_key).and_then(|v| v.as_f64()).unwrap_or(0.0);
        results.push((seed, value));
    }
    results
}

// ─── Preset Command ─────────────────────────────────────────────────

fn cmd_preset(args: &[String]) {
    match args.first().map(|s| s.as_str()) {
        Some("list") => cmd_preset_list(),
        Some("show") => cmd_preset_show(&args[1..]),
        Some("apply") => cmd_preset_apply(&args[1..]),
        _ => {
            eprintln!("Usage: engine-cli preset <list|show|apply> [args...]");
            eprintln!();
            eprintln!("Subcommands:");
            eprintln!("  list              List all built-in presets");
            eprintln!("  show <name>       Show TOML for a named preset");
            eprintln!("  apply <name>      Apply preset and report parameters");
        }
    }
}

fn cmd_preset_list() {
    let lib = FeelPresetLibrary::with_builtins();
    println!("Built-in Feel Presets ({}):", lib.len());
    println!();
    for name in lib.list() {
        if let Some(preset) = lib.get(name) {
            let desc = if preset.description.is_empty() {
                "(no description)".to_string()
            } else {
                preset.description.clone()
            };
            println!("  {:<20} {} ({} params)", name, desc, preset.params.len());
        }
    }
}

fn cmd_preset_show(args: &[String]) {
    let name = match args.first() {
        Some(n) => n.as_str(),
        None => {
            eprintln!("Usage: engine-cli preset show <name>");
            eprintln!();
            let lib = FeelPresetLibrary::with_builtins();
            eprintln!("Available presets:");
            for n in lib.list() { eprintln!("  {}", n); }
            return;
        }
    };

    let lib = FeelPresetLibrary::with_builtins();
    match lib.get(name) {
        Some(preset) => {
            match preset.to_toml() {
                Ok(toml_str) => println!("{}", toml_str),
                Err(e) => {
                    eprintln!("Failed to serialize preset '{}': {}", name, e);
                    std::process::exit(1);
                }
            }
        }
        None => {
            eprintln!("Unknown preset: '{}'", name);
            eprintln!();
            eprintln!("Available presets:");
            for n in lib.list() { eprintln!("  {}", n); }
            std::process::exit(1);
        }
    }
}

fn cmd_preset_apply(args: &[String]) {
    let name = match args.first() {
        Some(n) => n.as_str(),
        None => {
            eprintln!("Usage: engine-cli preset apply <name>");
            let lib = FeelPresetLibrary::with_builtins();
            eprintln!("Available presets:");
            for n in lib.list() { eprintln!("  {}", n); }
            return;
        }
    };

    let lib = FeelPresetLibrary::with_builtins();
    let preset = match lib.get(name) {
        Some(p) => p,
        None => {
            eprintln!("Unknown preset: '{}'", name);
            std::process::exit(1);
        }
    };

    let mut eng = engine_core::engine::Engine::new(480, 270);
    preset.apply(&mut eng);

    println!("Applied preset '{}' to engine global state.", name);
    if !preset.description.is_empty() {
        println!("Description: {}", preset.description);
    }
    println!();
    println!("Parameters set ({}):", preset.params.len());
    for (key, value) in &preset.params {
        println!("  {:<30} = {}", key, value);
    }
}

// ─── Ablation Command ───────────────────────────────────────────────

fn cmd_ablation(args: &[String]) {
    if has_flag(args, "--help") {
        eprintln!("Usage: engine-cli ablation [--seed-range S..E] [--frames N] [--metric KEY] [--out FILE] [--pretty]");
        eprintln!();
        eprintln!("Run a mechanic ablation study on DemoBall.");
        eprintln!();
        eprintln!("Tests what happens when specific mechanics are disabled or modified.");
        eprintln!("Default ablations: no_gravity, double_speed, half_speed, no_bounce.");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --seed-range S..E   Seed range (default: 0..10)");
        eprintln!("  --frames N          Frames per run (default: 600)");
        eprintln!("  --metric KEY        Metric to measure (default: score)");
        eprintln!("  --out FILE          Write JSON output to file");
        eprintln!("  --pretty            Pretty-print JSON output");
        return;
    }

    let (seed_start, seed_end) = if let Some(range_str) = get_arg(args, "--seed-range") {
        if let Some((a, b)) = range_str.split_once("..") {
            let start = a.parse().unwrap_or(0);
            let end = b.parse().unwrap_or(10);
            (start, end)
        } else {
            (0, 10)
        }
    } else {
        (0, 10)
    };
    let frames = get_arg(args, "--frames")
        .and_then(|s| s.parse().ok())
        .unwrap_or(600);
    let metric_key = get_arg(args, "--metric").unwrap_or_else(|| "score".into());
    let out_file = get_arg(args, "--out");
    let pretty = has_flag(args, "--pretty");

    // Build default ablations using global_state keys that DemoBall may read.
    let mut no_gravity = BTreeMap::new();
    no_gravity.insert("gravity".to_string(), 0.0);

    let mut double_speed = BTreeMap::new();
    double_speed.insert("speed_mult".to_string(), 2.0);

    let mut half_speed = BTreeMap::new();
    half_speed.insert("speed_mult".to_string(), 0.5);

    let mut no_bounce = BTreeMap::new();
    no_bounce.insert("bounce_damping".to_string(), 0.0);

    let config = AblationConfig {
        baseline_params: BTreeMap::new(),
        ablations: vec![
            Ablation { name: "no_gravity".to_string(), params: no_gravity },
            Ablation { name: "double_speed".to_string(), params: double_speed },
            Ablation { name: "half_speed".to_string(), params: half_speed },
            Ablation { name: "no_bounce".to_string(), params: no_bounce },
        ],
        seed_range: (seed_start, seed_end),
        frames,
        metric_key,
    };

    let total_seeds = seed_end - seed_start;
    let total_configs = 1 + config.ablations.len() as u64;
    eprintln!("Ablation: {} seeds ({}..{}), {} frames each, {} configurations ({} total runs)",
        total_seeds, seed_start, seed_end, frames, total_configs,
        total_seeds * total_configs);

    let report = run_ablation_study(
        || HeadlessRunner::new(480, 270),
        DemoBall::new,
        &config,
    );

    eprintln!("{}", report.summary());

    let json = if pretty {
        serde_json::to_string_pretty(&report).expect("Failed to serialize")
    } else {
        serde_json::to_string(&report).expect("Failed to serialize")
    };

    if let Some(path) = out_file {
        std::fs::write(&path, &json).expect("Failed to write output");
        eprintln!("Report written to {}", path);
    } else {
        println!("{}", json);
    }
}

// ─── Variants Command ──────────────────────────────────────────────

fn cmd_variants() {
    let game = DemoBall::new();
    let variants = game.variants();

    if variants.is_empty() {
        println!("No variants declared by DemoBall.");
        return;
    }

    println!("DemoBall variants ({}):", variants.len());
    println!();
    for variant in &variants {
        println!("  {}", variant.display_name());
        for (key, value) in variant.iter() {
            println!("    {:<30} = {}", key, value);
        }
    }
}

// ─── Variant Sweep Command ─────────────────────────────────────────

fn cmd_variant_sweep(args: &[String]) {
    if has_flag(args, "--help") {
        eprintln!("Usage: engine-cli variant-sweep [--seed-range S..E] [--frames N] [--turbo] [--out FILE] [--pretty]");
        eprintln!();
        eprintln!("Sweep across all declared DemoBall variants and seeds.");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --seed-range S..E   Seed range (default: 0..10)");
        eprintln!("  --frames N          Frames per run (default: 600)");
        eprintln!("  --turbo             Skip rendering");
        eprintln!("  --out FILE          Write output to file");
        eprintln!("  --pretty            Pretty-print JSON output");
        return;
    }

    let (seed_start, seed_end) = if let Some(range_str) = get_arg(args, "--seed-range") {
        if let Some((a, b)) = range_str.split_once("..") {
            (a.parse().unwrap_or(0u64), b.parse().unwrap_or(10u64))
        } else { (0, 10) }
    } else { (0, 10) };
    let frames = get_arg(args, "--frames").and_then(|s| s.parse().ok()).unwrap_or(600u64);
    let turbo = has_flag(args, "--turbo");
    let out_file = get_arg(args, "--out");
    let pretty = has_flag(args, "--pretty");

    let game = DemoBall::new();
    let variants = game.variants();
    if variants.is_empty() {
        eprintln!("No variants declared. Nothing to sweep.");
        return;
    }

    let seeds: Vec<u64> = (seed_start..seed_end).collect();
    let config = RunConfig { turbo, capture_state_hashes: false };

    let total_runs = variants.len() * seeds.len();
    eprintln!("Variant sweep: {} variants x {} seeds = {} runs, {} frames each",
        variants.len(), seeds.len(), total_runs, frames);

    let results = sweep_variants(&|| DemoBall::new(), &seeds, frames, config, &variants);
    let report = VariantSweepReport::new(results);

    eprintln!("{}", report.summary());

    let mut lines = Vec::new();
    for vr in &report.results {
        let entry = serde_json::json!({
            "variant": vr.variant_name,
            "frames": vr.result.frames_run,
            "state_hash": format!("{:#018x}", vr.result.state_hash),
            "score": vr.result.get_f64("score").unwrap_or(0.0),
            "elapsed": vr.result.elapsed_sim_time,
        });
        lines.push(if pretty {
            serde_json::to_string_pretty(&entry).unwrap()
        } else {
            serde_json::to_string(&entry).unwrap()
        });
    }
    let output = lines.join("\n");

    if let Some(path) = out_file {
        std::fs::write(&path, &output).expect("Failed to write output");
        eprintln!("Written {} results to {}", report.results.len(), path);
    } else {
        println!("{}", output);
    }
}

// ─── Highlights Command ────────────────────────────────────────────

fn cmd_highlights(args: &[String]) {
    if has_flag(args, "--help") {
        eprintln!("Usage: engine-cli highlights [--seed-range S..E] [--frames N] [--metric KEY] [--window N] [--out FILE] [--pretty]");
        eprintln!();
        eprintln!("Detect interesting moments (spikes, drops, milestones, near-death) in DemoBall runs.");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --seed-range S..E   Seed range (default: 0..10)");
        eprintln!("  --frames N          Frames per run (default: 600)");
        eprintln!("  --metric KEY        Metric to track (default: score)");
        eprintln!("  --window N          Rolling window size (default: 30)");
        eprintln!("  --out FILE          Write JSON output to file");
        eprintln!("  --pretty            Pretty-print JSON output");
        return;
    }

    let (seed_start, seed_end) = if let Some(range_str) = get_arg(args, "--seed-range") {
        if let Some((a, b)) = range_str.split_once("..") {
            (a.parse().unwrap_or(0u64), b.parse().unwrap_or(10u64))
        } else { (0, 10) }
    } else { (0, 10) };
    let frames = get_arg(args, "--frames").and_then(|s| s.parse().ok()).unwrap_or(600u64);
    let metric_key = get_arg(args, "--metric").unwrap_or_else(|| "score".into());
    let window: usize = get_arg(args, "--window").and_then(|s| s.parse().ok()).unwrap_or(30);
    let out_file = get_arg(args, "--out");
    let pretty = has_flag(args, "--pretty");

    let total = seed_end - seed_start;
    eprintln!("Highlights: {} seeds ({}..{}), {} frames each, tracking '{}'",
        total, seed_start, seed_end, frames, metric_key);

    let capture_keys = vec![metric_key.clone()];
    let run_config = RunConfig { turbo: true, capture_state_hashes: false };

    let highlight_config = HighlightConfig {
        spike_threshold: 2.0,
        near_death_threshold: 5.0,
        milestone_values: vec![100.0, 500.0, 1000.0],
        tracked_metrics: vec![metric_key.clone()],
        window_size: window,
    };

    let mut all_reports = Vec::new();
    for seed in seed_start..seed_end {
        let mut runner = HeadlessRunner::new(480, 270);
        let mut game = DemoBall::new();
        let inputs: Vec<InputFrame> = vec![InputFrame::default(); frames as usize];

        let (_result, captured) = runner.run_with_capture(
            &mut game, seed, &inputs, frames, run_config.clone(), &capture_keys,
        );

        let frame_values: Vec<(u64, f64)> = captured.iter().enumerate().map(|(i, row)| {
            let val = row.iter()
                .find(|(k, _)| k == &metric_key)
                .map(|(_, v)| *v)
                .unwrap_or(0.0);
            (i as u64, val)
        }).collect();

        let report = scan_for_highlights_report(&metric_key, &frame_values, &highlight_config);
        if !report.highlights.is_empty() {
            all_reports.push(serde_json::json!({
                "seed": seed,
                "total_frames": report.total_frames,
                "highlight_count": report.highlights.len(),
                "summary": report.summary(),
                "highlights": report.highlights,
            }));
        }
    }

    eprintln!("Found highlights in {} of {} seeds", all_reports.len(), total);

    let output = if pretty {
        serde_json::to_string_pretty(&all_reports).expect("Failed to serialize")
    } else {
        all_reports.iter()
            .map(|r| serde_json::to_string(r).unwrap())
            .collect::<Vec<_>>()
            .join("\n")
    };

    if let Some(path) = out_file {
        std::fs::write(&path, &output).expect("Failed to write output");
        eprintln!("Report written to {}", path);
    } else {
        println!("{}", output);
    }
}

// ─── Dashboard Data Command ────────────────────────────────────────

fn cmd_dashboard_data(args: &[String]) {
    if has_flag(args, "--help") {
        eprintln!("Usage: engine-cli dashboard-data [--seed-range S..E] [--frames N] [--metric KEY] [--out FILE]");
        eprintln!();
        eprintln!("Generate dashboard JSON by running sweep, death classification,");
        eprintln!("highlight detection, and optional ablation analysis.");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --seed-range S..E   Seed range (default: 0..100)");
        eprintln!("  --frames N          Frames per run (default: 600)");
        eprintln!("  --metric KEY        Metric to track (default: score)");
        eprintln!("  --with-ablation     Include ablation analysis (slower)");
        eprintln!("  --out FILE          Output file (default: dashboard.json)");
        return;
    }

    let (seed_start, seed_end) = if let Some(range_str) = get_arg(args, "--seed-range") {
        if let Some((a, b)) = range_str.split_once("..") {
            (a.parse().unwrap_or(0u64), b.parse().unwrap_or(100u64))
        } else { (0, 100) }
    } else { (0, 100) };
    let frames = get_arg(args, "--frames").and_then(|s| s.parse().ok()).unwrap_or(600u64);
    let metric_key = get_arg(args, "--metric").unwrap_or_else(|| "score".into());
    let out = get_arg(args, "--out").unwrap_or_else(|| "dashboard.json".into());
    let with_ablation = has_flag(args, "--with-ablation");

    let total = seed_end - seed_start;
    eprintln!("Dashboard: {} seeds ({}..{}), {} frames each, tracking '{}'",
        total, seed_start, seed_end, frames, metric_key);

    let ablation_config = if with_ablation {
        let mut no_gravity = BTreeMap::new();
        no_gravity.insert("gravity".to_string(), 0.0);
        let mut double_speed = BTreeMap::new();
        double_speed.insert("speed_mult".to_string(), 2.0);
        let mut no_bounce = BTreeMap::new();
        no_bounce.insert("bounce_damping".to_string(), 0.0);
        Some(AblationConfig {
            baseline_params: BTreeMap::new(),
            ablations: vec![
                Ablation { name: "no_gravity".into(), params: no_gravity },
                Ablation { name: "double_speed".into(), params: double_speed },
                Ablation { name: "no_bounce".into(), params: no_bounce },
            ],
            seed_range: (seed_start, seed_end.min(seed_start + 20)), // cap ablation seeds
            frames,
            metric_key: metric_key.clone(),
        })
    } else {
        None
    };

    let config = DashboardConfig {
        seed_range: (seed_start, seed_end),
        frames,
        metric_key,
        ablation_config,
    };

    let mut data = generate_dashboard_data(
        || HeadlessRunner::new(480, 270),
        DemoBall::new,
        &config,
    );
    data.generated_at = chrono_iso_now();

    let json = serde_json::to_string_pretty(&data).expect("Failed to serialize");

    if let Some(parent) = std::path::Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).ok();
        }
    }
    std::fs::write(&out, &json).expect("Failed to write dashboard data");

    eprintln!("Dashboard data written to {}", out);
    eprintln!("  Seeds: {}, Frames: {}", data.sweep.total_seeds, data.sweep.frames_per_run);
    eprintln!("  Score: mean={:.2}, median={:.2}, min={:.2}, max={:.2}",
        data.sweep.mean, data.sweep.median, data.sweep.min, data.sweep.max);
    eprintln!("  Deaths: {} total ({} close-call, {} blowout, {} cliff, {} attrition)",
        data.deaths.total, data.deaths.close_call, data.deaths.blowout,
        data.deaths.cliff, data.deaths.attrition);
    eprintln!("  Highlights: {}", data.highlights.len());
    if data.ablation.is_some() {
        eprintln!("  Ablation: included");
    }
}

/// Simple ISO 8601 timestamp without requiring the chrono crate.
fn chrono_iso_now() -> String {
    // Use a deterministic placeholder since we don't have chrono.
    // In production the CLI would use std::time::SystemTime.
    use std::time::SystemTime;
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(d) => format!("{}s-since-epoch", d.as_secs()),
        Err(_) => "unknown".to_string(),
    }
}
