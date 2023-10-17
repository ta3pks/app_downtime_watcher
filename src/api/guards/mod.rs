use educe::Educe;
use poem_openapi::{
    auth::{ApiKey, Basic},
    SecurityScheme,
};
use rust_helpers::time::now_nanos;
use serde::{Deserialize, Serialize};

#[derive(SecurityScheme, Educe)]
#[educe(Deref)]
#[oai(ty = "basic", checker = "verify_login")]
pub struct Login(User);

#[derive(SecurityScheme, Educe)]
#[oai(
    ty = "api_key",
    key_in = "header",
    key_name = "token",
    checker = "verify_user"
)]
#[educe(Deref)]
pub struct UserGuard(User);

#[derive(SecurityScheme, Educe)]
#[oai(
    ty = "api_key",
    key_in = "cookie",
    key_name = "token",
    checker = "verify_admin"
)]
#[educe(Deref)]
pub struct AdminGuard(User);

use poem::{session::Session, Request, Result};
#[derive(Serialize, Deserialize, Debug)]
struct Token {
    uid: String,
    time: u64,
}

use crate::db::{self, Db, User};
async fn verify_login(r: &Request, login: Basic) -> Result<db::User> {
    let usr = r.data::<Db>().unwrap().get_user(&login.username).await?;
    if usr.verify_password(&login.password) {
        r.extensions().get::<Session>().unwrap().set(
            "token",
            Token {
                uid: login.username,
                time: now_nanos(),
            },
        );
        Ok(usr.empty_password())
    } else {
        Err(crate::Error::Unauthorized.into())
    }
}

async fn fetch_user(r: &Request, is_admin: bool) -> Result<User> {
    let ses = r.extensions().get::<Session>().unwrap();
    let tkn = ses
        .get::<Token>("token")
        .ok_or(crate::Error::Unauthorized)?;
    let usr = r.data::<Db>().unwrap().get_user(&tkn.uid).await?;
    if is_admin && !usr.is_admin {
        return Err(crate::Error::Unauthorized.into());
    }
    ses.set("token", tkn);
    Ok(usr.empty_password())
}

async fn verify_admin(r: &Request, _: ApiKey) -> Result<User> {
    fetch_user(r, true).await
}
async fn verify_user(r: &Request, _: ApiKey) -> Result<User> {
    fetch_user(r, false).await
}
