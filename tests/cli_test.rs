use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_cli_non_interactive_flags() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_hensuki"));
    cmd.arg("-h");
    let output = cmd.output().expect("Failed to execute command");
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hensuki"));
    assert!(stdout.contains("--file"));
    assert!(stdout.contains("--mode"));
    assert!(stdout.contains("--select-type"));
}

#[test]
fn test_cli_stdin_pipe_validation_error() {
    // Pipe invalid file input
    let mut child = Command::new(env!("CARGO_BIN_EXE_hensuki"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(b"non_existent_test_file.txt\n1\n1\n")
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read output");
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should output error for non-existent file
    assert!(stderr.contains("Failed to open file") || stdout.contains("Failed to open file") || !output.status.success());
}
