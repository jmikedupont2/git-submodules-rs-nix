use std::process::Command;
use std::path::PathBuf;

#[test]
fn test_project_file_lattice_builder_main_execution() {
    let cargo_bin_path = PathBuf::from(env!("CARGO_BIN_EXE_project_file_lattice_builder"));

    // Ensure the binary exists
    assert!(cargo_bin_path.exists(), "Binary does not exist at {:?}", cargo_bin_path);

    let output = Command::new(&cargo_bin_path)
        .output()
        .expect("Failed to execute project_file_lattice_builder binary");

    // Assert that the command ran successfully
    assert!(output.status.success(), "Command failed with {:?}", output);

    // Assert that the output contains expected content from its normal execution
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("stdout: {}", stdout);
    assert!(stdout.contains("Project File Lattice Builder"), "stdout does not contain 'Project File Lattice Builder'");
    assert!(stdout.contains("Conceptual Lattice of Project Files Concluded"), "stdout does not contain 'Conceptual Lattice of Project Files Concluded'");
}
