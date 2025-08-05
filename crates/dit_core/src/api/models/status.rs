use std::path::PathBuf;

/// Represents current staging status
#[derive(Debug, Clone)]
pub struct Status {
    staged_unchanged_files: Vec<PathBuf>,
    staged_modified_files: Vec<PathBuf>,
    staged_deleted_files: Vec<PathBuf>,
    untracked_files: Vec<PathBuf>,
}

impl Status {
    pub fn staged_files(&self) -> &Vec<PathBuf> { &self.staged_unchanged_files }
    pub fn modified_files(&self) -> &Vec<PathBuf> { &self.staged_modified_files }
    pub fn deleted_files(&self) -> &Vec<PathBuf> { &self.staged_deleted_files }
    pub fn untracked_files(&self) -> &Vec<PathBuf> { &self.untracked_files }
}


/// Helpers methods
impl Status {
    pub(crate) fn new() -> Self {
        Self {
            staged_unchanged_files: vec![],
            staged_modified_files: vec![],
            staged_deleted_files: vec![],
            untracked_files: vec![],
        }
    }

    pub(crate) fn add_staged_unchanged_file(&mut self, file: PathBuf) {
        self.staged_unchanged_files.push(file);
    }

    pub(crate) fn add_staged_modified_file(&mut self, file: PathBuf) {
        self.staged_modified_files.push(file);
    }

    pub(crate) fn add_staged_deleted_file(&mut self, file: PathBuf) {
        self.staged_deleted_files.push(file);
    }

    pub(crate) fn add_untracked_file(&mut self, file: PathBuf) {
        self.untracked_files.push(file);
    }
}