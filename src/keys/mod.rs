//! Deploy keys interface
//! This [this document](https://developer.github.com/guides/managing-deploy-keys/)
//! for motivation and use
extern crate serde_json;

use self::super::{Github, Result};

pub struct Keys<'a> {
    github: &'a Github,
    owner: String,
    repo: String,
}

impl<'a> Keys<'a> {
    #[doc(hidden)]
    pub fn new<O, R>(github: &'a Github, owner: O, repo: R) -> Keys<'a>
    where
        O: Into<String>,
        R: Into<String>,
    {
        Keys {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/keys{}", self.owner, self.repo, more)
    }

    pub fn create(&self, key: &KeyOptions) -> Result<Key> {
        let data = serde_json::to_string::<KeyOptions>(key)?;
        self.github.post::<Key>(&self.path(""), data.as_bytes())
    }

    pub fn list(&self) -> Result<Vec<Key>> {
        self.github.get::<Vec<Key>>(&self.path(""))
    }

    pub fn get(&self, id: u64) -> Result<Key> {
        self.github.get::<Key>(&self.path(&format!("/{}", id)))
    }

    pub fn delete(&self, id: u64) -> Result<()> {
        self.github.delete(&self.path(&format!("/{}", id))).map(
            |_| (),
        )
    }
}

// representations

#[derive(Debug, Deserialize)]
pub struct Key {
    pub id: u64,
    pub key: String,
    pub title: String,
    pub verified: bool,
    pub created_at: String,
    pub read_only: bool,
}

#[derive(Debug, Serialize)]
pub struct KeyOptions {
    pub title: String,
    pub key: String,
    pub read_only: bool,
}
