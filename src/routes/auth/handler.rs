use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Json, Router,
};
use mongodb::Database;

use crate::{
    extensions::mongo::MongoClient,
    models::{InsertUser, User},
    routes::user::UserService,
    utils::{generate_uuid, hash_password},
};

use super::{
    dto::{LoginRequest, LoginResponse},
    AuthService,
};

pub async fn router() -> Router {
    let app = Router::new().route("/login", post(login));

    app
}

async fn login(
    Json(body): Json<LoginRequest>,
    database: Extension<Arc<Database>>,
) -> impl IntoResponse {
    let auth_service = AuthService::new(database.clone());
    let user_service = UserService::new(database);

    let mut response = LoginResponse {
        access_token: "".into(),
        success: false,
    };

    let email = body.email;
    let password = body.password;

    match user_service.find_by_email(email).await {
        Ok(user) => {
            if let Some(user) = user {
                let salt = user.password_salt;
                let hashed_password = hash_password(password, &salt);

                if hashed_password == user.password {
                    response.success = true;

                    let user_id = user._id.to_hex();
                    let access_token = auth_service.get_access_token(user_id);

                    response.access_token = access_token;
                }
            }
        }
        Err(error) => {
            println!("error: {:?}", error);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    Json(response).into_response()
}
