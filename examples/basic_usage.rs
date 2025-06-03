/// Basic usage example for noob-commit
/// 
/// This example demonstrates how to use noob-commit programmatically
/// 
/// Run with: cargo run --example basic_usage

use noob_commit::Commit;
use std::env;

fn main() {
    // Example of creating a commit struct
    let commit = Commit::new(
        "feat: add basic usage example".to_string(),
        "This example demonstrates how to use noob-commit programmatically and shows the basic structure of commit messages.".to_string(),
    );

    println!("Example commit title: {}", commit.title);
    println!("Example commit description: {}", commit.description);
    println!("\nFull commit message:\n{}", commit.to_string());

    // Example of checking for API key
    match env::var("OPENAI_API_KEY") {
        Ok(_) => println!("✅ OpenAI API key is set"),
        Err(_) => println!("❌ OpenAI API key is not set. Set OPENAI_API_KEY environment variable"),
    }

    println!("\nTo use noob-commit:");
    println!("1. Stage your changes: git add .");
    println!("2. Run: noob-commit");
    println!("3. Or for dry run: noob-commit --dry-run");
    println!("4. To review before commit: noob-commit --review");
}