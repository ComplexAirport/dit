use crate::errors::BranchError;
use crate::managers::branch::BranchMgr;
use crate::errors::DitResult;
use crate::helpers::remove_file_if_exists;

impl BranchMgr {
    pub fn remove_branch<S: AsRef<str>>(&mut self, name: S) -> DitResult<()> {
        let name = name.as_ref();

        let current = self.get_current_branch();

        // Forbid removing the branch we are currently on
        if let Some(current) = current && name == current {
            return Err(BranchError::CannotRemoveBranch.into());
        }

        let (exists, path) = self.find_branch(name);

        if !exists {
            return Err(BranchError::BranchDoesNotExist(name.to_string()).into());
        }

        remove_file_if_exists(&path)?;

        Ok(())
    }
}
