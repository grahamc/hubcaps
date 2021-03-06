//! Review comments interface

use super::{Github, Result};
use users::User;

/// A structure for interfacing with a issue comments
pub struct ReviewComments<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
    number: u64,
}

impl<'a> ReviewComments<'a> {
    #[doc(hidden)]
    pub fn new<O, R>(github: &'a Github, owner: O, repo: R, number: u64) -> ReviewComments<'a>
    where
        O: Into<String>,
        R: Into<String>,
    {
        ReviewComments {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
            number: number,
        }
    }

    /// list pull requests
    pub fn list(&self) -> Result<Vec<ReviewComment>> {
        self.github.get::<Vec<ReviewComment>>(&format!(
            "/repos/{}/{}/pulls/{}/comments",
            self.owner,
            self.repo,
            self.number
        ))
    }
}

// representations

#[derive(Debug, Deserialize)]
pub struct ReviewComment {
    pub id: u64,
    pub url: String,
    pub diff_hunk: String,
    pub path: String,
    pub position: u64,
    pub original_position: u64,
    pub commit_id: String,
    pub original_commit_id: String,
    pub user: User,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
    pub html_url: String,
    pub pull_request_url: String,
}
