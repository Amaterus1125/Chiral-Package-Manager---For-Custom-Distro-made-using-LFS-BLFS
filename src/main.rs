cat << 'EOF' > src/main.rs
mod ui;
use ui::ChiralUI;
use std::env;
use chiral::install_binary; 

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: chiral install <package>");
        return;
    }

    if env::var("CHIRAL_CONFIRM_LIVE_ROOT").is_err() {
        eprintln!("🛑 SAFETY LOCKOUT: Set CHIRAL_CONFIRM_LIVE_ROOT=1");
        std::process::exit(1);
    }

    let package = &args[2];
    let mut ui = ChiralUI::new(false); // Dextro-Orange
    
    match install_binary(&mut ui, package) {
        Ok(_) => (),
        Err(e) => eprintln!("\n❌ Error: {}", e),
    }
}
EOF
