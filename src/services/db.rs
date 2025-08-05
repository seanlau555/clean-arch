use std::env;

use actix_web::http::Error;
// use mongodb::bson::doc;
use mongodb::{
    bson::{doc, oid::ObjectId},
    results::{InsertOneResult, UpdateResult},
    Client, Collection,
};

use crate::models::{booking_model::Booking, dog_model::Dog, owner_model::Owner};

pub struct Database {
    booking: Collection<Booking>,
    dog: Collection<Dog>,
    owner: Collection<Owner>,
}

impl Database {
    pub async fn init() -> Self {
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => "mongodb://localhost:27017/?directConnection=true".to_string(),
        };
        let client = Client::with_uri_str(uri)
            .await
            .ok()
            .expect("Error in connecting mongodb");
        let db = client.database("dog_walking");

        let booking: Collection<Booking> = db.collection("booking");
        let dog: Collection<Dog> = db.collection("dog");
        let owner: Collection<Owner> = db.collection("owner");

        Database {
            booking,
            dog,
            owner,
        }
    }

    pub async fn create_owner(&self, owner: Owner) -> Result<InsertOneResult, Error> {
        let result = self
            .owner
            .insert_one(owner)
            .await
            .ok()
            .expect("Error creating owner");

        Ok(result)
    }

    pub async fn create_dog(&self, dog: Dog) -> Result<InsertOneResult, Error> {
        let result = self
            .dog
            .insert_one(dog)
            .await
            .ok()
            .expect("Error creating dog");

        Ok(result)
    }

    pub async fn create_booking(&self, booking: Booking) -> Result<InsertOneResult, Error> {
        let result = self
            .booking
            .insert_one(booking)
            .await
            .ok()
            .expect("Error creating booking");

        Ok(result)
    }

    pub async fn cancel_booking(&self, booking_id: &str) -> Result<UpdateResult, Error> {
        let result = self
            .booking
            .update_one(
                doc! {
                    "_id": ObjectId::parse_str(booking_id).expect("Failed to parse booking_id")
                },
                doc! {
                    "$set": doc! {
                        "cancelled": true
                    }
                },
            )
            .await
            .ok()
            .expect("Error cancelling booking");

        Ok(result)
    }
}
