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

use crate::constants::BUFFER_SIZE;
use crate::dit_project::DitProject;
use crate::errors::{DitResult, BlobError};
use crate::helpers::{get_buf_reader, get_buf_writer, read_from_buf_reader, write_to_buf_writer};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::rc::Rc;

/// Manages the blobs in our Dit version control system \
/// (see [`crate::blob`] for more detailed info)
pub struct BlobMgr {
    /// Represents the blobs directory, [`BLOBS_ROOT`]
    project: Rc<DitProject>,
}

/// Constructors
impl BlobMgr {
    pub fn from(project: Rc<DitProject>) -> Self {
        Self { project }
    }
}

/// API
impl BlobMgr {
    /// Adds a target file to the blobs and returns the hash
    pub fn create_blob<P: Into<PathBuf>>(&self, path: P) -> DitResult<String> {
        let path = path.into();

        // We will create one reader, one writer and one hasher.
        // The reader will read a fixed size of bytes from the source file
        // into a buffer, update the hasher using this buffer and
        // write it to a temporary file. When the final hash is calculated,
        // this temporary file will be renamed to that hash, creating the blob.
        let mut reader = get_buf_reader(&path)?;

        let temp_file_path = self.project.blobs().join(".temp");
        let mut writer = get_buf_writer(&temp_file_path)?;

        let mut hasher = Sha256::new();
        let mut buffer = [0; BUFFER_SIZE];
        loop {
            let n = read_from_buf_reader(&mut reader, &mut buffer, &temp_file_path)?;

            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);

            write_to_buf_writer(&mut writer, &buffer[..n], &temp_file_path)?;
        }

        let hash = format!("{:x}", hasher.finalize());
        let target_file = self.project.blobs().join(&hash);

        if target_file.is_file() {
            // if the blob already exists, we just remove the newly created temp file
            std::fs::remove_file(&temp_file_path)
                .map_err(|_|
                    BlobError::TempFileDeletionError(temp_file_path.display().to_string()))?;

        } else {
            // if it does not exist, we create it by renaming the newly created temp file
            std::fs::rename(&temp_file_path, &target_file)
                .map_err(|_| BlobError::TempFileRenameError(
                    temp_file_path.display().to_string(), target_file.display().to_string()))?;
        }

        Ok(hash)
    }

    /// Returns the blob content reader based on it's hash
    pub fn get_blob_reader<S: Into<String>>(&self, hash: S) -> DitResult<BufReader<File>> {
        let path = self.project.blobs().join(hash.into());
        let reader = get_buf_reader(&path)?;
        Ok(reader)
    }
}
