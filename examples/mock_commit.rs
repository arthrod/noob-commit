/// Mock commit example for testing purposes
/// 
/// This example shows how to create mock commits for testing
/// without actually making API calls
/// 
/// Run with: cargo run --example mock_commit

use noob_commit::Commit;

fn main() {
    // Example commit messages for different scenarios
    let commit_examples = vec![
        Commit::new(
            "feat: add user authentication system".to_string(),
            "Implemented OAuth2 authentication with support for Google and GitHub providers. Added JWT token generation and validation for secure session management.".to_string(),
        ),
        Commit::new(
            "fix: resolve memory leak in cache handler".to_string(),
            "Fixed memory leak caused by unreleased references in the LRU cache implementation. Added proper cleanup in the destructor and improved memory management.".to_string(),
        ),
        Commit::new(
            "docs: update API documentation with new endpoints".to_string(),
            "Added comprehensive documentation for the new REST API endpoints including request/response examples, authentication requirements, and error codes.".to_string(),
        ),
        Commit::new(
            "refactor: improve error handling in database module".to_string(),
            "Refactored database error handling to use Result types consistently. Added custom error types for better error propagation and debugging.".to_string(),
        ),
        Commit::new(
            "test: add unit tests for payment processing".to_string(),
            "Added comprehensive unit tests for payment processing module covering edge cases, error scenarios, and successful payment flows.".to_string(),
        ),
    ];

    println!("Example commit messages:");
    println!("========================");
    
    for (i, commit) in commit_examples.iter().enumerate() {
        println!("\n{}. Title: {}", i + 1, commit.title);
        println!("   Description: {}", commit.description);
    }
    
    println!("\nThese are examples of well-formatted commit messages that follow conventional commit standards.");
    println!("When using noob-commit, the AI will generate similar messages based on your staged changes.");
}