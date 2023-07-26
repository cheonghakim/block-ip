use rusqlite::params;

use crate::model::user_model::User;
use crate::utils::db;

pub struct SignupService;

impl SignupService {
    pub fn signup(id: &str, password: &str, email: &str) -> Option<User> {
        let mut db_state = db::DATABASE_STATE.lock().unwrap();

        // 비동기로 쿼리를 실행합니다.
        match db_state.execute(
            "INSERT INTO UserTbl (user_id, password, email) VALUES (?1, ?2, ?3)",
            params![id, password, email],
        ) {
            Ok(_) => {
                let user = User::new(
                    id.to_string().clone(),
                    password.to_string().clone(),
                    Some(email.to_string().clone()),
                );
                Some(user)
            }
            Err(_) => None,
        }
    }
}
