use git2::{BranchType, Commit, Error as GitError, Remote, Repository, ResetType};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct GitRemote {
    pub name: Option<String>,
    pub url: Option<String>,
    pub push_url: Option<String>,
}

fn some_or_empty(val: &Option<String>) -> &str {
    match val {
        Some(v) => v,
        None => "",
    }
}

impl fmt::Debug for GitRemote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = some_or_empty(&self.name);
        let url = some_or_empty(&self.url);
        let push_url = some_or_empty(&self.push_url);
        write!(f, "{{ name={}, url={}, push_url={} }}", name, url, push_url)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Contributor {
    pub name: Option<String>,
    pub email: Option<String>,
}

impl fmt::Display for Contributor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = some_or_empty(&self.name);
        let email = some_or_empty(&self.email);
        write!(f, "{{ name={}, email={} }}", name, email)
    }
}

#[derive(Serialize, Deserialize)]
pub struct GitCommit {
    pub id: String,
    pub tree_id: String,
    pub message: Option<String>,
    pub header: Option<String>,
    pub summary: Option<String>,
    pub timestamp: i64,
    pub author: Contributor,
    pub committer: Contributor,
}

impl From<Commit<'_>> for GitCommit {
    fn from(item: Commit) -> Self {
        let author = item.author();
        let committer = item.committer();
        let time = item.time();
        GitCommit {
            id: item.id().to_string(),
            tree_id: item.tree_id().to_string(),
            timestamp: time.seconds(),
            header: item.raw_header().map(String::from),
            message: item.message().map(String::from),
            summary: item.summary().map(String::from),
            author: Contributor {
                name: author.name().map(String::from),
                email: author.email().map(String::from),
            },
            committer: Contributor {
                name: committer.name().map(String::from),
                email: committer.email().map(String::from),
            },
        }
    }
}

impl fmt::Debug for GitCommit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header = some_or_empty(&self.header);
        let message = some_or_empty(&self.message);
        let summary = some_or_empty(&self.summary);
        let author = format!("{}", self.author);
        let committer = format!("{}", self.committer);

        write!(
            f,
            "{{ id={}, tree_id={}, timestamp={}, header={}, message={}, summary={}, author={}, committer={} }}",
            self.id, self.tree_id, self.timestamp, header, message, summary,
            author, committer
        )
    }
}

pub struct GitRepository {
    repo: Repository,
}

impl GitRepository {
    pub fn open(path: &str) -> Result<Self, GitError> {
        Ok(GitRepository {
            repo: Repository::open(&path)?,
        })
    }

    pub fn clone(remote: &str, path: &str) -> Result<Self, GitError> {
        Ok(GitRepository {
            repo: Repository::clone(&remote, &path)?,
        })
    }

    fn get_remote_names(&self) -> Result<Vec<String>, GitError> {
        let remotes: Vec<_> = self
            .repo
            .remotes()?
            .iter()
            .filter(|remote| remote.is_some())
            .map(|remote| String::from(remote.unwrap()))
            .collect();
        Ok(remotes)
    }
    fn find_remote(&self, name: &str) -> Result<Remote, GitError> {
        self.repo.find_remote(&name)
    }
    pub fn get_remotes(&self) -> Result<Vec<GitRemote>, GitError> {
        let remote_names = self.get_remote_names()?;
        let remotes: Vec<_> = remote_names
            .iter()
            .map(|remote| self.find_remote(remote))
            .filter(|remote| remote.is_ok())
            .map(|remote| {
                let remote = remote.unwrap();
                GitRemote {
                    name: remote.name().map(String::from),
                    url: remote.url().map(String::from),
                    push_url: remote.pushurl().map(String::from),
                }
            })
            .collect();
        Ok(remotes)
    }

    fn get_branches(&self, branch_type: BranchType) -> Result<Vec<String>, GitError> {
        let branches: Vec<_> = self
            .repo
            .branches(None)?
            .filter(|branch| branch.is_ok())
            .map(|branch| branch.unwrap())
            .filter(|branch| branch.1 == branch_type)
            .map(|branch| branch.0)
            .collect();
        let branch_names: Vec<_> = branches
            .iter()
            .map(|branch| branch.name())
            .filter(|name| name.is_ok())
            .map(|name| name.unwrap())
            .filter(|name| name.is_some())
            .map(|name| name.unwrap().to_string())
            .collect();
        Ok(branch_names)
    }

    pub fn get_local_branches(&self) -> Result<Vec<String>, GitError> {
        self.get_branches(BranchType::Local)
    }
    pub fn get_remote_branches(&self) -> Result<Vec<String>, GitError> {
        self.get_branches(BranchType::Remote)
    }

    fn _get_last_commit(&self, branch: &str) -> Result<Commit, GitError> {
        let branch = self.repo.find_branch(&branch, BranchType::Local)?;
        let reference = branch.get();
        let commit = reference.peel_to_commit()?;
        Ok(commit)
    }
    pub fn get_last_commit(&self, branch: &str) -> Result<GitCommit, GitError> {
        let commit = self._get_last_commit(&branch)?;
        Ok(GitCommit::from(commit))
    }

    pub fn checkout(&self, branch: &str) -> Result<(), GitError> {
        let obj = self
            .repo
            .revparse_single(&format!("refs/heads/{}", branch))?;
        self.repo.checkout_tree(&obj, None)
    }

    pub fn fetch(&self, remote: &str, branch: &str) -> Result<(), GitError> {
        let mut remote = self.repo.find_remote(&remote)?;
        remote.fetch(&[&branch], None, None)
    }

    pub fn pull(&self, remote: &str, branch: &str) -> Result<(), GitError> {
        self.fetch(&remote, &branch)?;
        let obj_id = self
            .repo
            .refname_to_id(&format!("refs/remotes/{}/{}", remote, branch))?;
        let obj = self.repo.find_object(obj_id, None)?;
        self.repo.reset(&obj, ResetType::Hard, None)
    }
}
