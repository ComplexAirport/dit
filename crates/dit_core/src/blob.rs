//! This module manages the blobs in dit version control system
//!
//! Blobs are files that store the contents of files included
//! in the repo. They have content hashes as their names, helping optimize space.
//! For example, if a file content doesn't change through multiple commits,
//! it will point to the same blob, avoiding unnecessary copying.
//! Files with the same content will also point to the same blob.
//!
//! Let's say we have a file called `main.py`, which is a simple python script with
//! content:
//! ```python
//! def main():
//!     print("Hello, world")
//!
//! if __name__ == '__main__':
//!     main()
//! ```
//!
//! When this file is committed to the repo the first time, the hash of the contents
//! will be calculated. A file, a "blob" will be created with this hash as its filename.
//! In our case, it's
//! ```plain
//! 22ae0256cb0b3d71e110ae7ff3c1f1086b847e5092b9b16976012310cce0b81f`
//! ```
//! This file can later be reused for the same file if the contents don't change
//! or other files with identical content. This way, we avoid unnecessary copying.

use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use crate::constants::BLOBS_ROOT;


/// Manages the blobs in our Dit version control system \
/// (see [`crate::blob`] for more detailed info)
pub struct BlobMgr {
    /// Represents the blobs directory, [`BLOBS_ROOT`]
    path: PathBuf,
}

/// Constructors
impl BlobMgr {
    /// Represents the maximum size of the buffer when reading/writing/hashing files \
    /// Larger value means more bytes loaded into the RAM during I/O
    const BUFFER_SIZE: usize = 8192;

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
            path: root,
        })
    }
}

/// API
impl BlobMgr {
    /// Adds a target file to the blobs and returns the hash
    pub fn create_blob<P: Into<PathBuf>>(&self, path: P) -> io::Result<String> {
        let path = path.into();

        // We will create one reader, one writer and one hasher.
        // The reader will read a fixed size of bytes from the source file
        // into a buffer, update the hasher using this buffer and
        // write it to a temporary file. When the final hash is calculated,
        // this temporary file will be renamed to that hash, creating the blob.
        let mut reader = BufReader::new(File::open(path)?);
        let temp_file_path = self.path.join(".temp");
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
        let target_file = self.path.join(&hash);

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
    pub fn get_blob<S: Into<String>>(&self, hash: S) -> io::Result<BufReader<File>> {
        let path = self.path.join(hash.into());
        let reader = BufReader::new(File::open(path)?);
        Ok(reader)
    }
}
