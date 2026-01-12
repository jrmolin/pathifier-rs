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

fn run_pathifier_status(input: &str) -> (String, String, i32) {
    let output = Command::new(env!("CARGO_BIN_EXE_pathifier"))
        .arg(input)
        .output()
        .expect("Failed to execute pathifier");

    let stdout = String::from_utf8(output.stdout)
        .expect("Invalid UTF-8 stdout")
        .trim()
        .to_string();
    let stderr = String::from_utf8(output.stderr)
        .expect("Invalid UTF-8 stderr")
        .trim()
        .to_string();
    let code = output.status.code().unwrap_or(-1);

    (stdout, stderr, code)
}

#[test]
fn test_basic_deduplication() {
    let result = run_pathifier("/usr/bin:/bin:/usr/bin:/usr/local/bin:/bin");
    assert_eq!(result, "/usr/bin:/bin:/usr/local/bin");
}

#[test]
fn test_no_duplicates() {
    let result = run_pathifier("/usr/bin:/bin:/usr/local/bin");
    assert_eq!(result, "/usr/bin:/bin:/usr/local/bin");
}

#[test]
fn test_all_same() {
    let result = run_pathifier("/usr/bin:/usr/bin:/usr/bin");
    assert_eq!(result, "/usr/bin");
}

#[test]
fn test_single_entry() {
    let result = run_pathifier("/usr/bin");
    assert_eq!(result, "/usr/bin");
}

#[test]
fn test_empty_string() {
    let result = run_pathifier("");
    assert_eq!(result, "");
}

#[test]
fn test_paths_with_spaces() {
    let result = run_pathifier("/path with spaces:/another:/path with spaces");
    assert_eq!(result, "/path with spaces:/another");
}

#[test]
fn test_complex_real_world_path() {
    let input = "/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:/usr/local/bin:/opt/homebrew/bin:/usr/bin";
    let expected = "/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin";
    let result = run_pathifier(input);
    assert_eq!(result, expected);
}

#[test]
fn test_preserves_first_occurrence() {
    // When there are duplicates, the first occurrence should be kept
    let result = run_pathifier("/first:/second:/first:/third:/second");
    assert_eq!(result, "/first:/second:/third");
}

#[test]
fn test_unicode_paths() {
    let result = run_pathifier("/home/用户/bin:/usr/bin:/home/用户/bin");
    assert_eq!(result, "/home/用户/bin:/usr/bin");
}

#[test]
fn test_empty_segments() {
    // Empty segments between colons
    let result = run_pathifier("/usr/bin::/bin::/usr/bin");
    assert_eq!(result, "/usr/bin::/bin");
}

#[test]
fn test_no_args_shows_usage() {
    let output = Command::new(env!("CARGO_BIN_EXE_pathifier"))
        .output()
        .expect("Failed to execute pathifier");

    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");
    assert!(stderr.contains("Usage:"));
    assert!(!output.status.success());
}
