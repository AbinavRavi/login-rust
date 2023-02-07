# login-rust
A login service backend coded in rust completely

## Workflow

![login-workflow.jpg](Login workflow with OTP)

## Methods

For a login API, there can be multiple endpoints to handle different aspects of the authentication process. Below are some common endpoints for a login API:

"/api/login" - This endpoint is used for the initial authentication step where the user submits their credentials.

"/api/verify" - This endpoint is used for verifying the 2FA code sent to the user.

"/api/logout" - This endpoint is used for logging out the user and invalidating the token.

"/api/refresh" - This endpoint is used for refreshing the token if it has expired.

"/api/forgot-password" - This endpoint is used for handling forgot password functionality, where the user requests to reset their password.

"/api/reset-password" - This endpoint is used for resetting the password after the user has requested it through the forgot password functionality.

These are some common endpoints for a login API, but the exact endpoints can vary based on the requirements of the specific application.