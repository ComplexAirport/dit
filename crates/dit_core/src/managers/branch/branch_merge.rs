use crate::managers::branch::BranchMgr;
use crate::managers::commit::CommitMgr;
use crate::errors::{BranchError, DitResult};

/// Public
impl BranchMgr {
    /// Merges two branches
    pub fn merge_to<S: AsRef<str>>(
        &mut self,
        merge_to: S,
        commit_mgr: &CommitMgr,
    ) -> DitResult<()> {
        let merge_to = merge_to.as_ref();
        let merge_from = self.curr_branch.as_ref().cloned()
            .ok_or_else(|| BranchError::CannotMergeToDetachedHead(merge_to.to_string()))?;
        self.merge_branches(merge_from, merge_to, commit_mgr)
    }
}


/// Private
impl BranchMgr {
    /// Tries to merge to branches
    pub(super) fn merge_branches<S1, S2>(
        &mut self,
        from: S1,
        to: S2,
        commit_mgr: &CommitMgr,
    ) -> DitResult<()>
    where S1: Into<String>, S2: Into<String> {
        let from = from.into();
        let to = to.into();

        // A -> B -> C -> D -> E -> F
        //      ^ BRANCH1           ^ BRANCH2

        // Case 1: Merge BRANCH1 into BRANCH2
        // do nothing, BRANCH2 is already up to date
        if commit_mgr.is_ancestor(&from, &to)? {
            Ok(())
        }
        // Case 2: Merge BRANCH2 into BRANCH1
        // In this case, simply move the BRANCH1 pointer to point to BRANCH2 head
        // A -> B -> C -> D -> E -> F
        //                          ^ BRANCH1, BRANCH2
        else if commit_mgr.is_ancestor(&to, &from)? {
            let merge_from_commit = self.get_branch_head(&from)?
                .unwrap_or_else(String::new);
            self.set_branch_head(&to, merge_from_commit)?;
            Ok(())
        }

        else {
            Err(BranchError::MergeNotSupported.into())  // todo
        }
    }
}