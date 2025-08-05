use crate::errors::{DitResult, StagingError};
use crate::helpers::{read_to_string, write_to_file};
use crate::managers::stage::StageMgr;
use crate::models::Stage;


/// Manage the stage file
impl StageMgr {
    /// Updates staged files stored in self based on the data in the [`STAGE_FILE`]
    ///
    /// [`STAGE_FILE`]: crate::project_structure::STAGE_FILE
    pub(super) fn load_stage_file(&mut self) -> DitResult<()> {
        let path = self.repo.stage_file();
        let serialized = read_to_string(path)?;

        let staged_files = if serialized.is_empty() {
            Stage::new()
        } else {
            serde_json::from_str(&serialized)
                .map_err(|_| StagingError::DeserializationError)?
        };

        self.stage = staged_files;

        Ok(())
    }

    /// Updates the data in the [`STAGE_FILE`] based on staged files stored in self
    ///
    /// [`STAGE_FILE`]: crate::project_structure::STAGE_FILE
    pub(super) fn update_stage_file(&self) -> DitResult<()> {
        let path = self.repo.stage_file();

        let serialized = serde_json::to_string_pretty(&self.stage)
            .map_err(|_| StagingError::SerializationError)?;

        write_to_file(path, serialized)?;

        Ok(())
    }
}

/// Getters
impl StageMgr {
    pub fn is_staged(&self) -> bool {
        !self.stage.files.is_empty()
    }

    pub fn get_stage(&self) -> &Stage {
        &self.stage
    }
}
