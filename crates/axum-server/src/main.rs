mod config;
mod errors;

use crate::errors::CustomError;
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{Html, Response},
    response::IntoResponse,
    response::Redirect,
    routing::get,
    routing::post,
    Form,
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use validator::Validate;

#[tokio::main]
async fn main() {
        let config = config::Config::new();

    let pool = db::create_pool(&config.database_url);

    // build our application with a route
    let app = Router::new()
        .route("/", get(users))
        .route("/sign_up", post(accept_form))
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await.unwrap();
}

async fn users(Extension(pool): Extension<db::Pool>) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let users = db::queries::users::get_users()
        .bind(&client)
        .all()
        .await?;

    // We now return HTML
    Ok(Html(ui_components::users::users(
        users,
    )))
}

#[derive(Deserialize, Validate)]
struct SignUp {
    #[validate(email)]
    email: String,
}

async fn accept_form(
    Extension(pool): Extension<db::Pool>,
    Form(form): Form<SignUp>,
) -> Result<Response, CustomError> {
    if form.validate().is_err() {
        return Ok((StatusCode::BAD_REQUEST, "Bad request").into_response());
    }

    let client = pool.get().await?;

    let email = form.email;
    // TODO - accept a password and hash it
    let hashed_password = String::from("aaaa");
    let _ = db::queries::users::create_user()
        .bind(&client, &email.as_str(), &hashed_password.as_str())
        .await?;

    // 303 redirect to users list
    Ok(Redirect::to("/").into_response())
}
