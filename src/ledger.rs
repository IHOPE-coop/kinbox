use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::Record;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stamp {
    pub giver: String,
    pub recipient: String,
    pub description: String
}

pub async fn add(record: Stamp, db: &Surreal<Client>) -> surrealdb::Result<Vec<Record>> {
    db.create("stamp").content(record).await
}

pub async fn of_user(username: &str, db: &Surreal<Client>) -> surrealdb::Result<impl Iterator<Item = Stamp>> {
    let people: Vec<Stamp> = db.select("stamp").await?;

    let username = username.to_owned();
    Ok(people.into_iter().filter(move |record| {
        record.giver == username
        || record.recipient == username
    }))
}
