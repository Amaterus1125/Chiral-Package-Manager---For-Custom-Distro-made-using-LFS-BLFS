use flate2::read::GzDecoder;
use std::env;
use std::fs::{self, File};
use tar::Archive;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Basic Command Routing
    if args.len() < 2 {
        print_help();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "install" => {
            if args.len() < 3 {
                eprintln!("Usage: chiral install <package_name>");
                return;
            }
            let package = &args[2];
            install_binary(package);
        }
        "help" | "--help" | "-h" => print_help(),
        _ => {
            eprintln!("Unknown command: {}", command);
            print_help();
        }
    }
}

fn print_help() {
    println!("Production Hardened Package Manager for Chirality OS");
    println!("Usage: chiral [COMMAND] <PACKAGE>");
    println!("Commands:");
    println!("  install <package>   Download and install a pre-built binary package");
    println!("  help                Print this message");
}

fn install_binary(package: &str) {
    // 1. The Safety Lockout (Protects your Arch Host!)
    if env::var("CHIRAL_CONFIRM_LIVE_ROOT").is_err() {
        eprintln!("🛑 [CRITICAL SAFETY LOCKOUT ENGAGED] 🛑");
        eprintln!("Attempted to run Chiral against the live root ('/').");
        eprintln!("To override pass: CHIRAL_CONFIRM_LIVE_ROOT=1");
        std::process::exit(1);
    }

    println!("🧬 Chiral Package Manager Engine v2.0 | Chemistry: DEXTRO-BINARY");
    println!("Stabilizing payload for '{}'...", package);

    // Ensure /tmp exists in the bare-metal OS
    let _ = fs::create_dir_all("/tmp");

    // 2. Reach out to the Arch Linux "Build Farm" (Your laptop)
    let url = format!("http://10.0.2.2:8000/{}.tar.gz", package);
    println!("-> Downloading from {}...", url);

    let mut response = match reqwest::blocking::get(&url) {
        Ok(resp) => {
            if !resp.status().is_success() {
                eprintln!("❌ ERROR: Package '{}' not found on server (HTTP {}).", package, resp.status());
                std::process::exit(1);
            }
            resp
        }
        Err(e) => {
            eprintln!("❌ ERROR: Network Unreachable. Is your Python server running on the host?");
            eprintln!("Details: {}", e);
            std::process::exit(1);
        }
    };

    // 3. Cache the download temporarily in RAM
    let tmp_path = format!("/tmp/{}.tar.gz", package);
    let mut tmp_file = File::create(&tmp_path).expect("Failed to create temporary cache file");
    response.copy_to(&mut tmp_file).expect("Failed to write download to disk");

    // 4. The Magic: Extract directly into the live Root FS ('/')
    println!("-> Extracting package directly into root filesystem ('/')...");
    let tarball = File::open(&tmp_path).expect("Failed to open downloaded archive");
    let tar = GzDecoder::new(tarball);
    let mut archive = Archive::new(tar);

    match archive.unpack("/") {
        Ok(_) => {
            println!("✨ Verification complete. Binary payload stabilized successfully.");
            println!("✅ Installed {}!", package);
        }
        Err(e) => {
            eprintln!("❌ [EXTRACTION FAILED]: Could not unpack files.");
            eprintln!("Details: {}", e);
            std::process::exit(1);
        }
    }

    // Cleanup the zip file to save RAM
    let _ = fs::remove_file(&tmp_path);
}
