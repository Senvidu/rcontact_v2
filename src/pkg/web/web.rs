use axum::Router;
use tokio::net::TcpListener;

pub async fn init_web(listener: TcpListener, app: Router) {
    axum::serve(listener, app)
        .await
        .expect("Failed to serve application");
}
