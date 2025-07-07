use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

/// Manages the blobs in our Dit version control system
pub struct BlobMgr {
    /// Represents the blobs directory, `.dit/blobs`
    root: PathBuf,
}

/// Constructors
impl BlobMgr {
    /// Constructs the object given the dit root path (`.dit`)
    pub fn from_dit_root<P: Into<PathBuf>>(dit_root: P) -> io::Result<Self> {
        let dit_root = dit_root.into();
        if !dit_root.is_dir() {
            panic!(
                "the specified path {} is not a directory",
                dit_root.display()
            )
        }
        
        let root = dit_root.join("blobs");
        if !root.is_dir() {
            std::fs::create_dir(&root)?;
        }

        Ok(Self {
            root,
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
        let mut temp_file = BufWriter::new(File::create(self.root.join(".temp"))?);
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
        let target_file = self.root.join(&hash);
        std::fs::rename(self.root.join(".temp"), target_file)?;
        Ok(hash)
    }

    /// Returns the blob content reader based on it's hash
    pub fn get_reader<S: Into<String>>(&self, hash: S) -> io::Result<BufReader<File>> {
        let path = self.root.join(hash.into());
        let reader = BufReader::new(File::open(path)?);
        Ok(reader)
    }
}
