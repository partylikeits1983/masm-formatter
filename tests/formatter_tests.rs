use std::fs::read_to_string;
use std::path::Path;

use masm_formatter::format_code;

fn read_file_to_string(path: &Path) -> String {
    read_to_string(path).expect("Unable to read file")
}

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

#[test]
fn test_format_example1() {
    let input_path = Path::new("src/asm/example1.masm");
    let expected_output_path = Path::new("tests/expected/example1_formatted.masm");

    let input_code = read_file_to_string(&input_path);
    let expected_output = read_file_to_string(&expected_output_path);

    let formatted_code = format_code(&input_code);
    assert_eq!(formatted_code, expected_output);
}

#[test]
fn test_format_example2() {
    let input_path = Path::new("src/asm/example2.masm");
    let expected_output_path = Path::new("tests/expected/example2_formatted.masm");

    let input_code = read_file_to_string(&input_path);
    let expected_output = read_file_to_string(&expected_output_path);

    let formatted_code = format_code(&input_code);
    assert_eq!(formatted_code, expected_output);
}

#[test]
fn test_format_example3() {
    let input_path = Path::new("src/asm/example3.masm");
    let expected_output_path = Path::new("tests/expected/example3_formatted.masm");

    let input_code = read_file_to_string(&input_path);
    let expected_output = read_file_to_string(&expected_output_path);

    let formatted_code = format_code(&input_code);
    assert_eq!(formatted_code, expected_output);
}
