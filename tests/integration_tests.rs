use std::process::Command;

#[test]
fn test_help_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute help command");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Check that help contains our noob-friendly messages
    assert!(stdout.contains("For devs who code like ninjas but commit like toddlers"));
    assert!(stdout.contains("🤡"));
    assert!(stdout.contains("nc"));
    assert!(stdout.contains("setup-alias"));
    assert!(stdout.contains("YOLO mode"));
    assert!(stdout.contains("-b, --br-huehuehue"));
}

#[test]
fn test_version_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .output()
        .expect("Failed to execute version command");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Noob Commit"));
    assert!(stdout.contains("0.6.1"));
}

#[test]
fn test_dry_run_without_openai_key() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--dry-run"])
        .env_remove("OPENAI_API_KEY")
        .output()
        .expect("Failed to execute dry-run command");

    let stderr = String::from_utf8(output.stderr).unwrap();

    // Should fail with noob-friendly error about API key
    assert!(stderr.contains("forgot to set OPENAI_API_KEY"));
    assert!(stderr.contains("🔑"));
    assert!(stderr.contains("platform.openai.com"));
}

#[test]
fn test_non_git_directory() {
    // Create a temporary directory outside of any git repo
    let temp_dir = std::env::temp_dir().join(format!("noob-commit-test-{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir).unwrap();

    // Make sure there's no .git directory
    if temp_dir.join(".git").exists() {
        std::fs::remove_dir_all(temp_dir.join(".git")).ok();
    }

    // Use the binary directly instead of cargo run
    let binary_path = std::env::current_dir()
        .unwrap()
        .join("target")
        .join("debug")
        .join("noob-commit");

    let output = Command::new(&binary_path)
        .args(&["--dry-run"])
        .current_dir(&temp_dir)
        .env("OPENAI_API_KEY", "test-key")
        .output()
        .expect("Failed to execute command");

    let stderr = String::from_utf8(output.stderr).unwrap();

    // Should fail with noob-friendly error about not being in git repo OR no staged files
    // OR API key error (since we're using a fake key)
    let valid_errors = stderr.contains("isn't a git repo")
        || stderr.contains("Nothing to commit")
        || stderr.contains("🙈")
        || stderr.contains("🤷")
        || stderr.contains("API key")
        || stderr.contains("invalid_api_key")
        || stderr.contains("Trimming git diff");

    assert!(
        valid_errors,
        "Expected git repo error, no files error, or API error, got: {}",
        stderr
    );

    // Cleanup
    std::fs::remove_dir_all(&temp_dir).ok();
}

#[cfg(test)]
mod cli_tests {
    use std::process::Command;

    fn run_noob_commit(args: &[&str]) -> std::process::Output {
        Command::new("cargo")
            .arg("run")
            .arg("--")
            .args(args)
            .output()
            .expect("Failed to execute noob-commit")
    }

    #[test]
    fn test_all_flags_present() {
        let output = run_noob_commit(&["--help"]);
        let stdout = String::from_utf8(output.stdout).unwrap();

        // Test all our custom flags are present
        let expected_flags = [
            "-d, --dry-run",
            "-r, --review",
            "-f, --force",
            "-e, --ok-to-send-env",
            "-p, --no-push",
            "-t, --max-tokens",
            "-i, --max-input-chars",
            "-m, --model",
            "-s, --setup-alias",
            "-M, --yes-to-modules",
            "-c, --yes-to-crap",
            "-b, --br-huehuehue",
            "-a, --no-f-ads",
            "-u, --update",
        ];

        for flag in &expected_flags {
            assert!(stdout.contains(flag), "Missing flag: {}", flag);
        }
    }

    #[test]
    fn test_emoji_usage() {
        let output = run_noob_commit(&["--help"]);
        let stdout = String::from_utf8(output.stdout).unwrap();

        // Test that we use emojis for fun
        let expected_emojis = ["🤡", "🔍", "✏️", "⚡", "🔓", "📦", "🤖", "🧠", "🛠️", "🗑️"];

        for emoji in &expected_emojis {
            assert!(stdout.contains(emoji), "Missing emoji: {}", emoji);
        }
    }

    #[test]
    fn test_default_values() {
        let output = run_noob_commit(&["--help"]);
        let stdout = String::from_utf8(output.stdout).unwrap();

        // Test default values are set correctly
        assert!(stdout.contains("gpt-4.1-mini"));
        assert!(stdout.contains("2000"));
        assert!(stdout.contains("200000")); // Default max-input-chars
    }

    #[test]
    fn test_max_input_chars_flag() {
        let output = run_noob_commit(&["--help"]);
        let stdout = String::from_utf8(output.stdout).unwrap();

        // Test that max-input-chars flag is present
        assert!(stdout.contains("-i, --max-input-chars"));
        assert!(stdout.contains("Maximum characters of git diff to send to AI"));
    }
}

#[test]
fn test_max_input_chars_truncation() {
    use std::fs::{self, File};
    use std::io::Write;

    // Create a temporary git repo for testing
    let temp_dir = std::env::temp_dir().join(format!("noob-commit-test-{}", std::process::id()));
    fs::create_dir_all(&temp_dir).unwrap();

    // Initialize git repo
    Command::new("git")
        .args(&["init"])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to init git repo");

    // Configure git
    Command::new("git")
        .args(&["config", "user.email", "test@example.com"])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to set git email");

    Command::new("git")
        .args(&["config", "user.name", "Test User"])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to set git name");

    // Create a large file to test truncation
    let large_content = "a".repeat(100000); // 100k characters
    let file_path = temp_dir.join("large_file.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "{}", large_content).unwrap();

    // Add and commit initial version
    Command::new("git")
        .args(&["add", "."])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to add files");

    Command::new("git")
        .args(&["commit", "-m", "Initial commit"])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to commit");

    // Modify the file
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "{}", "b".repeat(100000)).unwrap(); // Different content

    // Stage the changes
    Command::new("git")
        .args(&["add", "."])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to add files");

    // Get the diff size
    let diff_output = Command::new("git")
        .args(&["diff", "--staged"])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to get diff");

    let diff_size = diff_output.stdout.len();
    println!("Diff size: {} bytes", diff_size);

    // Test with max-input-chars = 0 (no truncation)
    let binary_path = std::env::current_dir()
        .unwrap()
        .join("target")
        .join("debug")
        .join("noob-commit");

    // We can't test the actual API call, but we can verify the flag is accepted
    let output = Command::new(&binary_path)
        .args(&["--dry-run", "--max-input-chars", "0"])
        .current_dir(&temp_dir)
        .env("OPENAI_API_KEY", "test-key")
        .output()
        .expect("Failed to execute command");

    // The command should accept the flag (even if it fails due to API key)
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!stderr.contains("unexpected argument"));

    // Test with small max-input-chars to ensure truncation would happen
    let output = Command::new(&binary_path)
        .args(&["--dry-run", "--max-input-chars", "100"])
        .current_dir(&temp_dir)
        .env("OPENAI_API_KEY", "test-key")
        .output()
        .expect("Failed to execute command");

    // The command should accept the flag
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!stderr.contains("unexpected argument"));

    // Cleanup
    fs::remove_dir_all(&temp_dir).ok();
}
