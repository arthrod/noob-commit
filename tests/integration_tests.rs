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
}

#[test]
fn test_version_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .output()
        .expect("Failed to execute version command");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Noob Commit"));
    assert!(stdout.contains("0.5.0"));
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
    // (depending on the git setup, it might detect as non-repo or no files)
    let valid_errors = stderr.contains("isn't a git repo") || 
                      stderr.contains("Nothing to commit") ||
                      stderr.contains("🙈") || 
                      stderr.contains("🤷");
    
    assert!(valid_errors, "Expected git repo error or no files error, got: {}", stderr);
    
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
            "--dry-run",
            "--review", 
            "--force",
            "--ok-to-send-env",
            "--no-push",
            "--max-tokens",
            "--model",
            "--setup-alias",
            "--yes-to-modules",
            "--yes-to-crap"
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
    }
}