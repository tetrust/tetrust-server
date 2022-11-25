use std::sync::Arc;

use axum::{
    http::{header::AUTHORIZATION, Request, StatusCode},
    middleware::Next,
    response::Response,
    Extension,
};
use mongodb::Database;

use crate::{extensions::CurrentUser, routes::user::UserService, utils::jwt};

pub async fn auth_middleware<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if let Some(auth_header) = auth_header {
        println!(">> Authorization: {}", auth_header);

        let user_id = jwt::verify(auth_header.to_owned());

        if let Some(user_id) = user_id {
            println!(">> Authorize Success");
            let database = req.extensions().get::<Arc<Database>>().unwrap();
            let user_service = UserService::new(Extension(database.to_owned()));

            if let Ok(Some(user)) = user_service.find_by_id(user_id).await {
                let current_user = Arc::new(CurrentUser {
                    user: Some(user),
                    authorized: true,
                });
                req.extensions_mut().insert(current_user);
            }
        }
    }

    Ok(next.run(req).await)
}
