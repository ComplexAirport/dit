use crate::errors::{DitResult, FsError, OtherError};
use crate::helpers::path_to_string;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::process;

/// Creates a temp file (ensuring it has a unique name which doesn't
/// already exist) and returns it's path
pub fn create_temp_file<P: AsRef<Path>>(dest_dir: P)
    -> DitResult<(File, PathBuf)>
{
    let dest_dir = dest_dir.as_ref();
    let pid = process::id();
    let mut attempts = 0;

    loop {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| OtherError::TimeWentBackwardsError)?
            .as_nanos();

        let filename = format!("temp_{pid}_{nanos}_{attempts}");
        let path = dest_dir.join(filename);

        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path);

        match file {
            Ok(file) => return Ok((file, path)),

            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                attempts += 1;
                continue;
            }

            Err(_) =>
                return Err(FsError::FileCreateError(path_to_string(path)).into())
        }
    }
}
