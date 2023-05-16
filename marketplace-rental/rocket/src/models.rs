use crate::schema::users;
use crate::Db;
use diesel::prelude::*;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{Build, Rocket};
use rocket_sync_db_pools::diesel;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Insertable, Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[post("/", data = "<user>")]
pub async fn create(db: Db, user: Json<User>) -> Result<Created<Json<User>>> {
    let user_value = user.clone();
    db.run(move |conn| {
        diesel::insert_into(users::table)
            .values(&*user_value)
            .execute(conn)
    })
    .await?;

    Ok(Created::new("/").body(user))
}

#[get("/")]
pub async fn list(db: Db) -> Result<Json<Vec<i32>>> {
    let ids: Vec<i32> = db
        .run(move |conn| users::table.select(users::id).load(conn))
        .await?;

    Ok(Json(ids))
}

pub async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    Db::get_one(&rocket)
        .await
        .expect("database connection")
        .run(|conn| {
            conn.run_pending_migrations(MIGRATIONS)
                .expect("diesel migrations");
        })
        .await;

    rocket
}
