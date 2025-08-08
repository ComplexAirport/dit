use std::path::PathBuf;

/// Represents current staging status
#[derive(Debug, Clone, Default)]
pub struct Status {
    pub tracked_creations: Vec<PathBuf>,
    pub tracked_modifications: Vec<PathBuf>,
    pub tracked_deletions: Vec<PathBuf>,

    pub unstaged_modifications: Vec<PathBuf>,
    pub unstaged_deletions: Vec<PathBuf>,

    pub untracked_files: Vec<PathBuf>,
    pub unchanged_files: Vec<PathBuf>,
}

/// Getters
impl Status {
    pub fn new() -> Self { Self::default() }

    pub fn get_tracked(&self, change_type: ChangeType) -> &Vec<PathBuf> {
        match change_type {
            ChangeType::Modified => &self.tracked_modifications,
            ChangeType::Deleted => &self.tracked_deletions,
            ChangeType::New => &self.tracked_creations,
        }
    }

    pub fn get_unstaged(&self, change_type: ChangeType) -> &Vec<PathBuf> {
        match change_type {
            ChangeType::Modified => &self.unstaged_modifications,
            ChangeType::Deleted => &self.unstaged_deletions,
            ChangeType::New => &self.untracked_files, // NOTE: use get_untracked() instead of this
        }
    }

    pub fn get_unchanged(&self) -> &Vec<PathBuf> {
        &self.unchanged_files
    }

    pub fn get_untracked(&self) -> &Vec<PathBuf> {
        &self.untracked_files
    }

    /// Checks if there are any tracked changes
    pub fn has_any_tracked(&self) -> bool {
        !self.tracked_creations.is_empty() || !self.tracked_modifications.is_empty() || !self.tracked_deletions.is_empty()
    }

    /// Checks if there are any unstaged changes
    pub fn has_any_unstaged(&self) -> bool {
        !self.unstaged_modifications.is_empty() || !self.unstaged_deletions.is_empty()
    }

    /// Checks if there are any untracked files
    pub fn has_any_untracked(&self) -> bool {
        !self.untracked_files.is_empty()
    }

    /// Checks if there are any unchanged files
    pub fn has_any_unchanged(&self) -> bool {
        !self.unchanged_files.is_empty()
    }
}

/// Setters
impl Status {
    pub fn add_tracked(&mut self, rel_path: PathBuf, change_type: ChangeType) {
        match change_type {
            ChangeType::Modified => self.tracked_modifications.push(rel_path),
            ChangeType::Deleted => self.tracked_deletions.push(rel_path),
            ChangeType::New => self.tracked_creations.push(rel_path),
        }
    }

    pub fn add_untracked(&mut self, rel_path: PathBuf, change_type: ChangeType) {
        match change_type {
            ChangeType::Modified => self.unstaged_modifications.push(rel_path),
            ChangeType::Deleted => self.unstaged_deletions.push(rel_path),
            ChangeType::New => self.untracked_files.push(rel_path),
        }
    }

    pub fn add_unchanged(&mut self, rel_path: PathBuf) {
        self.unchanged_files.push(rel_path);
    }
}

#[derive(Debug, Clone)]
pub enum ChangeType {
    Modified,
    Deleted,
    New,
}
