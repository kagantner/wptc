use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Compiles a novel manuscript from a YAML configuration file into a single .docx file.", long_about = None)]
pub struct Args {
    /// Path to the YAML configuration file (e.g., testbuild.yaml)
    pub config_file: String,

    /// The directory where the compiled manuscript file will be saved (e.g., build/)
    pub output_dir: String,
}
