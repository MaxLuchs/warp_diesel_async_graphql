use crate::db::DB;

pub struct GraphqlContext {
    pub db: DB
}

impl GraphqlContext {
    pub fn new(db: DB) -> GraphqlContext {
        GraphqlContext { db }
    }
}

