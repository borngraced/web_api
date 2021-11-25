use std::error::Error;

use crate::{
    error::ErrorT,
    middleware::{auth::hash_password, validate::validate_email},
};
use bson::{doc, oid::ObjectId};
use mongodb::{
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Database,
};
use rocket::{futures::TryStreamExt, http::Status, response::content::Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub username: String,
    pub email: String,
    pub age: i32,
    pub password: String,
    pub country: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Input {
    pub username: String,
    pub email: String,
    pub age: i32,
    pub password: String,
    pub country: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthInfo {
    pub email: String,
    pub password: String,
}

pub async fn post(data: &Input, db: &Database) -> Result<InsertOneResult, ErrorT> {
    let folder = db.collection("users");

    //check if password is longer than 8
    if data.password.len() < 8 {
        Json("Password too short".to_string());
        return Err(ErrorT {
            status: Status::ExpectationFailed,
            message: "Password is too short".to_string(),
        });
    }
    let mut email = String::new();
    match validate_email(&data.email) {
        Ok(t) => email.push_str(&t),
        Err(_) => {
            return Err(ErrorT {
                status: Status::ExpectationFailed,
                message: "Email not valid".to_string(),
            })
        }
    }
    //encrypted password
    let hash_password = hash_password(&data.password);

    //check for existing email
    let folder2 = db.collection::<User>("users");
    let mut cursor = folder2.find(None, None).await.unwrap();
    while let Some(d) = cursor.try_next().await.unwrap() {
        if to_lowercase(&d.email) == to_lowercase(&email) {
            return Err(ErrorT {
                status: Status::NotFound,
                message: format!("User found for email: {}", &email),
            });
        }
    }

    //mongo doc to insert new user detials to our mongodb table
    let new_doc = doc! {
        "username": &data.username,
        "email": email,
        "age": &data.age,
        "password": hash_password,
        "country": &data.country
    };

    //get collection name from our database
    let operation_result = folder.insert_one(new_doc.clone(), None).await;
    match operation_result {
        Ok(t) => Ok(t),
        Err(_) => Err(ErrorT {
            status: Status::NotFound,
            message: "Check field and try again".to_string(),
        }),
    }
}

pub async fn get_one(email: &Option<String>, db: &Database) -> Result<AuthInfo, ErrorT> {
    let folder = db.collection::<User>("users");
    let e = email.as_ref().unwrap();
    let q = doc! {"email":  e};
    //let q2 = doc! {"_id":  ObjectId::with_string(best) };
    //let fakeget = folder.find_one(q2, None).await;
   // println!("{:?}", fakeget);
    let load_user = folder.find_one(q, None).await;
    match load_user {
        Ok(t) => match t {
            Some(u) => {
                let user = Box::new(u);
                Ok(AuthInfo {
                    email: user.email,
                    password: user.password,
                })
            }
            None => Err(ErrorT {
                status: Status::NotFound,
                message: "No user found".to_string(),
            }),
        },
        Err(_) => Err(ErrorT {
            status: Status::NotFound,
            message: "No user found".to_string(),
        }),
    }
}

pub async fn get(db: &Database) -> Result<Vec<User>, ErrorT> {
    let folder = db.collection::<User>("users");

    let mut cursor = folder.find(None, None).await.unwrap();
    match cursor.try_next().await {
        Ok(data) => match data {
            Some(data) => {
                let mut load_user: Vec<User> = vec![];
                load_user.push(User {
                    _id: data._id,
                    username: data.username,
                    email: data.email,
                    age: data.age,
                    password: data.password,
                    country: data.country,
                });
                Ok(load_user)
            }
            None => {
                return Err(ErrorT {
                    status: Status::NotFound,
                    message: "No user found".to_string(),
                })
            }
        },
        Err(_) => {
            return Err(ErrorT {
                status: Status::NotFound,
                message: "No user found".to_string(),
            })
        }
    }
}

pub async fn delete(name: &String, db: &Database) -> Result<DeleteResult, Box<dyn Error>> {
    let folder = db.collection::<User>("users");
    let query = doc! {"name":  name};
    let result = folder.delete_one(query, None).await?;
    Ok(result)
}

pub async fn edit(data: &User, id: &String, db: &Database) -> Result<UpdateResult, Box<dyn Error>> {
    let cat = db.collection::<User>("users");
    let query = doc! {"_id":  id};
    let update = doc! {"$set":{"name":  &data.username}};
    let updated_user = cat.update_one(query, update, None).await?;
    Ok(updated_user)
}

fn to_lowercase(e: &String) -> String {
    e.to_lowercase()
}
