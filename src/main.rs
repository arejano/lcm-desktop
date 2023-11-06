mod core;
pub mod models;
pub mod schema;

fn main() {
    println!("Hello, world!");

    let conn = &mut core::db::establish_connection();
    let user = core::db::create_user(conn);

    println!("{:?}", user.name);
}
