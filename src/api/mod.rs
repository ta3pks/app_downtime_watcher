use crate::db;
use poem::{
    listener::TcpListener,
    session::{CookieConfig, CookieSession},
    web::cookie::CookieKey,
    EndpointExt, Route, Server,
};
use poem_openapi::{OpenApiService, Tags};
use rust_helpers::time::DurationExt;
mod guards;
mod user_api;
mod watcher_api;
#[derive(Tags)]
#[oai(rename_all = "snake_case")]
enum Groups {
    /// Endpoint management
    Endpoints,
    /// User management [admin only]
    Users,
}
const COOKIE_KEY: &[u8; 64] = b"4LuMhqwbIgM8Pj2RldPoT7sBXzeiVK5krDy9YJZUFtfa6n0QOHxC3SumNcv1WGA!";
pub async fn start(db: db::Db) {
    let watcher = watcher_api::WatcherApi;
    let users_api = user_api::UserApi;
    let api_service = OpenApiService::new((watcher, users_api), "Watcher Api", "1.0");
    let ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/", api_service)
        .nest("/api/docs", ui)
        .data(db)
        .with(CookieSession::new(
            CookieConfig::private(CookieKey::from(COOKIE_KEY))
                .max_age(24.hours())
                .name("token"),
        ));

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
        .unwrap();
}
