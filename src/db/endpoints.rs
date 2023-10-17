use crate::Result;
use educe::Educe;
use poem_openapi::Object;

#[derive(Debug, serde::Serialize, serde::Deserialize, clickhouse::Row, Object, Educe)]
#[educe(Default)]
pub struct Endpoint {
    /// A descriptive name for the endpoint
    pub name: String,
    /// The url to check
    pub url: String,
    /// The maximum latency in seconds
    pub max_latency: usize,
    /// The group this endpoint belongs to
    #[educe(Default = "default")]
    pub group: String,
    /// Allowed users to see this endpoint
    #[oai(skip)]
    pub allowed_users: Vec<String>,
}
impl Endpoint {
    pub const CREATE_TABLE: &'static str = r#"
        create table if not exists endpoints (
            name String,
            url String,
            group String default 'default',
            max_latency UInt64,
            allowed_users Array(String),
        ) order by (name,url)
    "#;
}
impl super::Db {
    pub async fn add_endpoint(&self, endpoint: &Endpoint) -> Result<()> {
        let mut inserter = self.client.insert("endpoints")?;
        inserter.write(endpoint).await?;
        inserter.end().await?;
        Ok(())
    }
    pub async fn get_endpoints(&self) -> Result<Vec<Endpoint>> {
        Ok(self
            .client
            .query("select * from endpoints")
            .fetch_all()
            .await?)
    }
    pub async fn delete_endpoint(&self, name: &str, url: &str) -> Result<()> {
        self.client
            .query("delete from endpoints where name=? and url=?")
            .bind(name)
            .bind(url)
            .execute()
            .await?;
        Ok(())
    }
}
