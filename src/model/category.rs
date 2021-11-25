use std::error::Error;

use bson::{doc, oid::ObjectId};
use mongodb::{Database, results::{DeleteResult, UpdateResult}};
use rocket::futures::TryStreamExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    // #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: ObjectId,
    pub name: String,
}

pub async fn post(name: &Option<String>, db: &Database) -> Result<Vec<Category>, Category> {
    let new_doc = doc! {
        "name": &name
    };
    let folder = db.collection("category");
    let _ = folder.insert_one(new_doc.clone(), None).await;
    let cat = get(name, &db).await;
    Ok(cat.unwrap())
}

pub async fn get(name: &Option<String>, db: &Database) -> Result<Vec<Category>, Box<dyn Error>> {
    let cat = db.collection::<Category>("category");
    let mut load_category: Vec<Category> = vec![];

    match name {
        Some(n) => {
            let query = doc! {"name":  n};
            if let Some(category) = cat.find_one(query, None).await? {
                load_category.push(category);
                Ok(load_category)
            } else {
                return Err("not found".into());
            }
        }
        None => {
            let mut cursor = cat.find(None, None).await?;
            while let Some(data) = cursor.try_next().await? {
                load_category.push(Category {
                    _id: data._id,
                    name: data.name,
                });
            }
            Ok(load_category)
        }
    }
}

pub async fn delete(name: &String, db: &Database) -> Result<DeleteResult, Box<dyn Error>> {
    let cat = db.collection::<Category>("category");
    let query = doc! {"name":  name};
    let result = cat.delete_one(query, None).await?;
    Ok(result)
}

pub async fn put(
    name: &String,
    id: &String,
    db: &Database,
) -> Result<UpdateResult, Box<dyn Error>> {
    let cat = db.collection::<Category>("category");
    let query = doc! {"_id":  id};
    let update = doc! {"$set":{"name":  name}};
    let updated_cat = cat.update_one(query, update, None).await?;
    Ok(updated_cat)
}
