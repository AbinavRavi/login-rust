use actix_web::Responder;
use std::{fs, collections::btree_set::Union};
use std::io::BufWriter;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use model::{LoginInfo, SignupInfo};
use std::path::Path;
use crate::model;

fn check_path(filename: String) -> bool {
    let result = Path::new(&filename).exists();
    result
}

pub async fn register(email: String, password: String, name: String) -> impl Responder{
    let mut register: SignupInfo = SignupInfo{
        email: email,
        password: password,
        name: name
    };
    let serialized_user = serde_json::to_string(&register).unwrap();
    let var: bool = check_path("db.json".to_string());
    // if var {
    //     // write to the json file

    // }
    // else{
    //     return "DB doesnt exist"
    // }
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

