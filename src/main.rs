use axum::Server;
use azman::{api::apply_routes, util::handle_error};
use dotenv::dotenv;
use std::{env, net::SocketAddr, time::Duration};
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tower_http::compression::CompressionLayer;

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "app=DEBUG")
    }
    tracing_subscriber::fmt().pretty().init();
    dotenv().ok();
    let middlewares = ServiceBuilder::new()
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        // .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new());
    let routes = apply_routes().layer(middlewares).handle_error(handle_error);

    let port = env::var("APP_PORT").expect("environment variable APP_PORT must be set");
    let port = port
        .parse::<u16>()
        .expect("environment variable APP_PORT must be u16");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .expect("app started failed")
}
