use crate::db::DB;
use async_graphql::*;
use std::sync::{Arc, Mutex};
use r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

pub struct GraphqlContext {
    pub db: DB
}

impl GraphqlContext {
    pub fn new(db: DB) -> GraphqlContext {
        GraphqlContext { db }
    }
}

