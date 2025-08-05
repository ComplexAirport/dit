//! This module provides the API to work with the Dit version control system
use crate::commit::CommitMgr;
use crate::stage::StageMgr;
use crate::branch::BranchMgr;
use crate::tree::TreeMgr;
use crate::blob::BlobMgr;
use crate::errors::DitResult;
use crate::Repo;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

/// Main API for working with the Dit version control system
pub struct Dit {
    pub(super) repo: Rc<Repo>,
    pub(super) blob_mgr: RefCell<BlobMgr>,
    pub(super) tree_mgr: RefCell<TreeMgr>,
    pub(super) commit_mgr: RefCell<CommitMgr>,
    pub(super) stage_mgr: RefCell<StageMgr>,
    pub(super) branch_mgr: RefCell<BranchMgr>,
}


/// Constructor
impl Dit {
    /// Constructs the object given the project path (inside which the `.dit` is located) \
    /// Constructs all the managers
    pub fn from<P: AsRef<Path>>(project_path: P) -> DitResult<Self> {
        let repo = Rc::new(Repo::init(project_path)?);

        let dit = Self {
            repo: repo.clone(),
            blob_mgr: RefCell::new(BlobMgr::from(repo.clone())),
            tree_mgr: RefCell::new(TreeMgr::from(repo.clone())),
            stage_mgr: RefCell::new(StageMgr::from(repo.clone())?),
            commit_mgr: RefCell::new(CommitMgr::from(repo.clone())),
            branch_mgr: RefCell::new(BranchMgr::from(repo)?),
        };

        Ok(dit)
    }
}

