use axum::Json;
use regex::Regex;

pub fn validate_username(username: &str) -> bool {
    let username_regex = Regex::new(r"^[a-zA-Z0-9_-]{5,20}$").unwrap();
    username_regex.is_match(username)
}

pub fn validate_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
    email_regex.is_match(email)
}

pub fn validate_password(password: &str) -> bool {
    password.len() >= 8 && password.len() <= 50
}
