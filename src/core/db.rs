use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::{NewUser, User};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("db not found");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &mut SqliteConnection) -> User {
    use crate::schema::users;

    let new_user = NewUser {
        name: "user_test",
        email: "email_test",
        password: "password_test",
        active: true,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error create user")
}
