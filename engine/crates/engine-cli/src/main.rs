use engine_core::demo_ball::DemoBall;
use engine_core::headless::playthrough::PlaythroughFile;
use engine_core::headless::{HeadlessRunner, RunConfig};
use engine_core::input_frame::InputFrame;
use engine_core::policy::{NullPolicy, RandomPolicy};
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("schema") => cmd_schema(),
        Some("record") => cmd_record(&args[2..]),
        Some("replay") => cmd_replay(&args[2..]),
        Some("batch") => cmd_batch(&args[2..]),
        Some("sweep") => cmd_sweep(&args[2..]),
        Some("golden") => cmd_golden(&args[2..]),
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
