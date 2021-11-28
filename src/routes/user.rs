use crate::middleware::auth::{account_decoder, account_encoder};
use crate::model;
use crate::model::User;
use crate::{connection::DB, model::user};
//use model::category;
use crate::error::ErrorT;
use crate::middleware;
use middleware::auth::{verify_password, AuthenticatedUser};
use model::user::Input;
use mongodb::results::InsertOneResult;
use mongodb::results::{DeleteResult, UpdateResult};
use rocket::http::{Cookie, Status};
use rocket::Request;
pub use rocket::{http::CookieJar, http::Header, serde::json::Json, State};

#[get("/login", format = "json", data = "<data>")]
pub async fn login(
    state: &State<DB>,
    data: Json<user::AuthInfo>,
    cookies: &CookieJar<'_>,
) -> Json<Option<String>> {
    let auth_info_clone = data.clone();
    let folder = user::login(&Some(auth_info_clone.email), &state.db).await;
    match folder {
        Ok(t) => {
            // verify user with password and if valid add to cookie and return user's email
            if verify_password(&auth_info_clone.password, &t.password) {
                let user_details = format!("{}:{}", t.email, &t.password);
                let account_token = account_encoder(user_details);
                cookies.add_private(Cookie::new("email", t.email));
                Json(Some(account_token))
            } else {
                // return nothing if password doesn't match records
                Json(Some("Password does't match account".to_string()))
            }
        }
        // return nothing if error
        Err(_) => Json(Some("No user found".to_string())),
    }
}

#[get("/user/cookie/<email>")]
pub async fn fetch_by_cookie(
    db: &State<DB>,
    email: String,
    cookies: &CookieJar<'_>,
) -> Result<Json<user::UserOutput>, ErrorT> {
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
) -> Result<Json<Vec<user::UserOutput>>, ErrorT> {
    let folder = user::get(&state.db).await;
    match folder {
        Ok(data) => Ok(Json(data)),
        Err(e) => return Err(e),
    }
}

#[get("/user/<id>")]
pub async fn get_one(
    state: &State<DB>,
    _auth: AuthenticatedUser,
    id: String,
) -> Result<Json<user::UserOutput>, ErrorT> {
    let folder = user::get_one(&Some(id), &state.db).await;
    match folder {
        Ok(data) => Ok(Json(data)),
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
    let decoded_token = account_decoder(_auth.token).unwrap();
    if decoded_token.email == email {
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
