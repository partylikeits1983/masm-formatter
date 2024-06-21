use clap::Parser;
use glob::glob;
use masm_formatter::format_file;

#[derive(Parser)]
#[command(
    bin_name = "cargo",
    subcommand_required = true,
    arg_required_else_help = true
)]
enum Cli {
    #[command(name = "masm-fmt")]
    MasmFmt(MasmFmtArgs),
}

#[derive(Parser)]
struct MasmFmtArgs {
    /// The pattern to search for (e.g., "**/*.masm")
    pattern: String,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    match args {
        Cli::MasmFmt(args) => {
            for entry in glob(&args.pattern).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        if path.extension().and_then(|s| s.to_str()) == Some("masm") {
                            println!("Formatting file: {:?}", path);
                            format_file(&path)?;
                        }
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
        }
    }

    Ok(())
}
