use crate::model;
use crate::model::User;
use crate::{connection::DB, model::user};
//use model::category;
use crate::error::ErrorT;
use crate::middleware;
use middleware::auth::{hash_password, verify_password, AuthenticatedUser};
use model::user::Input;
use mongodb::results::InsertOneResult;
use mongodb::results::{DeleteResult, UpdateResult};
use rocket::http::{Cookie, Status};
use rocket::Request;
pub use rocket::{http::CookieJar, http::Header, serde::json::Json, State};

#[get("/test")]
pub async fn _test(_auth: AuthenticatedUser) -> String {
    let header = hash_password(&"ope".to_string());
    header
}

#[get("/login", format = "json", data = "<data>")]
pub async fn login(
    state: &State<DB>,
    data: Json<user::AuthInfo>,
    cookies: &CookieJar<'_>,
) -> Json<Option<String>> {
    let auth_info_clone = data.clone();
    let folder = user::get_one(&Some(auth_info_clone.email), &state.db).await;
    match folder {
        Ok(t) => {
            //verify user with password and if valid add to cookie and return user's email
            if verify_password(&auth_info_clone.password, &t.password) {
                cookies.add_private(Cookie::new("email", t.email.to_string()));
                Json(Some(t.email))
            } else {
                //Return nothing if password doesn't match records
                Json(Some("Password does't match account".to_string()))
            }
        }
        //Return nothing if err
        Err(_) => Json(Some("No user found".to_string())),
    }
}

#[get("/user/cookie/<email>")]
pub async fn fetch_by_cookie(
    db: &State<DB>,
    email: String,
    cookies: &CookieJar<'_>,
) -> Result<Json<user::AuthInfo>, ErrorT> {
    let user_logged_in = cookies.get_private("email");
    match user_logged_in {
        Some(t) => {
            let user_logged_in_email = t.value().parse::<String>().unwrap();
            if email == user_logged_in_email {
                let fetch_user = user::get_one(&Some(email), &db.db).await;
                match fetch_user {
                    Ok(data) => Ok(Json(data)),
                    Err(e) => return Err(e),
                }
            } else {
                return Err(ErrorT {
                    status: Status::BadRequest,
                    message: "Invalid user data".to_string(),
                });
            }
        }
        None => {
            return Err(ErrorT {
                status: Status::NetworkAuthenticationRequired,
                message: "Please authorize again".to_string(),
            })
        }
    }
}

#[get("/user")]
pub async fn get(
    state: &State<DB>,
    _auth: AuthenticatedUser,
) -> Result<Json<Vec<user::User>>, ErrorT> {
    let folder = user::get(&state.db).await;
    match folder {
        Ok(data) => {
            println!("{:?}", &data[0]._id);
            Ok(Json(data))
        }
        Err(e) => return Err(e),
    }
}

#[get("/user/<ido>")]
pub async fn get_one(
    state: &State<DB>,
    _auth: AuthenticatedUser,
    ido: String
) -> Result<Json<Vec<user::User>>, ErrorT> {
    let folder = user::get(&state.db).await;
    match folder {
        Ok(data) => {
            println!("{:?}", &data[0]._id);
            Ok(Json(data))
        }
        Err(e) => return Err(e),
    }
}


#[post("/user", format = "json", data = "<input>")]
pub async fn new(input: Json<Input>, state: &State<DB>) -> Result<Json<InsertOneResult>, ErrorT> {
    let folder = user::post(&input, &state.db).await;
    match folder {
        Ok(t) => Ok(Json(t)),
        Err(e) => Err(e),
    }
}

#[delete("/user/<name>/delete")]
pub async fn delete(
    name: String,
    state: &State<DB>,
    _auth: AuthenticatedUser,
) -> Json<DeleteResult> {
    let user = user::delete(&name, &state.db).await.unwrap();
    Json(user)
}

#[put("/user/<email>", format = "json", data = "<input>")]
pub async fn edit(
    input: Json<User>,
    email: String,
    state: &State<DB>,
    _auth: AuthenticatedUser,
) -> Json<Option<UpdateResult>> {
    if _auth.email == email {
        let category = user::edit(&input, &email, &state.db).await.unwrap();
        Json(Some(category))
    } else {
        Json(None)
    }
}

#[post("/logout")]
pub async fn logout(cookies: &CookieJar<'_>) -> Json<String> {
    cookies.remove_private(Cookie::named("email"));
    Json("You've been logged out".to_string())
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}
