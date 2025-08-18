use crate::errors::DitResult;
use crate::managers::config::ConfigMgr;

impl ConfigMgr {
    pub fn set_user_name(&mut self, user_name: String) -> DitResult<()> {
        self.config.user_name = Some(user_name);
        self.store()
    }

    pub fn set_user_email(&mut self, user_email: String) -> DitResult<()> {
        self.config.user_email = Some(user_email);
        self.store()
    }
}
