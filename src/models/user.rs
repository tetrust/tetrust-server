use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// 사용자 계정 모델

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub password_salt: String,
    pub is_anonymous: bool,
}

impl User {
    pub const NAME: &'static str = "user";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertUser {
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub password_salt: String,
    pub is_anonymous: bool,
}
