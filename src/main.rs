use crate::models::NewUser;

mod core;
pub mod models;
mod repositories;
pub mod schema;

fn main() {
    println!("Hello, world!");

    let new_user = NewUser {
        name: "user_new_test",
        email: "emddail_test",
        password: "password_test",
        active: true,
    };

    let conn = &mut core::db::establish_connection();

    let user = repositories::users_repository::create_user(conn, new_user);

    println!("{:?}", user.name);
}
