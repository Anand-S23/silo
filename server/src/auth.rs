use axum::Json;
use regex::Regex;

use crate::models::BaseUserData;

pub fn validate_register_data(data: &BaseUserData) -> Vec<String> {
    let mut errors: Vec<String> = Vec::new();
    let username_regex = Regex::new(r"^[a-zA-Z0-9_-]{5,20}$").unwrap();
    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();

    if !username_regex.is_match(&data.username) {
        errors.push("Username must be 5 to 20 alphanumeric characters seperated by dashes or underscores".to_string())
    }

    if !email_regex.is_match(&data.email) {
        errors.push("Email is not valid".to_string())
    }

    if data.password.len() < 8 || data.password.len() > 50 {
        errors.push("Password must have at least 8 characters".to_string())
    }

    errors
}