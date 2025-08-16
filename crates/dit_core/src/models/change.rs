use crate::models::FileFingerprint;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Change {
    New(NewFile),
    Modified(ModifiedFile),
    Deleted(DeletedFile),
    Unchanged(UnchangedFile),
    None, // if the file does not exist and did not exist
}
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NewFile {
    pub hash: String,
    pub fp: FileFingerprint,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ModifiedFile {
    pub old_hash: String,
    pub hash: String,
    pub old_fp: FileFingerprint,
    pub fp: FileFingerprint,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct UnchangedFile {
    pub hash: String,
    pub fp: FileFingerprint,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct DeletedFile {
    pub hash: String,
    pub fp: FileFingerprint,
}