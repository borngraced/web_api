use crate::connection::DB;
use crate::model;
use model::category;
use mongodb::results::{DeleteResult, UpdateResult};
pub use rocket::{serde::json::Json, State};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Input {
    pub name: String,
}

#[get("/category/<name>")]
pub async fn get(state: &State<DB>, name: Option<String>) -> Json<Vec<category::Category>> {
    let cat = category::get(&name, &state.db).await.unwrap();
    Json(cat)
}

#[get("/category")]
pub async fn get_one(state: &State<DB>) -> Json<Vec<category::Category>> {
    let cat = category::get(&None, &state.db).await.unwrap();
    Json(cat)
}

#[post("/category", format = "json", data = "<input>")]
pub async fn add(input: Json<Input>, state: &State<DB>) -> Json<Vec<category::Category>> {
    //let inp = Some(input.name);
    let category = category::post(&Some(input.name.clone()), &state.db).await;
    match category {
        Ok(t) => Json(t),
        Err(e) => Json(vec![e]),
    }
}

#[delete("/category/<name>/delete")]
pub async fn delete(name: String, state: &State<DB>) -> Json<DeleteResult> {
    let category = category::delete(&name, &state.db).await.unwrap();
    Json(category)
}

#[put("/category/<id>", format = "json", data = "<input>")]
pub async fn put(input: Json<Input>, id: String, state: &State<DB>) -> Json<UpdateResult> {
    let category = category::put(&input.name, &id, &state.db).await.unwrap();
    Json(category)
}
