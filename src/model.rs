mod model;
use regex::Regex;

struct SignupInfo{
    email: String,
    password: String,
    name: String,
}

impl SignupInfo{
    fn validate(){
        let re = Regex::new(r"^\w+@[a-zA-Z_]+?\.[a-zA-Z]{2,3}$").unwrap();
        let email_id = SignupInfo.email;
        assert!(re.is_match(email_id));
    }
}

struct LoginInfo{
    email: String,
    password: String
}
