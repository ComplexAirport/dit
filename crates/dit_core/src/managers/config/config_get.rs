use crate::errors::{ConfigError, DitResult};
use crate::managers::config::ConfigMgr;
use crate::models::USER_NAME_CONFIG;

impl ConfigMgr {
    pub fn get_user(&self) -> Option<String> {
        self.require_user().ok()
    }

    pub fn get_user_name(&self) -> Option<String> {
        self.config.user_name.clone()
    }

    pub fn get_user_email(&self) -> Option<String> {
        self.config.user_email.clone()
    }

    /// If at least one of the username and user email is set, returns the formatted version.
    /// Otherwise, returns an error
    pub fn require_user(&self) -> DitResult<String> {
        match &self.config.user_name {
            Some(name) => match &self.config.user_email {
                Some(email) => Ok(format!("{name} <{email}>")),
                None => Ok(name.clone())
            }

            None => match &self.config.user_email {
                Some(email) => Ok(format!("<{email}>")),
                None => Err(ConfigError::ConfigNotFound(USER_NAME_CONFIG.to_string()).into())
            }
        }
    }
}

