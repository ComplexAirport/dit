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

use crate::repo::Repo;
use std::rc::Rc;

/// Manages the blobs in our Dit version control system \
/// (see [`crate::blob`] for more detailed info)
pub struct BlobMgr {
    /// Represents the blobs directory, [`BLOBS_ROOT`]
    pub(super) repo: Rc<Repo>,
}

/// Constructors
impl BlobMgr {
    pub fn from(project: Rc<Repo>) -> Self {
        Self { repo: project }
    }
}
