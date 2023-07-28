use lazy_static::lazy_static;
use rusqlite::{Connection, OpenFlags, Result, Row, ToSql};
use std::fmt;
use std::sync::{Arc, Mutex};

pub struct DatabaseState {
    pub connection: Connection,
}

impl DatabaseState {
    pub fn new(connection: Connection) -> Self {
        DatabaseState { connection }
    }

    pub fn execute(&mut self, query: &str, params: &[&dyn ToSql]) -> Result<()> {
        self.connection.execute(query, params)?;
        Ok(())
    }

    pub fn prepare(&mut self, query: &str) -> Result<rusqlite::Statement> {
        self.connection.prepare(query)
    }
}

// lazy_static! 매크로를 사용하여 데이터베이스 연결을 전역적으로 공유
impl fmt::Debug for DatabaseState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DatabaseState {{ ... }}")
    }
}

lazy_static! {
    pub static ref DATABASE_STATE: Arc<Mutex<DatabaseState>> = {
        // 데이터베이스 파일 경로를 지정하여 데이터베이스에 연결합니다.
        let connection = Connection::open_with_flags("db/database.db", OpenFlags::SQLITE_OPEN_READ_WRITE).unwrap();
        Arc::new(Mutex::new(DatabaseState::new(connection)))
    };
}
