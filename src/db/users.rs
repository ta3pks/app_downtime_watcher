use crate::Result;
use poem_openapi::Object;

#[derive(Debug, serde::Serialize, serde::Deserialize, clickhouse::Row, Object, Clone)]
pub struct User {
    pub id: String,
    pub group: String,
    #[oai(skip_serializing_if_is_empty)]
    pub password: String,
    pub is_admin: bool,
    pub permissions: u64,
    pub theme: String,
}

impl User {
    pub const CREATE_TABLE: &'static str = r#"
        create table if not exists users (
            id String ,
            group String default 'default',
            password String,
            is_admin Boolean,
            permissions UInt64,
            theme String default 'crimson',
        ) engine = ReplacingMergeTree() order by (id)
    "#;
    fn hash_password(&mut self) {
        self.password = format!(
            "{:x}",
            md5::compute(format!("{}{}", self.id, &self.password))
        );
    }
    pub fn verify_password(&self, password: &str) -> bool {
        format!("{:x}", md5::compute(format!("{}{}", self.id, password))) == self.password
    }
    pub fn empty_password(mut self) -> Self {
        self.password = String::new();
        self
    }
}

impl super::Db {
    pub async fn list_users(&self) -> Result<Vec<User>> {
        Ok(self
            .client
            .query("select * from users")
            .fetch_all()
            .await?
            .into_iter()
            .map(User::empty_password)
            .collect())
    }
    pub async fn get_user(&self, id: &str) -> Result<User> {
        Ok(self
            .client
            .query("select * from users where id=?")
            .bind(id)
            .fetch_one()
            .await?)
    }
    pub async fn delete_user(&self, id: &str) -> Result<()> {
        self.client
            .query("delete from users where id=?")
            .bind(id)
            .execute()
            .await?;
        Ok(())
    }
    pub async fn new_user(&self, mut user: User) -> Result<()> {
        if self.get_user(&user.id).await.is_ok() {
            return Err(crate::Error::AlreadyExists);
        }
        user.hash_password();
        let mut inserter = self.client.insert("users")?;
        inserter.write(&user).await?;
        inserter.end().await?;
        Ok(())
    }
}

#[repr(u64)]
pub enum UserPermissions {
    None = 0,
    Read = 1 << 0,
    Write = 1 << 1,
}
