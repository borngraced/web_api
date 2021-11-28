use std::str::from_utf8;

use crate::{connection::DB, error::ErrorT, model::user};
use base64::{decode, encode};
use bcrypt::{hash, verify, DEFAULT_COST};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuthenticatedUser {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ErrorT;

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let db = request.guard::<DB>().await;
        let token = request.headers().get_one("X-Token");
        match token {
            Some(t) => {
                let decode_token = account_decoder(t.to_string());
                match decode_token {
                    Ok(d) => {
                        let maybe_user = user::login(&Some(d.email), &db.unwrap().db).await;
                        match maybe_user {
                            Ok(i) => {
                                println!("{}, {}", &i.password, &d.password);
                                if i.password == d.password {
                                    Outcome::Success(AuthenticatedUser {
                                        token: t.to_string(),
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
                                    status: Status::NotFound,
                                    message: "Account Does Not Exist".to_string(),
                                },
                            )),
                        }
                    }

                    Err(_) => Outcome::Failure((
                        Status::Forbidden,
                        ErrorT {
                            status: Status::Forbidden,
                            message: "Forbidden X-Token Token".to_string(),
                        },
                    )),
                }
            }
            None => Outcome::Failure((
                Status::Forbidden,
                ErrorT {
                    status: Status::Forbidden,
                    message: "No X-Token Token Provided".to_string(),
                },
            )),
        }
    }
}

#[derive(Debug)]
pub struct AccountInfo {
    pub email: String,
    pub password: String,
}
pub fn account_encoder(info: String) -> String {
    let token = encode(info);
    token
}
pub fn account_decoder(info: String) -> Result<AccountInfo, String> {
    let result = decode(info);
    match result {
        Ok(details) => {
            let string_from_u8 = from_utf8(&details);
            match string_from_u8 {
                Ok(d) => {
                    let infos: Vec<_> = d.split(":").collect();
                    return Ok(AccountInfo {
                        email: infos[0].to_string(),
                        password: infos[1].to_string(),
                    });
                }
                Err(_) => return Err("Invalid Account".to_string()),
            }
        }
        Err(_) => return Err("Token not decodeable".to_string()),
    }
}

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

    #[test]
    fn code_test() {
        let result = account_encoder("Onoja:OPE".to_string());
        println!("{}", result)
    }

    #[test]
    fn decode_test() {
        let r = account_decoder("T25vamE6T1BF".to_string()).unwrap();
        println!("{:?}", r)
    }
}
