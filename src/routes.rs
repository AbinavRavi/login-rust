use actix_web::Responder;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use model::{LoginInfo, SignupInfo};

use crate::model;

pub async fn register(email: String, password: String, name: String) -> impl Responder{
    let register: SignupInfo = SignupInfo{
        email: email.to_owned(),
        password: password.to_owned(),
        name: name.to_owned()
    };
    
    "Registration Successful"
}

pub async fn login() -> impl Responder{
    "Login Successful"
}

pub async fn logout() -> impl Responder{
    "Logout successful"
}

pub async fn verify() -> impl Responder{
    "verification successful"
}

pub async fn refresh() -> impl Responder{
    "refresh token"
}

pub async fn forget_password() -> impl Responder{
    "Enter email to generate password"
}

pub async fn reset_password() -> impl Responder {
    "Patch request to update password"
}

