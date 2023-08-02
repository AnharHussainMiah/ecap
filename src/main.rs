mod logo;

use lazy_static::lazy_static;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::{json, with_status};
use warp::Filter;
use validator::Validate;

const VERSION: &str = "0.1.0";

lazy_static! {
    static ref KEY: String = self::load_key();
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
struct Payload {
    #[validate(length(max = 300, message = "the email exceeds 300 characters"), email( message = "this email does not appear to be valid"))]
    email: String,
}

#[tokio::main]
async fn main() {
    logo::draw(&VERSION);
    let api_key = warp::header::exact("x-api-key", &KEY);

    let post_send = warp::post()
        .and(warp::path("submit"))
        .and(warp::path::end())
        .and(api_key)
        .and(self::extract_json_of::<Payload>())
        .and_then(self::submit);

    warp::serve(post_send).run(([0, 0, 0, 0], 8080)).await;
}

pub fn extract_json_of<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn submit(payload: Payload) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(err) = payload.validate() {
        let concatenated_errors = err.field_errors().values().map(|v| {
            let nested_errors = v.iter().map(|ve| {
                    if let Some(m) = &ve.message {
                        m.to_string()
                    } else {
                        "".to_string()
                    }
                }).collect::<Vec<String>>().join(",");
                nested_errors
        }).collect::<Vec<String>>().join(",");

        return Ok(with_status(json(&concatenated_errors), StatusCode::BAD_REQUEST));
    }

    Ok(with_status(json(&"success"), StatusCode::OK))
}

fn load_key() -> String {
    return match env::var("API-KEY") {
        Ok(v) => v,
        Err(_) => Uuid::new_v4().to_string(),
    };
}
