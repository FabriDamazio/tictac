#[cfg(test)]
mod integration {
    use std::process::Command;

    #[test]
    fn test_cli_with_valid_time() {
        let output = Command::new("target/debug/tictac")
            .args(&["1", "Test timer"])
            .output()
            .expect("Failed to run");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        assert!(stderr.is_empty());
        assert!(stdout.contains("Test timer"));
        assert!(stdout.contains("1"));
        assert!(output.status.success());
    }
}
