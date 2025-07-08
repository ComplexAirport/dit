use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use crate::commit::CommitMgr;
use crate::constants::{DIT_ROOT, STAGED_ROOT, STAGED_FILE, HEAD_FILE};
use crate::tree::{StagedFiles};

/// Main API for working with the Dit version control system
pub struct Dit {
    /// Represents the project directory, where the `.dit` is located
    project_path: PathBuf,

    /// Represents the `.dit` path, [`DIT_ROOT`]
    root: PathBuf,

    /// Represents the staged files root, [`STAGED_ROOT`]
    staged_root: PathBuf,

    /// Represents the staged file location, [`STAGED_FILE`]
    staged_file: PathBuf,

    /// Represents the head file location, [`HEAD_FILE`]
    head_file: PathBuf,

    /// Represents the commit manager
    commit_mgr: CommitMgr,

    /// Tree builder for staging changes
    staged_files: StagedFiles,

    /// Head hash
    head: Option<String>,
}


/// Constructors
impl Dit {
    /// Constructs the object given the project path (inside which the `.dit` is located) \
    /// Creates commit, tree and blob managers
    pub fn from_project<P: Into<PathBuf>>(project_path: P) -> io::Result<Self> {
        let project_path = project_path.into();
        let root= project_path.join(DIT_ROOT);

        if !root.exists() {
            std::fs::create_dir_all(&root)?;
        }

        let commit_mgr = CommitMgr::from_project(&project_path)?;

        let staged_root = project_path.join(STAGED_ROOT);
        let staged_file = project_path.join(STAGED_FILE);

        if !staged_root.exists() {
            std::fs::create_dir_all(&staged_root)?;
        }

        if !staged_file.exists() {
            std::fs::write(&staged_file, "")?;
        }
        let staged_files = Self::load_staged_files(staged_file.clone())?;

        let head_path = project_path.join(HEAD_FILE);
        let head_file = Self::load_head(&head_path)?;

        Ok(Self {
            project_path,
            root,
            staged_root,
            staged_file,
            head_file: head_path,
            commit_mgr,
            staged_files,
            head: head_file,
        })
    }
}


/// Manage commits
impl Dit {
    pub fn commit<S1, S2>(
        &mut self,
        author: S1,
        message: S2,
    ) -> io::Result<()>
    where S1: Into<String>, S2: Into<String>
    {
        let author = author.into();
        let message = message.into();
        let parent_commit_hash = Self::load_head(&self.head_file)?;
        let commit_hash = self.commit_mgr.create_commit(
            author, message, self.staged_files.clone(), parent_commit_hash)?;

        // change the head
        self.head = Some(commit_hash.clone());
        Self::store_head(&self.head_file, commit_hash)?;
        for file in &self.staged_files.files {
            std::fs::remove_file(file)?;
        }

        self.staged_files = StagedFiles::new();
        Self::store_staged_files(&self.staged_file, self.staged_files.clone())?;

        Ok(())
    }
}


/// Manage file staging/unstaging
impl Dit {
    const BUFFER_SIZE: usize = 8192;

    pub fn stage_file<P: Into<PathBuf>>(&mut self, file_path: P) -> io::Result<()> {
        let file_path = file_path.into();

        let mut reader = BufReader::new(File::open(&file_path)?);

        let write_to = self.staged_root.join(file_path.file_name().unwrap());
        let mut writer = BufWriter::new(File::create(&write_to)?);

        let mut buffer: [u8; Self::BUFFER_SIZE] = [0; Self::BUFFER_SIZE];

        loop {
            let n = reader.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            writer.write_all(&buffer[..n])?;
        }

        self.staged_files.add_file(&write_to)?;

        Self::store_staged_files(&self.staged_file, self.staged_files.clone())?;

        Ok(())
    }

    pub fn unstage_file<P: Into<PathBuf>>(&mut self, file_path: P) -> io::Result<()> {
        let file_path = file_path.into();

        self.staged_files.remove_file(&file_path);

        std::fs::remove_file(&file_path)?;

        Self::store_staged_files(&self.staged_file, self.staged_files.clone())?;

        Ok(())
    }
}


/// Private helper methods
impl Dit {
    fn load_head<P: Into<PathBuf>>(path: P) -> io::Result<Option<String>> {
        let path = path.into();
        let serialized = std::fs::read_to_string(&path)?;
        if serialized.is_empty() {
            Ok(None)
        } else {
            Ok(Some(serialized))
        }
    }

    fn store_head<P: Into<PathBuf>>(path: P, hash: String) -> io::Result<()> {
        let path = path.into();
        std::fs::write(path, hash)?;
        Ok(())
    }

    fn load_staged_files<P: Into<PathBuf>>(path: P) -> io::Result<StagedFiles> {
        let path = path.into();
        let serialized = std::fs::read_to_string(path)?;
        let staged_files = if serialized.is_empty() {
            StagedFiles::new()
        } else {
            serde_json::from_str(&serialized)?
        };

        Ok(staged_files)
    }

    fn store_staged_files<P: Into<PathBuf>>(path: P, staged_files: StagedFiles) -> io::Result<()> {
        let path = path.into();
        let serialized = serde_json::to_string_pretty(&staged_files)?;
        std::fs::write(path, serialized)?;
        Ok(())
    }
}
