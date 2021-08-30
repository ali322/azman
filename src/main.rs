#[tokio::main]
async fn main() {
    use axum::Server;
    use azman::api::v1::apply_routes;
    use dotenv::dotenv;
    use std::{env, net::SocketAddr};
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "app=DEBUG")
    }
    tracing_subscriber::fmt().pretty().init();
    dotenv().ok();
    let port = env::var("APP_PORT").expect("environment variable APP_PORT must be set");
    let port = port
        .parse::<u16>()
        .expect("environment variable APP_PORT must be u16");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let routes = apply_routes();
    Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .expect("app started failed")
}
