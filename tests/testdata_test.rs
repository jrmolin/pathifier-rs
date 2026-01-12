use std::fs;
use std::process::Command;

fn run_pathifier(input: &str) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_pathifier"))
        .arg(input)
        .output()
        .expect("Failed to execute pathifier");

    String::from_utf8(output.stdout)
        .expect("Invalid UTF-8 output")
        .trim()
        .to_string()
}

fn load_test_case(name: &str) -> (String, String) {
    let input_path = format!("tests/testdata/{}.txt", name);
    let expected_path = format!("tests/testdata/{}_expected.txt", name);

    let input = fs::read_to_string(&input_path)
        .unwrap_or_else(|_| panic!("Failed to read {}", input_path))
        .trim()
        .to_string();

    let expected = fs::read_to_string(&expected_path)
        .unwrap_or_else(|_| panic!("Failed to read {}", expected_path))
        .trim()
        .to_string();

    (input, expected)
}

#[test]
fn test_typical_path_from_file() {
    let (input, expected) = load_test_case("typical_path");
    let result = run_pathifier(&input);
    assert_eq!(result, expected);
}

#[test]
fn test_many_duplicates_from_file() {
    let (input, expected) = load_test_case("many_duplicates");
    let result = run_pathifier(&input);
    assert_eq!(result, expected);
}

#[test]
fn test_special_characters_from_file() {
    let (input, expected) = load_test_case("special_characters");
    let result = run_pathifier(&input);
    assert_eq!(result, expected);
}
