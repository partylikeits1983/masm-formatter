use clap::Parser;
use masm_formatter::format_file;
use std::fs;
use std::path::Path;

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
    /// The folder or file path to search for .masm files.
    ///
    /// You can specify a folder to recursively format all .masm files,
    /// or a file path to format a single file.
    ///
    /// Example:
    ///     cargo masm-fmt source_dir
    ///     cargo masm-fmt some_file.masm
    path: String,
}

/// Recursively traverse directories in a DFS manner and format every .masm file.
fn process_path(path: &Path) -> std::io::Result<()> {
    if path.is_dir() {
        // DFS: For each entry in this directory, process recursively.
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            process_path(&entry.path())?;
        }
    } else if path.extension().and_then(|s| s.to_str()) == Some("masm") {
        println!("Formatting file: {:?}", path);
        format_file(path)?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    match args {
        Cli::MasmFmt(args) => {
            let source_path = Path::new(&args.path);
            if source_path.exists() {
                if source_path.is_file() {
                    // If it's a file, process just that file.
                    if source_path.extension().and_then(|s| s.to_str()) == Some("masm") {
                        println!("Formatting file: {:?}", source_path);
                        format_file(source_path)?;
                    } else {
                        eprintln!("The specified file is not a .masm file: {:?}", source_path);
                    }
                } else {
                    // Otherwise, recursively process the directory.
                    process_path(source_path)?;
                }
            } else {
                eprintln!("The specified path does not exist: {}", args.path);
            }
        }
    }
    Ok(())
}
