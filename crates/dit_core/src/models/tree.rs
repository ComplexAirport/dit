use crate::models::Index;
use crate::helpers::path_to_string;
use crate::helpers::{read_to_string, write_to_file};
use crate::errors::{DitResult, TreeError};
use crate::impl_read_write_model;
use std::path::Path;
use serde::{Deserialize, Serialize};

/// Represents a tree model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    /// Maps the relative file paths to corresponding blob hashes
    pub index: Index,

    /// Represents the tree hash
    #[serde(skip)]
    pub hash: String,
}

impl_read_write_model!(Tree, TreeError);
