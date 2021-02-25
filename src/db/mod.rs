use r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

pub struct DB {
    pub pool: Pool<ConnectionManager<PgConnection>>
}

impl DB {
    pub fn new(url: &String) -> eyre::Result<DB> {
        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool: Pool<ConnectionManager<PgConnection>> = r2d2::Pool::builder().max_size(5).build(manager)?.into();
        Ok(DB {
            pool
        })
    }
}