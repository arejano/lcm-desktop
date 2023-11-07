use std::error::Error;

use diesel::prelude::*;

use crate::models::{NewUser, User};

pub fn create_user(conn: &mut SqliteConnection, user: NewUser) -> User {
    use crate::schema::users;

    diesel::insert_into(users::table)
        .values(&user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error create user")
}

pub fn get_user(conn: &mut SqliteConnection, user: NewUser) -> User {
    use crate::schema::users;

    diesel::insert_into(users::table)
        .values(&user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error create user")
}

pub fn update_user(conn: &mut SqliteConnection) -> Result<Option<User>, Box<dyn Error>> {
    use crate::schema::users::dsl::users;
    let user = users
        .find(1)
        .select(User::as_select())
        .first(conn)
        .optional();

    // match user {
    //     Ok(Some(user)) => {println!("working")},
    //     Ok(None) => println!("unable to find")
    //     Err(_) => println!("errr")
    // }

    Ok(Some(user))
}

pub fn new_update_user(conn: &mut SqliteConnection) -> Result<User, Box<dyn Error>> {
    use crate::schema::users::dsl::users;

    let me = users.find(1).first::<User>(conn).expect("Erro");
}
