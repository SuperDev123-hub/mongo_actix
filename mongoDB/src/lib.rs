use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId}, results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection
};

pub struct MongoDbClient {
    client: Client
}

impl MongoDbClient {
    pub async fn init(uri: &str) -> Self {
        let client = Client::with_uri_str(uri).await.expect("err connecting to database");
        MongoDbClient{client}
    }
}
