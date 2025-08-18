use crate::Dit;
use crate::errors::DitResult;

/// Set
impl Dit {
    pub fn config_set_user_name(&mut self, value: String) -> DitResult<()> {
        self.config_mgr()?.borrow_mut().set_user_name(value)
    }

    pub fn config_set_user_email(&mut self, value: String) -> DitResult<()> {
        self.config_mgr()?.borrow_mut().set_user_email(value)
    }
}

/// Get
impl Dit {
    pub fn config_get_user_name(&self) -> DitResult<Option<String>> {
        Ok(self.config_mgr()?.borrow().get_user_name())
    }

    pub fn config_get_user_email(&self) -> DitResult<Option<String>> {
        Ok(self.config_mgr()?.borrow().get_user_email())
    }

    pub fn config_get_user(&self) -> DitResult<Option<String>> {
        Ok(self.config_mgr()?.borrow().get_user())
    }
}