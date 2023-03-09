use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;
use crate::models::fursuit_model::Fursuit;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub active: bool,
    pub suspended: bool
}
