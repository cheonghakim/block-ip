// services/user_service.rs

use rusqlite::params;

use crate::model::user_model::User; // Import the User struct from models
use crate::utils::db;

pub struct UserService;

impl UserService {
    pub fn check_user(id: &str, password: &str) -> Option<User> {
        let mut db_state = db::DATABASE_STATE.lock().unwrap();

        match db_state.execute(
            "SELECT * FROM UserTbl WHERE user_id = ?1 AND password = ?2",
            params![id, password],
        ) {
            Ok(_) => {
                let user = User::new(id.to_string().clone(), password.to_string().clone(), None);
                Some(user)
            }
            Err(_) => None,
        }
    }
}
