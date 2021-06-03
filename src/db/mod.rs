pub mod models;
pub mod schema;
pub mod user;

use std::env;

use diesel::pg::PgConnection;
//use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};

pub type DbConnection = PgConnection;
pub type DbPool = Pool<ConnectionManager<DbConnection>>;

pub fn init_pool() -> Result<DbPool, PoolError> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<DbConnection>::new(url);
    Pool::builder().build(manager)
}
