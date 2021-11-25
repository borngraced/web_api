#[macro_use]
extern crate rocket;
use dotenv;
mod connection;
mod model;
use connection::DB;
mod routes;
use routes::{category, user};
mod error;
mod middleware;

#[rocket::main]
async fn main() {
    let _ = dotenv::dotenv();
    let connect = DB::init().await.unwrap();
    rocket::build()
        .mount(
            "/api/v1/",
            routes![
                user::new,
                user::fetch_by_cookie,
                user::get,
                user::get_one,
                user::edit,
                user::delete,
                user::login,
                user::logout,
                category::add,
                category::get,
                category::get_one,
                category::put,
                category::delete
            ],
        )
        .register("/", catchers![user::not_found])
        .manage(connect)
        .launch()
        .await
        .unwrap();
}
