/// Represents the root dit directory, where all dit-related data
/// (blobs, trees, commits, etc.) is stored
pub const DIT_ROOT: &str = ".dit";

/// Represents a directory where the dit blobs are stored
pub const BLOBS_ROOT: &str = ".dit/blobs";

/// Represents a directory where the dit trees are stored
pub const TREES_ROOT: &str = ".dit/trees";

/// Represents a directory where the dit commits are stored
pub const COMMITS_ROOT: &str = ".dit/commits";

/// Represents a directory where the dit branches are stored
pub const BRANCHES_ROOT: &str = ".dit/branches";

/// Represents a directory where the staged files are stored temporarily
pub const STAGE_ROOT: &str = ".dit/stage";

/// Represents a file where information about the staged files is stored
pub const STAGE_FILE: &str = ".dit/stage/staged_files";

/// Represents a file where the current branch is stored
pub const HEAD_FILE: &str = ".dit/head";
