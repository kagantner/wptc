use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Compiles a novel manuscript from a YAML configuration file into a single .docx file.", long_about = None)]
pub struct Args {
    /// Path to the YAML configuration file (e.g., testbuild.yaml)
    pub config_file: String,

    /// The directory where the compiled manuscript file will be saved (e.g., build/)
    pub output_dir: String,

    /// If true, removes author name and contact info from the manuscript
    #[arg(long)]
    pub blind: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parsing() {
        let args = Args::try_parse_from(["mdmf", "config.yaml", "out_dir"]);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.config_file, "config.yaml");
        assert_eq!(args.output_dir, "out_dir");
    }

    #[test]
    fn test_missing_args() {
        let args = Args::try_parse_from(["mdmf", "config.yaml"]);
        assert!(args.is_err());
    }
}
