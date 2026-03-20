use contact_management::app::route::create_router;
use contact_management::app::state::AppState;
use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env or environment");

    let mut options = ConnectOptions::new(database_url);
    options
        .max_connections(10)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(true);

    let db: DatabaseConnection = Database::connect(options)
        .await
        .expect("Failed to connect to PostgreSQL");

    let state = AppState { db };
    let app = create_router(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");

    println!("Server running on http://{}", addr);

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
