pub(super) struct UserApi;
use poem_openapi::{param::Path, payload::Json, OpenApi};

use crate::db::{self, Db, User};
use poem::{web::Data, Result};

use super::guards::{AdminGuard, Login};

#[OpenApi(prefix_path = "/api/users", tag = "super::Groups::Users")]
impl UserApi {
    #[oai(path = "/", method = "get")]
    ///list all users
    async fn list_users(&self, db: Data<&Db>, _u: AdminGuard) -> Result<Json<Vec<db::User>>> {
        Ok(Json(db.list_users().await?))
    }
    #[oai(path = "/", method = "post")]
    ///add a new user
    async fn add_user(&self, usr: Json<db::User>, _u: AdminGuard, db: Data<&Db>) -> Result<()> {
        db.new_user((*usr).clone()).await?;
        Ok(())
    }
    #[oai(path = "/:id", method = "delete")]
    ///delete a user
    async fn delete_user(&self, id: Path<String>, db: Data<&Db>, _u: AdminGuard) -> Result<()> {
        db.delete_user(&id).await?;
        Ok(())
    }
    #[oai(path = "/login", method = "post")]
    ///login using basic auth
    async fn login(&self, usr: Login) -> Result<Json<User>> {
        Ok(Json(usr.clone()))
    }
}
