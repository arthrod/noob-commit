use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
pub struct Commit {
    /// The title of the commit.
    pub title: String,
    /// An exhaustive description of the changes.
    pub description: String,
}

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
pub struct CommitAdvice {
    /// Friendly message to the noob developer.
    pub message: String,
    /// The actual commit information.
    pub commit: Commit,
}

impl ToString for Commit {
    fn to_string(&self) -> String {
        format!("{}\n\n{}", self.title, self.description)
    }
}

impl ToString for CommitAdvice {
    fn to_string(&self) -> String {
        format!("{}\n\n{}", self.message, self.commit.to_string())
    }
}

impl Commit {
    pub fn new(title: String, description: String) -> Self {
        Self { title, description }
    }
}

impl CommitAdvice {
    pub fn new(message: String, commit: Commit) -> Self {
        Self { message, commit }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit_creation() {
        let commit = Commit::new(
            "Add noob-friendly features".to_string(),
            "Added self-deprecating humor and alias setup functionality for developers who are great at coding but terrible at git.".to_string(),
        );

        assert_eq!(commit.title, "Add noob-friendly features");
        assert!(commit.description.contains("self-deprecating"));
    }

    #[test]
    fn test_commit_to_string() {
        let commit = Commit::new("Fix stuff".to_string(), "idk it works now".to_string());

        let result = commit.to_string();
        assert_eq!(result, "Fix stuff\n\nidk it works now");
    }

    #[test]
    fn test_commit_with_empty_description() {
        let commit = Commit::new("Update README".to_string(), "".to_string());

        let result = commit.to_string();
        assert_eq!(result, "Update README\n\n");
    }

    #[test]
    fn test_commit_with_multiline_description() {
        let commit = Commit::new(
            "Refactor code".to_string(),
            "Line 1\nLine 2\nLine 3".to_string(),
        );

        let result = commit.to_string();
        assert_eq!(result, "Refactor code\n\nLine 1\nLine 2\nLine 3");
    }

    #[test]
    fn test_commit_serialization() {
        let commit = Commit::new("Test commit".to_string(), "This is a test".to_string());

        let json = serde_json::to_string(&commit).unwrap();
        assert!(json.contains("Test commit"));
        assert!(json.contains("This is a test"));
    }

    #[test]
    fn test_commit_deserialization() {
        let json = r#"{"title":"Test commit","description":"This is a test"}"#;
        let commit: Commit = serde_json::from_str(json).unwrap();

        assert_eq!(commit.title, "Test commit");
        assert_eq!(commit.description, "This is a test");
    }

    #[test]
    fn test_commit_advice_to_string() {
        let commit = Commit::new("Init".to_string(), "First commit".to_string());
        let advice = CommitAdvice::new("Be careful".to_string(), commit);
        let result = advice.to_string();
        assert!(result.contains("Be careful"));
        assert!(result.contains("Init"));
        assert!(result.contains("First commit"));
    }
}
