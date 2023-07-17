use actix_web::{get, HttpResponse};

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", "0.0.1"))
        .finish()
}

#[tracing::instrument]
#[get("/version")]
async fn version(db: actix_web::web::Data<sqlx::PgPool>) -> String {
    tracing::info!("Getting version");

    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(db.get_ref())
        .await;

    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {:?}", e),
    }
}
