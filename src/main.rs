use axum::{routing::{get}, Json, Router};
use axum::extract::State;
use axum::http::StatusCode;
use bb8::{Pool};
use bb8_postgres::PostgresConnectionManager;
use serde::Serialize;
use tokio_postgres::NoTls;

#[derive(Serialize)]
struct Content {
    id: i32,
    title: String,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();


    let manager =
        PostgresConnectionManager::new_from_stringlike("host=localhost user=postgres password=example", NoTls)
            .unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();

 // build our application with some routes
    let app = Router::new()
        .route(
            "/",
            get(get_contents),
        )
        .with_state(pool);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

#[axum::debug_handler]
async fn get_contents(
    State(pool): State<ConnectionPool>,
) -> Result<Json<Vec<Content>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let rows = conn
        .query("select * from content", &[])
        .await
        .map_err(internal_error)?;

    let contents: Vec<Content> = rows.iter().map(|row| Content {
        id: row.get(0),
        title: row.get(1)
    }).collect();

    Ok(Json(contents))
}


// async fn using_connection_pool_extractor(
//     State(pool): State<ConnectionPool>,
// ) -> Result<String, (StatusCode, String)> {
//     let conn = pool.get().await.map_err(internal_error)?;
//
//     let row = conn
//         .query_one("select 1 + 1", &[])
//         .await
//         .map_err(internal_error)?;
//     let two: i32 = row.try_get(0).map_err(internal_error)?;
//
//     Ok(two.to_string())
// }


fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}