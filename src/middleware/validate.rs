use regex::Regex;

pub fn validate_email(email: &String) -> Result<String, String> {
    let r = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    let is_match = r.is_match(&email);
    let error_message = String::from("Email is invalid, try again please");
    if is_match {
        Ok(email.to_string())
    } else {
        return Err(error_message.into());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn email_validation_test() {
        let email = String::from("sami@gmail.com");
        let email2 = String::from("sami@gmail.com");
         assert_eq!(email2, validate_email(&email).unwrap())
    }
}
