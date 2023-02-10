use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SignupInfo{
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginInfo{
    pub email: String,
    pub password: String
}
