
pub mod routes{
    use actix_web::Responder;
    pub async fn register() -> impl Responder{
        "Registration Successful"
    }

    pub async fn login() -> impl Responder{
        "Login Successful"
    }
}

