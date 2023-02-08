
pub mod routes{
    use actix_web::Responder;
    pub async fn register() -> impl Responder{
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
}

