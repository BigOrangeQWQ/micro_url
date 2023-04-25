use diesel::prelude::*;
use dotenvy::dotenv;
use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
    routing::{get, post},
    Json, Router,
};

use micro_url::errors::{internal_error, not_found_error};
use micro_url::models::{NewLink, ShortURL};
use micro_url::sql::{establish_connection, SqlitePool};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = establish_connection();
    let app = Router::new()
        .route("/surl/api", post(short_url))
        .route("/mt/:qsalt", get(redirect_url))
        .with_state(pool);

    // run it with hyper on 0.0.0.0:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

//
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
    let mut acsii_url = String::new();
    let hasher = DefaultHasher::new();
    for i in hasher.finish().to_ne_bytes().iter().take(6) {
        if (i % 127).is_ascii_alphanumeric() {
            print!("is");
            acsii_url.push(i.clone() as char);
        } else {
            acsii_url.push((rand::thread_rng().gen_range(65..90) as u8) as char);
        }
    }
    let new = NewLink {
        salt: acsii_url.clone(),
        link: payload.url,
    };
    let _ = diesel::insert_into(links::table)
        .values(new)
        .execute(&mut conn)
        .map_err(internal_error);

    Ok(Json(acsii_url))
}
