use clap::Parser;
use mytodo::Cli;

fn main() {
    let cli = Cli::parse(); // parsing argumen CLI menjadi struct
    if let Err(e) = mytodo::run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
