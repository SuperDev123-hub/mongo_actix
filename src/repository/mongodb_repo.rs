use std::env;
extern crate dotenv;

use actix_web::web::Path;
use dotenv::dotenv;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::InsertOneResult,
    sync::{Client, Collection},
};

use crate::models::user_model::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self.col.insert_one(new_doc, None).unwrap();

        Ok(user)
    }

    pub fn get_user(&self, id: String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self.col.find_one(filter, None).unwrap();

        Ok(user_detail.unwrap())
    }

    // pub fn update_user(&self, path: Path<String>, new_user: User) -> Result<User, Error> {
    //     let id = path.into_inner();
    //     let obj_id = ObjectId::parse_str(id).unwrap();
    //     let filter = doc! {"_id": obj_id};
    //     // let new_doc =  User {
    //     //     id: Some(obj_id),
    //     //     name: new_user.name,
    //     //     location: new_user.location,
    //     //     title: new_user.title,
    //     // };
    //     let new_doc = doc! { "$set":
    //     {"name": new_user.name,
    //     "location": new_user.location,
    //     "title": new_user.title}};
    //     let updated_doc = self.col.update_one(filter, new_doc, None).unwrap();
    //     let mut result: User;

    //     if updated_doc.modified_count == 1 {
    //        let result = self.col.find_one(filter, None).unwrap();
    //     };
    // }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self.col.find(None, None).unwrap();
        let users = cursors.map(|doc| doc.unwrap()).collect();

        Ok(users)
    }
}
