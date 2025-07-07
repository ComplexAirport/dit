use crate::constants::BLOBS_ROOT;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

/// Manages the blobs in our Dit version control system
pub struct BlobMgr {
    /// Represents the blobs directory, `.dit/blobs`
    root_path: PathBuf,
}

/// Constructors
impl BlobMgr {
    /// Constructs the object given the project path (inside which the `.dit` is located)
    pub fn from_project<P: Into<PathBuf>>(project_path: P) -> io::Result<Self> {
        let project_path = project_path.into();
        if !project_path.is_dir() {
            panic!(
                "the specified path {} is not a directory",
                project_path.display()
            )
        }
        
        let root = project_path.join(BLOBS_ROOT);
        if !root.is_dir() {
            std::fs::create_dir_all(&root)?;
        }

        Ok(Self {
            root_path: root,
        })
    }
}

/// Main methods
impl BlobMgr {
    const BUFFER_SIZE: usize = 8192;

    /// Adds a target file to the blobs and returns the hash \
    pub fn add_file<P: Into<PathBuf>>(&self, path: P) -> io::Result<String> {
        let path = path.into();

        let mut reader = BufReader::new(File::open(path)?);
        let temp_file_path = self.root_path.join(".temp");
        let mut temp_file = BufWriter::new(File::create(&temp_file_path)?);
        let mut hasher = Sha256::new();

        let mut buffer = [0; Self::BUFFER_SIZE];
        loop {
            let n = reader.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
            temp_file.write_all(&buffer[..n])?;
        }
        
        let hash = format!("{:x}", hasher.finalize());
        let target_file = self.root_path.join(&hash);

        if target_file.is_file() {
            // if the blob already exists, we just remove the newly created temp file
            std::fs::remove_file(&temp_file_path)?;
        } else {
            // if it does not exist, we create it by renaming the newly created temp file
            std::fs::rename(&temp_file_path, target_file)?;
        }

        Ok(hash)
    }

    /// Returns the blob content reader based on it's hash
    pub fn get_reader<S: Into<String>>(&self, hash: S) -> io::Result<BufReader<File>> {
        let path = self.root_path.join(hash.into());
        let reader = BufReader::new(File::open(path)?);
        Ok(reader)
    }
}
