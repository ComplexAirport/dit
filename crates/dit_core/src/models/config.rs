use serde::{Serialize, Deserialize};


/// Username config name
pub const USER_NAME_CONFIG: &str = "user.name";

/// User email config name
pub const USER_EMAIL_CONFIG: &str = "user.email";


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(rename = "user.name")]
    pub user_name: Option<String>,

    #[serde(rename = "user.email")]
    pub user_email: Option<String>,
}
