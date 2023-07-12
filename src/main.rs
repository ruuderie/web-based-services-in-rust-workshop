use std::net::SocketAddr;
use webservice_rust_workshop::{router, AppState, SharedState};

type BoxError = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let state = SharedState::default();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let app = router(state.clone());

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
