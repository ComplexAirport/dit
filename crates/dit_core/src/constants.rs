/// Represents the root dit directory, where all dit-related data
/// (blobs, trees, commits, etc.) is stored
pub const DIT_ROOT: &str = ".dit";

/// Represents a directory where the dit blobs are stored
pub const BLOBS_ROOT: &str = ".dit/blobs";

/// Represents a directory where the dit trees are stored
pub const TREES_ROOT: &str = ".dit/trees";

/// Represents a directory where the dit commits are stored
pub const COMMITS_ROOT: &str = ".dit/commits";

/// Represents a directory where the staged files are stored temporarily
pub const STAGED_ROOT: &str = ".dit/staged";

/// Represents a file where information about the staged files is stored
pub const STAGED_FILE: &str = ".dit/staged/staged";

/// Represents a file where the information about the head commit is stored \
/// this file contains the commit hash which usually points to the latest commit
pub const HEAD_FILE: &str = ".dit/head";
