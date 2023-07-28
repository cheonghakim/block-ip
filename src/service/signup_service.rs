use rusqlite::{params, Error};

use crate::model::user_model::User;
use crate::utils::db;

pub struct SignupService;

impl SignupService {
    pub fn signup(id: &str, password: &str, email: &str) -> Result<Option<User>, Error> {
        let mut db_state = db::DATABASE_STATE.lock().unwrap();
        let query = "INSERT INTO UserTbl (user_id, password, email) VALUES (?1, ?2, ?3)";
        let mut stmt = db_state.prepare(query)?;
        // execute 메서드를 사용하여 INSERT 쿼리 실행
        stmt.execute(params![id, password, email])?;

        // 사용자가 추가되었으므로 새로운 SELECT 쿼리를 실행하여 사용자 정보를 가져옴
        let query = "SELECT * FROM UserTbl WHERE user_id = ?1 AND password = ?2";
        let user_row = stmt.query_row(params![id, password], |row| {
            Ok(User::new(row.get(0)?, row.get(1)?, row.get(2)?))
        });

        // 사용자 정보를 Option<User> 형태로 반환
        match user_row {
            Ok(user) => Ok(Some(user)),
            Err(_) => Ok(None),
        }
    }
}
