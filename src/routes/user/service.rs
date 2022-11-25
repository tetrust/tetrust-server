use std::sync::Arc;

use axum::Extension;
use mongodb::{
    bson::{doc, Array},
    Database,
};
use std::error::Error;

use crate::models::{InsertUser, User};

pub struct UserService {
    database: Extension<Arc<Database>>,
}

impl UserService {
    pub fn new(database: Extension<Arc<Database>>) -> Self {
        Self { database }
    }

    pub async fn exists_email(&self, email: String) -> Result<bool, Box<dyn Error>> {
        let user = self.database.collection::<User>(User::NAME);

        let filter = doc! {"email": email};
        let result = user.find_one(filter, None).await?;
        Ok(result.is_some())
    }

    pub async fn find_by_email(&self, email: String) -> Result<Option<User>, Box<dyn Error>> {
        let user = self.database.collection::<User>(User::NAME);

        let filter = doc! {"email": email};
        let result = user.find_one(filter, None).await?;

        Ok(result)
    }

    pub async fn create_user(&self, user_data: InsertUser) -> Result<String, Box<dyn Error>> {
        let user = self.database.collection::<InsertUser>(User::NAME);

        let result = user.insert_one(user_data, None).await?;
        let user_id = result.inserted_id.as_object_id().unwrap();
        let user_id = user_id.to_hex();

        Ok(user_id)
    }
}
