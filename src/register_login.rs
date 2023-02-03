use actix_web::Responder;

async fn register() -> impl Responder{
    "Registration Successful"
}

async fn login() -> impl Responder{
    "Login Successful"
}

