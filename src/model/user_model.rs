use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    id: String,
    password: String,
    email: Option<String>,
}

impl User {
    pub fn new(id: String, password: String, email: Option<String>) -> Self {
        User {
            id,
            password,
            email,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginForm {
    pub id: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignupForm {
    pub id: String,
    pub password: String,
    pub email: String,
}
