// services/user_service.rs

use rusqlite::params;

use crate::model::user_model::User; // Import the User struct from models
use crate::utils::db;

pub struct UserService;

use crate::utils::utils::verify_password;

impl UserService {
    pub fn check_user(id: &str, password: &str) -> bool {
        let mut db_state = db::DATABASE_STATE.lock().unwrap();

        let query = "SELECT password FROM UserTbl WHERE user_id = ?1";
        let mut stmt = db_state
            .prepare(query)
            .expect("Failed to prepare statement");
        let user_row = stmt.query_map(params![id], |row| Ok(row.get::<_, String>(0)?));

        match user_row {
            Ok(result) => {
                for query_password_result in result {
                    match query_password_result {
                        Ok(hashed_password) => {
                            if verify_password(password, &hashed_password) {
                                return true;
                            }
                        }
                        Err(err) => eprintln!("Error while getting hashed password: {:?}", err),
                    }
                }
                false // Password not found or doesn't match
            }
            Err(err) => {
                eprintln!("Error while querying user: {:?}", err);
                false
            }
        }
    }
}
