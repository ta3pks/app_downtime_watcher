pub(super) struct WatcherApi;
#[derive(serde::Deserialize, serde::Serialize, Object)]
struct DeleteEpRequest {
    name: String,
    url: String,
}
use poem_openapi::{
    payload::{Form, Json},
    Object, OpenApi,
};

use crate::db::{self, Db};
use poem::{web::Data, Result};

use super::guards::{AdminGuard, UserGuard};
#[OpenApi(prefix_path = "/api/endpoints", tag = "super::Groups::Endpoints")]
impl WatcherApi {
    #[oai(path = "/", method = "get")]
    async fn get_endpoints(&self, _u: UserGuard, db: Data<&Db>) -> Result<Json<Vec<db::Endpoint>>> {
        Ok(Json(db.get_endpoints().await?))
    }
    #[oai(path = "/", method = "post")]
    ///Admin required
    async fn add_endpoint(
        &self,
        ep: Form<db::Endpoint>,
        db: Data<&Db>,
        _u: AdminGuard,
    ) -> Result<()> {
        db.add_endpoint(&ep).await?;
        Ok(())
    }
    #[oai(path = "/", method = "delete")]
    async fn delete_endpoint(
        &self,
        ep: Form<DeleteEpRequest>,
        db: Data<&Db>,
        _u: AdminGuard,
    ) -> Result<()> {
        db.delete_endpoint(&ep.name, &ep.url).await?;
        Ok(())
    }
}
