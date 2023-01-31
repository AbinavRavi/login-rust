

pub mod model{
    use regex::Regex;
    use std::io;

    pub struct SignupInfo{
        email: String,
        password: String,
        name: String,
    }

    impl SignupInfo{
        fn validate_email(self: &Self){
            let re = Regex::new(r"^\w+@[a-zA-Z_]+?\.[a-zA-Z]{2,3}$").unwrap();
            let email_id = &self.email;
            assert!(re.is_match(&email_id));
        }

        fn validate_password() -> io::Result<()>{
            Ok(())
        }
    }

    pub struct LoginInfo{
        email: String,
        password: String
    }

    impl LoginInfo{
        fn validate_email(self: &Self){
            let re = Regex::new(r"^\w+@[a-zA-Z_]+?\.[a-zA-Z]{2,3}$").unwrap();
            let email_id = &self.email;
            assert!(re.is_match(&email_id));
        }

        fn validate_password() -> io::Result<()>{
            Ok(())
        }
    }
}