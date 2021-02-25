use diesel::{Insertable, Queryable, Identifiable};
use async_graphql::*;
use crate::schema::user;
use diesel::prelude::*;

#[derive(Insertable, Queryable, Identifiable, SimpleObject, Debug, Clone)]
#[table_name = "user"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
}
