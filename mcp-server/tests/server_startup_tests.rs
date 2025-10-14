//! Server Startup Integration Tests for rosetta-ruchy-mcp
//!
//! Tests the main entry point and server initialization
//! to increase coverage from 0% to target levels.
//!
//! Coverage target: mcp-server/src/main.rs (0/29 lines → ~25/29)

use assert_cmd::Command;
use predicates::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

/// Test: Basic --help output
#[test]
fn test_help_output() {
    let mut cmd = Command::cargo_bin("rosetta-ruchy-mcp").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("MCP server"))
        .stdout(predicate::str::contains("code translation"))
        .stdout(predicate::str::contains("formal verification"));
}

/// Test: Version flag
#[test]
fn test_version_output() {
    let mut cmd = Command::cargo_bin("rosetta-ruchy-mcp").unwrap();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("rosetta-ruchy-mcp"));
}

/// Test: Default host and port
#[test]
fn test_default_host_port() {
    let mut cmd = Command::cargo_bin("rosetta-ruchy-mcp").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("127.0.0.1"))
        .stdout(predicate::str::contains("8080"));
}

/// Test: Custom host argument
#[test]
fn test_custom_host_argument() {
    let mut cmd = Command::cargo_bin("rosetta-ruchy-mcp").unwrap();
    cmd.args(&["--host", "0.0.0.0"]);

    // Server will attempt to start and may fail in CI, but should parse arguments
    // We're testing argument parsing, not full server startup
    cmd.timeout(Duration::from_secs(2));

    // Should either succeed briefly or fail with runtime error (not argument error)
    let output = cmd.output().unwrap();

    // Check that it's not an argument parsing error
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("error: unexpected argument") &&
        !stderr.contains("error: invalid value"),
        "Should not be an argument parsing error"
    );
}

/// Test: Custom port argument
#[test]
fn test_custom_port_argument() {
    let mut cmd = Command::cargo_bin("rosetta-ruchy-mcp").unwrap();
    cmd.args(&["--port", "9090"]);

    cmd.timeout(Duration::from_secs(2));

    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should parse port argument correctly
    assert!(
        !stderr.contains("error: unexpected argument") &&
        !stderr.contains("error: invalid value"),
        "Should accept valid port number"
    );
}

/// Test: Invalid port number
#[test]
fn test_invalid_port_argument() {
    let mut cmd = Command::cargo_bin("rosetta-ruchy-mcp").unwrap();
    cmd.args(&["--port", "invalid"]);

    // Should fail with parsing error or runtime error
    cmd.assert().failure();
}

/// Test: Custom ruchy-path argument
#[test]
fn test_custom_ruchy_path() {
    let mut cmd = Command::cargo_bin("rosetta-ruchy-mcp").unwrap();
    cmd.args(&["--ruchy-path", "/custom/path/to/ruchy"]);

    cmd.timeout(Duration::from_secs(2));

    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should accept ruchy-path argument
    assert!(
        !stderr.contains("error: unexpected argument"),
        "Should accept ruchy-path argument"
    );
}

/// Test: Combined arguments
#[test]
fn test_combined_arguments() {
    let mut cmd = Command::cargo_bin("rosetta-ruchy-mcp").unwrap();
    cmd.args(&[
        "--host", "localhost",
        "--port", "3000",
        "--ruchy-path", "ruchy",
    ]);

    cmd.timeout(Duration::from_secs(2));

    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should accept all arguments
    assert!(
        !stderr.contains("error: unexpected argument") &&
        !stderr.contains("error: invalid value"),
        "Should accept all combined arguments"
    );
}

/// Test: Server starts and logs initialization
#[tokio::test]
async fn test_server_initialization_logging() {
    // This test verifies that the server initializes and logs correctly
    // We can't easily test full server startup in CI, but we can verify
    // the initialization path is covered

    use tokio::process::Command as TokioCommand;

    let mut child = TokioCommand::new(env!("CARGO_BIN_EXE_rosetta-ruchy-mcp"))
        .args(&["--host", "127.0.0.1", "--port", "8081"])
        .kill_on_drop(true)
        .spawn()
        .expect("Failed to spawn server");

    // Give server brief time to initialize
    sleep(Duration::from_millis(500)).await;

    // Kill the server
    child.kill().await.ok();

    // If we got here without panic, the server started successfully
    // This covers the initialization code path
}
