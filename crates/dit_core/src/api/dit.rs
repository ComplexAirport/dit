//! This module provides the API to work with the Dit version control system
use crate::commit::CommitMgr;
use crate::index::IndexMgr;
use crate::branch::BranchMgr;
use crate::tree::TreeMgr;
use crate::blob::BlobMgr;
use crate::ignore::IgnoreMgr;
use crate::errors::DitResult;
use crate::Repo;
use once_cell::unsync::OnceCell;
use std::cell::RefCell;
use std::path::Path;
use std::sync::Arc;

/// Main API for working with the Dit version control system
pub struct Dit {
    pub(super) repo: Arc<Repo>,
    blob_mgr: OnceCell<RefCell<BlobMgr>>,
    tree_mgr: OnceCell<RefCell<TreeMgr>>,
    commit_mgr: OnceCell<RefCell<CommitMgr>>,
    index_mgr: OnceCell<RefCell<IndexMgr>>,
    branch_mgr: OnceCell<RefCell<BranchMgr>>,
    ignore_mgr: OnceCell<RefCell<IgnoreMgr>>
}


/// Constructor
impl Dit {
    /// Constructs the object given the project path (inside which the `.dit` is located) \
    /// Constructs all the managers
    pub fn from<P: AsRef<Path>>(project_path: P) -> DitResult<Self> {
        let repo = Arc::new(Repo::init(project_path)?);

        let dit = Self {
            repo: repo.clone(),
            blob_mgr: OnceCell::new(),
            tree_mgr: OnceCell::new(),
            index_mgr: OnceCell::new(),
            commit_mgr: OnceCell::new(),
            branch_mgr: OnceCell::new(),
            ignore_mgr: OnceCell::new(),
        };

        Ok(dit)
    }
}

/// Manager getters
impl Dit {
    /// Returns the blob manager
    pub fn blob_mgr(&self) -> &RefCell<BlobMgr> {
        self.blob_mgr.get_or_init(|| RefCell::new(BlobMgr::from(self.repo.clone())))
    }

    /// Returns the tree manager
    pub fn tree_mgr(&self) -> &RefCell<TreeMgr> {
        self.tree_mgr.get_or_init(|| RefCell::new(TreeMgr::from(self.repo.clone())))
    }

    /// Returns the index manager
    pub fn index_mgr(&self) -> DitResult<&RefCell<IndexMgr>> {
        self.index_mgr.get_or_try_init(|| {
            Ok(RefCell::new(IndexMgr::from(self.repo.clone())?))
        })
    }

    /// Returns the commit manager
    pub fn commit_mgr(&self) -> &RefCell<CommitMgr> {
        self.commit_mgr.get_or_init(|| RefCell::new(CommitMgr::from(self.repo.clone())))
    }

    /// Returns the branch manager
    pub fn branch_mgr(&self) -> DitResult<&RefCell<BranchMgr>> {
        self.branch_mgr.get_or_try_init(|| {
            Ok(RefCell::new(BranchMgr::from(self.repo.clone())?))
        })
    }

    /// Returns the ignore manager
    pub fn ignore_mgr(&self) -> DitResult<&RefCell<IgnoreMgr>> {
        self.ignore_mgr.get_or_try_init(|| {
            Ok(RefCell::new(IgnoreMgr::from(self.repo.clone())?))
        })
    }
}
