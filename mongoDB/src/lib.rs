use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection, Database,
};
use serde::{de::DeserializeOwned, Serialize};
pub struct MongoDbClient {
    client: Client,
    db: Database,
}

pub trait MongoDbModel: Sync + Send + Unpin {
    fn model_name() -> String;
}

impl MongoDbClient {
    pub async fn init(uri: &str, database: &str) -> Self {
        let client = Client::with_uri_str(uri)
            .await
            .expect("err connecting to database");
        let db = client.database(database);
        MongoDbClient { client, db }
    }

    pub async fn insert_one<T>(&self, data: T) -> Result<InsertOneResult, Error>
    where
        T: MongoDbModel + Serialize,
    {
        let model_name = T::model_name().clone();
        let col = self.db.collection(&model_name);
        let ret = col
            .insert_one(data, None)
            .await
            .ok()
            .expect(&format!("Error occured while adding {}", &model_name));
        Ok(ret)
    }
}
