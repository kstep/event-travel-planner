use diesel::r2d2::ConnectionManager;
use diesel::sqlite::SqliteConnection;
use r2d2::Pool;
use crate::result::Result;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn create_pool(database_url: &str) -> Result<DbPool> {
    let manager = ConnectionManager::new(database_url);
    let pool = DbPool::builder().build(manager)?;
    Ok(pool)
}