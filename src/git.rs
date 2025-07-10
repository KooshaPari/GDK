use anyhow::{anyhow, Result};
use git2::{BranchType, Oid, Repository, ResetType, Signature};

pub struct GitOperations {
    repo: Repository,
}

impl GitOperations {
    pub fn new(repo_path: &str) -> Result<Self> {
        let repo = Repository::open(repo_path)
            .or_else(|_| Repository::init(repo_path))
            .map_err(|e| anyhow!("Failed to open/create git repository: {}", e))?;

        Ok(Self { repo })
    }

    pub fn get_current_commit_hash(&self) -> Result<String> {
        let head = self.repo.head()?;
        let commit = head.peel_to_commit()?;
        Ok(commit.id().to_string())
    }

    pub fn create_commit(&self, message: &str) -> Result<String> {
        let mut index = self.repo.index()?;
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;

        let signature = Signature::now("GDK System", "gdk@system.local")?;
        let parent_commit = self.repo.head()?.peel_to_commit().ok();

        let parents = if let Some(ref parent) = parent_commit {
            vec![parent]
        } else {
            vec![]
        };

        let commit_id = self.repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parents,
        )?;

        Ok(commit_id.to_string())
    }

    pub fn create_branch(&self, branch_name: &str, commit_hash: Option<&str>) -> Result<()> {
        let commit = if let Some(hash) = commit_hash {
            let oid = Oid::from_str(hash)?;
            self.repo.find_commit(oid)?
        } else {
            self.repo.head()?.peel_to_commit()?
        };

        self.repo.branch(branch_name, &commit, false)?;
        Ok(())
    }

    pub fn switch_branch(&mut self, branch_name: &str) -> Result<()> {
        let branch_ref = format!("refs/heads/{branch_name}");
        self.repo.set_head(&branch_ref)?;
        self.repo
            .checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        Ok(())
    }

    pub fn hard_reset_to_commit(&mut self, commit_hash: &str) -> Result<()> {
        let oid = Oid::from_str(commit_hash)?;
        let commit = self.repo.find_commit(oid)?;
        self.repo.reset(commit.as_object(), ResetType::Hard, None)?;
        Ok(())
    }

    pub fn get_changed_files_since_commit(&self, commit_hash: &str) -> Result<Vec<String>> {
        let oid = Oid::from_str(commit_hash)?;
        let commit = self.repo.find_commit(oid)?;
        let current_tree = self.repo.head()?.peel_to_tree()?;
        let commit_tree = commit.tree()?;

        let diff = self
            .repo
            .diff_tree_to_tree(Some(&commit_tree), Some(&current_tree), None)?;

        let mut files = Vec::new();
        diff.foreach(
            &mut |delta, _progress| {
                if let Some(path) = delta.new_file().path() {
                    if let Some(path_str) = path.to_str() {
                        files.push(path_str.to_string());
                    }
                }
                true
            },
            None,
            None,
            None,
        )?;

        Ok(files)
    }

    pub fn get_file_diff(&self, file_path: &str, commit_hash: &str) -> Result<String> {
        let oid = Oid::from_str(commit_hash)?;
        let commit = self.repo.find_commit(oid)?;
        let current_tree = self.repo.head()?.peel_to_tree()?;
        let commit_tree = commit.tree()?;

        let diff = self
            .repo
            .diff_tree_to_tree(Some(&commit_tree), Some(&current_tree), None)?;

        let mut diff_content = String::new();
        let mut found_file = false;

        diff.foreach(
            &mut |delta, _progress| {
                if let Some(path) = delta.new_file().path() {
                    if path.to_str() == Some(file_path) {
                        diff_content = format!("File: {file_path}\nStatus: Modified\n");
                        found_file = true;
                    }
                }
                true
            },
            None,
            None,
            None,
        )?;

        if found_file {
            // Get line-by-line diff
            diff.foreach(
                &mut |_delta, _progress| true,
                None,
                None,
                Some(&mut |_delta, _hunk, line| {
                    match line.origin() {
                        '+' => {
                            let content = std::str::from_utf8(line.content()).unwrap_or("");
                            diff_content.push_str(&format!("+{content}"));
                        }
                        '-' => {
                            let content = std::str::from_utf8(line.content()).unwrap_or("");
                            diff_content.push_str(&format!("-{content}"));
                        }
                        _ => {
                            let content = std::str::from_utf8(line.content()).unwrap_or("");
                            diff_content.push_str(&format!(" {content}"));
                        }
                    }
                    true
                }),
            )?;
        }

        Ok(diff_content)
    }

    pub fn list_branches(&self) -> Result<Vec<String>> {
        let mut branches = Vec::new();
        let branch_iter = self.repo.branches(Some(BranchType::Local))?;

        for branch_result in branch_iter {
            let (branch, _branch_type) = branch_result?;
            if let Some(name) = branch.name()? {
                branches.push(name.to_string());
            }
        }

        Ok(branches)
    }

    pub fn get_commit_message(&self, commit_hash: &str) -> Result<String> {
        let oid = Oid::from_str(commit_hash)?;
        let commit = self.repo.find_commit(oid)?;
        Ok(commit.message().unwrap_or("").to_string())
    }

    pub fn get_commit_parents(&self, commit_hash: &str) -> Result<Vec<String>> {
        let oid = Oid::from_str(commit_hash)?;
        let commit = self.repo.find_commit(oid)?;

        let mut parents = Vec::new();
        for i in 0..commit.parent_count() {
            if let Ok(parent) = commit.parent(i) {
                parents.push(parent.id().to_string());
            }
        }

        Ok(parents)
    }

    pub fn get_repository(&self) -> &Repository {
        &self.repo
    }
}
