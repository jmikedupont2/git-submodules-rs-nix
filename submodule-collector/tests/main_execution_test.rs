use std::process::Command;
use std::path::PathBuf;

#[test]
fn test_submodule_collector_main_execution() {
    let cargo_bin_path = PathBuf::from(env!("CARGO_BIN_EXE_submodule-collector"));

    // Ensure the binary exists
    assert!(cargo_bin_path.exists(), "Binary does not exist at {:?}", cargo_bin_path);

    let output = Command::new(&cargo_bin_path)
        .arg("--help") // Example: run with --help to check basic execution
        .output()
        .expect("Failed to execute submodule-collector binary");

    // Assert that the command ran successfully
    assert!(output.status.success(), "Command failed with {:?}", output);

    // Assert that the output contains expected help message content
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("stdout: {}", stdout);
    assert!(stdout.contains("submodule-collector"), "stdout does not contain 'submodule-collector'");
    assert!(stdout.contains("--help"), "stdout does not contain '--help'");
}
