use diesel::prelude::*;
use dotenvy::dotenv;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
    routing::{get, post},
    Json, Router,
};

use micro_url::{errors::{internal_error, not_found_error, found_error}, GenCode};
use micro_url::models::{NewLink, ShortURL};
use micro_url::sql::{establish_connection, SqlitePool};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = establish_connection();
    let app = Router::new()
        .route("/", get(view))
        .route("/api", post(short_url))
        .route("/j/:qsalt", get(redirect_url))
        .with_state(pool);

    // run it with hyper on 0.0.0.0:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn redirect_url(
    Path(qsalt): Path<String>,
    State(sql): State<SqlitePool>,
) -> Result<Redirect, (StatusCode, String)> {
    use micro_url::schema::links::dsl::*;

    let mut conn = sql.get().map_err(internal_error)?;
    let uri: String = links
        .filter(salt.eq(&qsalt))
        .select(link)
        .first::<String>(&mut conn)
        .map_err(not_found_error)?;

    Ok(Redirect::to(&uri))
}

async fn short_url(
    State(sql): State<SqlitePool>,
    Json(payload): Json<ShortURL>,
) -> Result<Json<String>, (StatusCode, String)> {
    use micro_url::schema::links;

    let mut conn = sql.get().map_err(internal_error)?;
    let acsii_url = String::new().generate_code(4).to_owned();

    let new = NewLink {salt: acsii_url.clone(), link: payload.url};
    let _ = diesel::insert_into(links::table)
        .values(new)
        .execute(&mut conn)
        .map_err(found_error);

    Ok(Json(acsii_url))
}

async fn view() {

}