use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
};
use serde::{Deserialize, Serialize};
use std::env;
use crate::{connection::DB, error::ErrorT, model::user};


#[derive(Debug, Serialize)]
pub struct AuthenticatedUser {
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}


//#[rocket::async_trait]
/*impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ErrorT;

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let db = request.guard::<DB>().await;
        let authorization_header = request.headers().get_one("Authorization");
        //let email = request.headers().get_one("email");
        //let password = request.headers().get_one("password");
        //let cookie = request.headers().cookies;

        let token = match authorization_header {
            Some(s) => s.to_string(),
            None => "Data not valid on server, try again".to_string(),
        };
        //let key_vec: Vec<&str> = token.split(" ").collect();
        let secret = env::var("JWT_SECRET").expect("You must set the MONGODB_URI environment var!");
        let k = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.he0ErCNloe4J7Id0Ry2SEDg09lKkZkfsRiGsdX_vgEg";
        // Decode the JWT
        let token = jsonwebtoken::decode::<TokenClaims>(
            k,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
            // Do not require the "exp" claim in the token payload
            &jsonwebtoken::Validation {
                validate_exp: false,
                ..Default::default()
            },
        );
        println!("{:?}", &token);
        if token.is_err() {
            return Outcome::Success(AuthenticatedUser {
                email: "v".to_string(),
            });
        } else {
            return Outcome::Failure((
                Status::NotFound,
                ErrorT {
                    status: Status::Unauthorized,
                    message: "Email Doesn't Exist".to_string(),
                },
            ));
        }
        /*match (email, password) {
            (Some(e), Some(p)) => {
                let maybe_user = user::get_one(&Some(e.to_string()), &db.unwrap().db).await;
                match maybe_user {
                    Ok(t) => {
                        let verify = verify_password(&p.to_string(), &t.password);
                        if verify {
                            Outcome::Success(AuthenticatedUser {
                                email: e.to_string(),
                            })
                        } else {
                            Outcome::Failure((
                                Status::NonAuthoritativeInformation,
                                ErrorT {
                                    status: Status::NonAuthoritativeInformation,
                                    message: "Unauthorized".to_string(),
                                },
                            ))
                        }
                    }
                    Err(_) => Outcome::Failure((
                        Status::NotFound,
                        ErrorT {
                            status: Status::Unauthorized,
                            message: "Email Doesn't Exist".to_string(),
                        },
                    )),
                }
            }
            _ => Outcome::Failure((
                Status::ExpectationFailed,
                ErrorT {
                    status: Status::ExpectationFailed,
                    message: "Data not valid on server, try again".to_string(),
                },
            )),
        }*/
    }
}*/
pub fn hash_password(p: &String) -> String {
    //p as the password
    let hasher = hash(p, DEFAULT_COST);
    let p = hasher.unwrap();
    p.to_string()
}

pub fn verify_password(p: &String, h: &String) -> bool {
    //p as the password and h as the hash
    let v = verify(p, h);
    v.unwrap()
}

#[derive(Debug, Deserialize)]
struct TokenClaims {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn hash() {
        println!("{}", hash_password(&"Ope".to_string()))
    }

    #[test]
    fn testhasher() {
        assert_eq!(
            true,
            verify_password(
                &"Ope".to_string(),
                &"$2b$12$ZcMgtsJVO8cIcI5zzkDKgOqsIBJBoQ.bor2GivRCELo2ptoMMZhfm".to_string(),
            ),
        )
    }
}
