use crate::graphql::graphql_context::GraphqlContext;
use crate::models::user_model::User;
use crate::schema;
use diesel::{QueryDsl, RunQueryDsl, BoolExpressionMethods};
use async_graphql::{Result, Object, InputObject, Context, Schema, EmptyMutation, EmptySubscription};
use crate::diesel::ExpressionMethods;

pub struct Query {}

#[derive(Clone, Debug, InputObject)]
pub struct UserFilterInput {
    name: Option<String>,
    age: Option<i32>,
}

#[Object]
impl Query {
    pub async fn user(&self, context: &Context<'_>, id: i32) -> Result<Option<User>> {
        let con = context.data::<GraphqlContext>().map(|ctx| ctx.db.pool.get().unwrap())?;
        Ok(schema::user::table.find(id).first(&con).ok())
    }

    pub async fn users(&self, context: &Context<'_>, filter_input: Option<UserFilterInput>) -> Result<Vec<User>> {
        let con = context.data::<GraphqlContext>().map(|ctx| ctx.db.pool.get().unwrap())?;
        if let Some(UserFilterInput { age, name }) = filter_input {
            use schema::user;
            let result = match (age, name) {
                (None, Some(n)) => {
                    user::table.filter(user::name.eq(n)).get_results(&con)?
                }
                (Some(a), None) => {
                    user::table.filter(user::age.eq(a)).get_results(&con)?
                }
                (Some(a), Some(n)) => {
                    schema::user::table.filter(user::age.eq(a).and(user::name.eq(n))).get_results(&con)?
                }
                (None, None) => {
                    schema::user::table.get_results(&con)?
                }
            };
            return Ok(result);
        }
        return schema::user::table.load(&con).map_err(|error| error.into());
    }
}

pub type MySchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn schema(context: GraphqlContext) -> MySchema {
    async_graphql::Schema::build(
        Query {},
        EmptyMutation::default(),
        EmptySubscription::default(),
    ).data(context).finish()
}