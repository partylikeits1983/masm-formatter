use clap::Parser;
use masm_formatter::{format_code, format_file};
use std::fs;
use std::io;
use std::path::Path;
use std::process;

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
    /// Check for formatting issues without writing changes.
    #[arg(long)]
    check: bool,
}

/// Recursively traverse directories in a DFS manner and either format or check every .masm file.
/// Returns Ok(true) if any file is unformatted (in check mode), otherwise Ok(false).
fn process_path(path: &Path, check: bool) -> io::Result<bool> {
    let mut unformatted_found = false;
    if path.is_dir() {
        // DFS: For each entry in this directory, process recursively.
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if process_path(&entry.path(), check)? {
                unformatted_found = true;
            }
        }
    } else if path.extension().and_then(|s| s.to_str()) == Some("masm") {
        if check {
            let original = fs::read_to_string(path)?;
            let formatted = format_code(&original);
            if original != formatted {
                println!("File is not formatted correctly: {:?}", path);
                unformatted_found = true;
            }
        } else {
            println!("Formatting file: {:?}", path);
            format_file(path)?;
        }
    }
    Ok(unformatted_found)
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    match args {
        Cli::MasmFmt(args) => {
            let source_path = Path::new(&args.path);
            if source_path.exists() {
                let unformatted = if source_path.is_file() {
                    if source_path.extension().and_then(|s| s.to_str()) == Some("masm") {
                        if args.check {
                            let original = fs::read_to_string(source_path)?;
                            let formatted = format_code(&original);
                            if original != formatted {
                                println!("File is not formatted correctly: {:?}", source_path);
                                true
                            } else {
                                false
                            }
                        } else {
                            println!("Formatting file: {:?}", source_path);
                            format_file(source_path)?;
                            false
                        }
                    } else {
                        eprintln!("The specified file is not a .masm file: {:?}", source_path);
                        false
                    }
                } else {
                    process_path(source_path, args.check)?
                };

                if args.check && unformatted {
                    eprintln!("Formatting check failed: some files are not formatted correctly.");
                    process::exit(1);
                }
            } else {
                eprintln!("The specified path does not exist: {}", args.path);
            }
        }
    }
    Ok(())
}
