use crate::db;
use poem::{
    listener::TcpListener,
    middleware::Cors,
    session::{CookieConfig, CookieSession},
    web::cookie::{CookieKey, SameSite},
    Endpoint, EndpointExt, Response, Route, Server,
};
use poem_openapi::{Object, OpenApiService, Tags};
use rust_helpers::{serde_json::json, time::DurationExt};
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

#[derive(serde::Serialize, serde::Deserialize, Object)]
pub struct StatusOk {
    pub status: String,
}
impl From<()> for StatusOk {
    fn from(_: ()) -> Self {
        Self {
            status: "ok".to_string(),
        }
    }
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
                .same_site(SameSite::None)
                .name("token"),
        ))
        .around(|ep, req| async move {
            let path = req.uri().path().to_string();
            let ip = req
                .header("cf-connecting-ip")
                .map(ToString::to_string)
                .unwrap_or_else(|| {
                    req.remote_addr()
                        .0
                        .as_socket_addr()
                        .map(|s| s.ip().to_string())
                        .unwrap_or("unknown ip".to_string())
                });
            match ep.call(req).await {
                res @ Ok(_) => res,
                Err(ref e) => {
                    eprintln!("{ip} {path}: {e:?}");
                    Ok(Response::builder()
                        .status(e.status())
                        .content_type("application/json")
                        .body(
                            json!({
                                "error": e.to_string()
                            })
                            .to_string(),
                        ))
                }
            }
        })
        .with(
            Cors::new()
                .allow_origin("http://localhost:5173")
                .allow_credentials(true),
        );

    Server::new(TcpListener::bind("127.0.0.1:18080"))
        .run(app)
        .await
        .unwrap();
}
