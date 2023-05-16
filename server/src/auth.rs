use serde_json::{Map, Value};
use regex::Regex;

use crate::models::BaseUserData;

pub struct AuthValidation {
    pub len: i32,
    pub errors: Value
}

impl AuthValidation {
    pub fn validate_register_data(data: &BaseUserData) -> AuthValidation {
        let mut errors_container = Map::new();
        let mut errors: Vec<Value> = Vec::new();

        let username_regex = Regex::new(r"^[a-zA-Z0-9_-]{5,20}$").unwrap();
        let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();

        if !username_regex.is_match(&data.username) {
            let mut username_error = Map::new();
            username_error.insert("param".to_string(), Value::String("username".to_string()));
            username_error.insert(
                "msg".to_string(), 
                Value::String("Username must be 5 to 20 alphanumeric characters seperated by dashes or underscores".to_string())
            );

            errors.push(Value::Object(username_error));
        }

        if !email_regex.is_match(&data.email) {
            let mut email_error = Map::new();
            email_error.insert("param".to_string(), Value::String("email".to_string()));
            email_error.insert(
                "msg".to_string(), 
                Value::String("Email is not valid".to_string())
            );

            errors.push(Value::Object(email_error));
        }

        if data.password.len() < 8 || data.password.len() > 50 {
            let mut password_error = Map::new();
            password_error.insert("param".to_string(), Value::String("email".to_string()));
            password_error.insert(
                "msg".to_string(), 
                Value::String("Password must have at least 8 characters".to_string())
            );

            errors.push(Value::Object(password_error));
        }

        let errors_len = errors.len();
        errors_container.insert("errors".to_string(), Value::Array(errors));

        AuthValidation {
            len: errors_len as i32, 
            errors: Value::Object(errors_container)
        }
    }
}