use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use glob::glob;
use clap::Parser;

#[derive(Debug, PartialEq, Clone)]
enum ConstructType {
    Begin,
    If,
    Proc,
    Repeat,
    While,
    End,
    Else,
}

impl ConstructType {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "begin" => Some(Self::Begin),
            "if" => Some(Self::If),
            "else" => Some(Self::Else),
            "proc" => Some(Self::Proc),
            "repeat" => Some(Self::Repeat),
            "while" => Some(Self::While),
            "end" => Some(Self::End),
            _ => None,
        }
    }
}

fn format_code(code: &str) -> String {
    let mut formatted_code = String::new();
    let mut indentation_level = 0;
    let mut construct_stack = Vec::new();

    for line in code.lines() {
        let trimmed_line = line.trim();
        let first_word = trimmed_line.split_whitespace().next();

        if !trimmed_line.is_empty() {
            if let Some(word) = first_word {
                if let Some(construct) = ConstructType::from_str(word) {
                    match construct {
                        ConstructType::End => {
                            if let Some(last_construct) = construct_stack.pop() {
                                if last_construct != ConstructType::End && indentation_level > 0 {
                                    indentation_level -= 1;
                                }
                            }
                        }
                        ConstructType::Else => {
                            if let Some(last_construct) = construct_stack.last() {
                                if *last_construct == ConstructType::If && indentation_level > 0 {
                                    indentation_level -= 1;
                                }
                            }
                        }
                        _ => {
                            construct_stack.push(construct.clone());
                        }
                    }

                    formatted_code.push_str(&"    ".repeat(indentation_level));
                    formatted_code.push_str(trimmed_line);
                    formatted_code.push('\n');

                    match construct {
                        ConstructType::Begin
                        | ConstructType::If
                        | ConstructType::Proc
                        | ConstructType::Repeat
                        | ConstructType::While
                        | ConstructType::Else => {
                            indentation_level += 1;
                        }
                        _ => {}
                    }
                } else {
                    formatted_code.push_str(&"    ".repeat(indentation_level));
                    formatted_code.push_str(trimmed_line);
                    formatted_code.push('\n');
                }
            }
        } else {
            formatted_code.push('\n'); // Replace empty lines with just a newline
        }
    }

    formatted_code
}

fn format_file(file_path: &Path) -> io::Result<()> {
    let file = File::open(&file_path)?;
    let mut input_code = String::new();

    let reader = BufReader::new(file);
    for line in reader.lines() {
        input_code.push_str(&line?);
        input_code.push('\n');
    }

    let formatted_code = format_code(&input_code);

    // Write the formatted code back to the file
    let mut file = File::create(file_path)?;
    file.write_all(formatted_code.as_bytes())?;

    Ok(())
}

#[derive(Parser)]
struct Cli {
    /// The pattern to search for (e.g., "**/*.masm")
    pattern: String,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    
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

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_simple() {
        let input = "begin\nend";
        let expected_output = "begin\nend\n";
        assert_eq!(format_code(input), expected_output);
    }

    #[test]
    fn test_format_with_indentation() {
        let input = "begin\n    proc\n    end\nend";
        let expected_output = "begin\n    proc\n    end\nend\n";
        assert_eq!(format_code(input), expected_output);
    }

    #[test]
    fn test_format_if_else() {
        let input = "if\n    begin\n    end\nelse\n    begin\n    end\nend";
        let expected_output = "if\n    begin\n    end\nelse\n    begin\n    end\nend\n";
        assert_eq!(format_code(input), expected_output);
    }

    #[test]
    fn test_format_complex() {
        let input = "begin\n    if\n        while\n        end\n    else\n        repeat\n        end\n    end\nend";
        let expected_output = "begin\n    if\n        while\n        end\n    else\n        repeat\n        end\n    end\nend\n";
        assert_eq!(format_code(input), expected_output);
    }

    #[test]
    fn test_format_with_empty_lines() {
        let input = "begin\n\n    proc\n\n    end\n\nend";
        let expected_output = "begin\n\n    proc\n\n    end\n\nend\n";
        assert_eq!(format_code(input), expected_output);
    }
}
