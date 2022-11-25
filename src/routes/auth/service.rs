use std::sync::Arc;

use axum::Extension;
use epoch_timestamp::Epoch;
use mongodb::{
    bson::{doc, Array},
    Database,
};
use std::error::Error;

use crate::{models::User, utils::jwt};

pub struct AuthService {
    _database: Extension<Arc<Database>>,
}

impl AuthService {
    pub fn new(database: Extension<Arc<Database>>) -> Self {
        Self {
            _database: database,
        }
    }

    pub fn get_access_token(&self, user_id: String) -> String {
        let epoch = (Epoch::now() + Epoch::day(1)) as usize;

        jwt::sign(epoch, user_id)
    }
}
