use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
enum ConstructType {
    Begin,
    If,
    Proc,
    Repeat,
    While,
    End,
    Else,
    Export,
    ExportLine, 
}

impl ConstructType {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "begin" => Some(Self::Begin),
            "if" => Some(Self::If),
            "else" => Some(Self::Else),
            "proc" => Some(Self::Proc),
            "export" => Some(Self::Export),
            "repeat" => Some(Self::Repeat),
            "while" => Some(Self::While),
            "end" => Some(Self::End),
            _ => None,
        }
    }
}

const INDENT: &str = "    ";

fn is_comment(line: &str) -> bool {
    line.trim_start().starts_with('#')
}

// to handle single line export statements
fn is_export_line(line: &str) -> bool {
    let re = Regex::new(r"^export\.\w+::\w+$").unwrap();
    re.is_match(line)
}

pub fn format_code(code: &str) -> String {
    let mut formatted_code = String::new();
    let mut indentation_level = 0;
    let mut construct_stack = Vec::new();
    let mut last_line_was_empty = false;
    let mut last_was_export_line = false;

    let mut lines = code.lines().peekable();

    while let Some(line) = lines.next() {
        let trimmed_line = line.trim();
        let first_word = trimmed_line.split('.').next();

        if !trimmed_line.is_empty() {
            if is_comment(trimmed_line) {
                if last_was_export_line {
                    formatted_code.push_str(trimmed_line);
                } else {
                    if let Some(prev_line) = formatted_code.lines().last() {
                        let prev_indent_level = prev_line.chars().take_while(|&c| c == ' ').count() / 4;
                        if prev_line.trim_start().starts_with("export") {
                            formatted_code.push_str(&INDENT.repeat(prev_indent_level + 1));
                        } else {
                            formatted_code.push_str(&INDENT.repeat(indentation_level));
                        }
                    } else {
                        formatted_code.push_str(&INDENT.repeat(indentation_level));
                    }
                    formatted_code.push_str(trimmed_line);
                }
                formatted_code.push('\n');
                last_line_was_empty = false;
                continue;
            }

            if is_export_line(trimmed_line) {
                construct_stack.push(ConstructType::ExportLine);
                formatted_code.push_str(trimmed_line);
                formatted_code.push('\n');
                last_line_was_empty = false;
                last_was_export_line = true;
                continue;
            }

            last_was_export_line = false;

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

                    formatted_code.push_str(&INDENT.repeat(indentation_level));
                    formatted_code.push_str(trimmed_line);
                    formatted_code.push('\n');
                    last_line_was_empty = false;

                    match construct {
                        ConstructType::Begin
                        | ConstructType::If
                        | ConstructType::Proc
                        | ConstructType::Export
                        | ConstructType::Repeat
                        | ConstructType::While
                        | ConstructType::Else => {
                            indentation_level += 1;
                        }
                        _ => {}
                    }

                    continue;
                }
            }

            formatted_code.push_str(&INDENT.repeat(indentation_level));
            formatted_code.push_str(trimmed_line);
            formatted_code.push('\n');
            last_line_was_empty = false;
        } else {
            if !last_line_was_empty {
                formatted_code.push('\n');
                last_line_was_empty = true;
            }
        }
    }

    formatted_code
}

pub fn format_file(file_path: &Path) -> io::Result<()> {
    let file = File::open(&file_path)?;
    let mut input_code = String::new();

    let reader = BufReader::new(file);
    for line in reader.lines() {
        input_code.push_str(&line?);
        input_code.push('\n');
    }

    let formatted_code = format_code(&input_code);

    let mut file = File::create(file_path)?;
    file.write_all(formatted_code.as_bytes())?;

    Ok(())
}
