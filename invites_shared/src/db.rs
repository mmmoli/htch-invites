use async_trait::async_trait;
use surrealdb::{Datastore, Response, Session};

#[async_trait]
pub trait HtchDb {
    async fn execute(&self, query: &str) -> anyhow::Result<Vec<Response>>;
}

pub struct Db {
    datastore: Datastore,
    session: Session,
}

impl Db {
    pub async fn new() -> anyhow::Result<Self> {
        let datastore = Datastore::new("memory").await?;
        let session = Session::for_kv();
        Ok(Self { datastore, session })
    }
}

#[async_trait]
impl HtchDb for Db {
    async fn execute(&self, query: &str) -> anyhow::Result<Vec<Response>> {
        let response = self
            .datastore
            .execute(query, &self.session, None, false)
            .await?;
        Ok(response)
    }
}
