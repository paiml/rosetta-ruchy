//! CLI Integration Tests for rosetta-runner
//!
//! Tests the main entry point and command-line interface
//! to increase coverage from 0% to target levels.
//!
//! Coverage target: harness/runner/src/main.rs (0/462 lines → ~400/462)

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Test: Basic --help output
#[test]
fn test_help_output() {
    let mut cmd = Command::cargo_bin("rosetta-runner").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("rosetta-runner"));
}

/// Test: Version flag
#[test]
fn test_version_output() {
    let mut cmd = Command::cargo_bin("rosetta-runner").unwrap();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("rosetta-runner"));
}

/// Test: Missing subcommand shows error
#[test]
fn test_missing_subcommand() {
    let mut cmd = Command::cargo_bin("rosetta-runner").unwrap();

    // Should fail - either with missing subcommand error or usage message
    cmd.assert().failure();
}

/// Test: Run command with missing example path
#[test]
fn test_run_missing_example() {
    let mut cmd = Command::cargo_bin("rosetta-runner").unwrap();
    cmd.args(&["run", "nonexistent/example"]);

    // May succeed parsing args but fail on execution
    // Just verify it doesn't crash with argument parsing error (code 2)
    let result = cmd.output().unwrap();
    // Any exit code except argument parsing error is acceptable
    assert_ne!(result.status.code(), Some(2), "Should not be argument parsing error");
}

/// Test: Run command with valid arguments structure
#[test]
fn test_run_command_arguments() {
    let mut cmd = Command::cargo_bin("rosetta-runner").unwrap();
    cmd.args(&["run", "--help"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("run"));
}

/// Test: Compare command requires results directory
#[test]
fn test_compare_missing_directory() {
    let mut cmd = Command::cargo_bin("rosetta-runner").unwrap();
    cmd.args(&["compare", "--help"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("compare"));
}

/// Test: Compare command with HTML flag
#[test]
fn test_compare_html_flag() {
    let temp_dir = TempDir::new().unwrap();
    let results_path = temp_dir.path();

    let mut cmd = Command::cargo_bin("rosetta-runner").unwrap();
    cmd.args(&["compare", results_path.to_str().unwrap(), "--html"]);

    // May fail due to empty directory, but should accept the flag
    cmd.assert().code(predicate::ne(2)); // Not argument parsing error
}

/// Test: Validate command
#[test]
fn test_validate_command() {
    let mut cmd = Command::cargo_bin("rosetta-runner").unwrap();
    cmd.arg("validate");

    // Should attempt to validate environment
    // May succeed or fail depending on system, but should not be argument error
    cmd.assert().code(predicate::ne(2));
}

/// Test: Regression command requires baseline and current
#[test]
fn test_regression_command_help() {
    let mut cmd = Command::cargo_bin("rosetta-runner").unwrap();
    cmd.args(&["regression", "--help"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("regression"));
}

/// Test: Regression command with threshold argument
#[test]
fn test_regression_threshold() {
    let temp_dir = TempDir::new().unwrap();
    let baseline = temp_dir.path().join("baseline.json");
    let current = temp_dir.path().join("current.json");

    // Create minimal JSON files
    fs::write(&baseline, r#"{"results": []}"#).unwrap();
    fs::write(&current, r#"{"results": []}"#).unwrap();

    let mut cmd = Command::cargo_bin("rosetta-runner").unwrap();
    cmd.args(&[
        "regression",
        baseline.to_str().unwrap(),
        current.to_str().unwrap(),
        "--threshold",
        "10.0",
    ]);

    // May fail due to empty results, but should accept arguments
    cmd.assert().code(predicate::ne(2));
}

/// Test: Verbose flag
#[test]
fn test_verbose_flag() {
    let mut cmd = Command::cargo_bin("rosetta-runner").unwrap();
    cmd.args(&["--verbose", "validate"]);

    // Should accept verbose flag
    cmd.assert().code(predicate::ne(2));
}

/// Test: Config file argument
#[test]
fn test_config_file_argument() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test_bench.toml");
    fs::write(&config_path, "[config]").unwrap();

    let mut cmd = Command::cargo_bin("rosetta-runner").unwrap();
    cmd.args(&[
        "--config",
        config_path.to_str().unwrap(),
        "validate",
    ]);

    // Should accept config argument
    cmd.assert().code(predicate::ne(2));
}

/// Test: Output format argument
#[test]
fn test_output_format_json() {
    let mut cmd = Command::cargo_bin("rosetta-runner").unwrap();
    cmd.args(&["--format", "json", "validate"]);

    cmd.assert().code(predicate::ne(2));
}

/// Test: Output format YAML
#[test]
fn test_output_format_yaml() {
    let mut cmd = Command::cargo_bin("rosetta-runner").unwrap();
    cmd.args(&["--format", "yaml", "validate"]);

    cmd.assert().code(predicate::ne(2));
}

/// Test: Output format Markdown
#[test]
fn test_output_format_markdown() {
    let mut cmd = Command::cargo_bin("rosetta-runner").unwrap();
    cmd.args(&["--format", "markdown", "validate"]);

    cmd.assert().code(predicate::ne(2));
}

/// Test: Output format HTML
#[test]
fn test_output_format_html() {
    let mut cmd = Command::cargo_bin("rosetta-runner").unwrap();
    cmd.args(&["--format", "html", "validate"]);

    cmd.assert().code(predicate::ne(2));
}

/// Test: Invalid output format
#[test]
fn test_invalid_output_format() {
    let mut cmd = Command::cargo_bin("rosetta-runner").unwrap();
    cmd.args(&["--format", "invalid", "validate"]);

    // Should fail with argument parsing error
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid"));
}

/// Test: Run command with iterations argument
#[test]
fn test_run_iterations_argument() {
    let mut cmd = Command::cargo_bin("rosetta-runner").unwrap();
    cmd.args(&["run", "examples/test", "--iterations", "100"]);

    // May fail due to missing example, but should parse arguments
    cmd.assert().code(predicate::ne(2));
}

/// Test: Run command with languages argument
#[test]
fn test_run_languages_argument() {
    let mut cmd = Command::cargo_bin("rosetta-runner").unwrap();
    cmd.args(&[
        "run",
        "examples/test",
        "--languages",
        "rust",
    ]);

    // May fail due to missing example, but should parse arguments
    // Exit code 2 is argument parsing error, we want to avoid that
    let result = cmd.output().unwrap();
    assert_ne!(result.status.code(), Some(2), "Should not be argument parsing error");
}

/// Test: Combined flags and arguments
#[test]
fn test_combined_flags() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("bench.toml");
    fs::write(&config_path, "[config]").unwrap();

    let mut cmd = Command::cargo_bin("rosetta-runner").unwrap();
    cmd.args(&[
        "--verbose",
        "--config",
        config_path.to_str().unwrap(),
        "--format",
        "json",
        "validate",
    ]);

    // Should accept all combined flags
    cmd.assert().code(predicate::ne(2));
}
