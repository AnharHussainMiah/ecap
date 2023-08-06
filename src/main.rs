mod logo;

use lazy_static::lazy_static;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use std::env;
use std::process;
use uuid::Uuid;
use validator::Validate;
use warp::http::StatusCode;
use warp::reply::{json, with_status};
use warp::Filter;
use warp::http::Method;

const VERSION: &str = "0.1.0";

lazy_static! {
    static ref KEY: String = self::load_key("API_KEY");
    static ref DBURL: String = self::load_key("DATABASE_URL");
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
struct Payload {
    #[validate(
        length(max = 250, message = "the email is too long, can not exceed 250 characters"),
        email(message = "this is not a valid email address")
    )]
    email: String,
}

#[tokio::main]
async fn main() {
    logo::draw(&VERSION);
    let api_key = warp::header::exact("x-api-key", &KEY);

    if let Ok(pool) = PgPool::connect(&DBURL).await {
        let _ = sqlx::migrate!().run(&pool).await;

        // the CORS Preflight call we need to allow the client to access the resource
        let cors = warp::cors()
                        .allow_methods(&[Method::POST])
                        .allow_headers(vec!["x-api-key", "content-type"])
                        .allow_any_origin();

        let cors_copy = cors.clone();

        let preflight = warp::any().map(warp::reply).with(cors);
        
        let post_submit = warp::post()
            .and(warp::path("submit"))
            .and(warp::path::end())
            .and(api_key)
            .and(self::extract_json_of::<Payload>())
            .and(warp::any().map(move || pool.clone()))
            .and_then(self::submit)
            .with(cors_copy);

        let port = self::get_port();

        println!("==> starting server on port {}, CTRL+C to stop...", port);
        let routes = post_submit.or(preflight);
        warp::serve(routes).run(([0, 0, 0, 0], port)).await;
    } else {
        println!("WARNING: unable to establish a database connection, exiting...");
        process::exit(1);
    }
}

pub fn extract_json_of<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn submit(payload: Payload, pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(err) = payload.validate() {
        let concatenated_errors = err
            .field_errors()
            .values()
            .map(|v| {
                let nested_errors = v
                    .iter()
                    .map(|ve| {
                        if let Some(m) = &ve.message {
                            m.to_string()
                        } else {
                            "".to_string()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(",");
                nested_errors
            })
            .collect::<Vec<String>>()
            .join(",");

        println!("==> rejecting invalid email submitted");

        return Ok(with_status(
            json(&concatenated_errors),
            StatusCode::BAD_REQUEST,
        ));
    }

    let email = payload
        .email
        .replace(" ", "")
        .to_lowercase()
        .trim()
        .to_string();

    if let Ok(is_duplicate) = self::is_email_exists(&pool, &email).await {
        if !is_duplicate {
            let _ = self::insert_email(&pool, &email).await;
            println!("==> inserting new email record [{}]", email);
        } else {
            println!("==> skipping email is duplicate [{}]", email);
        }
        return Ok(with_status(json(&"success"), StatusCode::OK));
    }

    println!("==> warning internal server error unable to process email");

    Ok(with_status(
        json(&"Unable to process request"),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

fn load_key(k: &str) -> String {
    return match env::var(k) {
        Ok(v) => v,
        Err(_) => Uuid::new_v4().to_string(),
    };
}

async fn insert_email(pool: &PgPool, email: &str) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        r#"
        insert into emails (email, date_added)
        values             ($1,     now()     );
        "#,
        email
    )
    .fetch_one(pool)
    .await?;
    Ok(())
}

async fn is_email_exists(pool: &PgPool, email: &str) -> Result<bool, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
        select count(1)::int as hit from emails where email = $1;
        "#,
        email
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.hit > Some(0))
}


fn get_port() -> u16 {
    let mut port: u16 = 8080;
    if let Ok(v) = env::var("ECAP_PORT") {
        if let Ok(v) = v.parse::<u16>() {
            port = v;
        } else {
            println!("WARNING: rejecting port [{}] as invalid", v);
        }
    }
    port
}