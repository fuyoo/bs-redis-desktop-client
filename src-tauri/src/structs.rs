#[derive(Debug, Clone)]
pub enum AuthType {
    Password(String),
    PublicKey(String),
    Unknown,
}

impl Default for AuthType {
    fn default() -> Self {
        AuthType::Unknown
    }
}

#[derive(Debug, Clone)]
pub struct SshJumpTaskInfo {
    pub id: String,
    pub auth: AuthType,
    pub user: String,
    pub host: String,
    pub port: usize,
}
