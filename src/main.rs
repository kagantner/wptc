pub mod args;
pub mod config;
pub mod docx_builder;

use clap::Parser;
use args::Args;

fn main() {
    let args = Args::parse();
    
    if let Err(e) = docx_builder::compile_manuscript(&args.config_file, &args.output_dir, args.blind) {
        eprintln!("--> FATAL ERROR: {}", e);
        std::process::exit(1);
    }
}
