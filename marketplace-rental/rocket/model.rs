use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
}

impl NewUser<'_> {
    pub fn save(&self, conn: &PgConnection) -> QueryResult<User> {
        use crate::schema::users::dsl::*;

        diesel::insert_into(users).values(self).get_result(conn)
    }
}
