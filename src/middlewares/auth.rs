use std::sync::Arc;

use axum::{
    http::{header::AUTHORIZATION, Request, StatusCode},
    middleware::Next,
    response::Response,
    Extension,
};
use mongodb::Database;
use url::{ParseError, Url};

use crate::{extensions::CurrentUser, routes::user::UserService, utils::jwt};

pub async fn auth_middleware<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .map(|e| e.to_owned());

    let auth_query = {
        let uri = "http://fake.host".to_owned() + req.uri().to_string().as_str();
        let parsed_url = Url::parse(uri.as_str()).ok();
        parsed_url.map(|e| {
            e.query_pairs()
                .into_owned()
                .find(|(key, _)| key.to_owned() == "AUTHORIZATION")
                .map(|(_, value)| value.to_owned())
        })
    }
    .flatten();

    let auth_header = auth_header.or(auth_query);

    let mut current_user = CurrentUser::default();

    if let Some(auth_header) = auth_header {
        println!(">> Authorization: {}", auth_header);

        let user_id = jwt::verify(auth_header.to_owned());

        if let Some(user_id) = user_id {
            println!(">> Authorize Success");
            let database = req.extensions().get::<Arc<Database>>().unwrap();
            let user_service = UserService::new(Extension(database.to_owned()));

            if let Ok(Some(user)) = user_service.find_by_id(user_id).await {
                current_user = CurrentUser {
                    user: Some(user),
                    authorized: true,
                };
            }
        }
    }

    req.extensions_mut().insert(current_user);

    Ok(next.run(req).await)
}
