use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::user::UserId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StampId(pub Thing);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stamp {
    pub id: StampId,
    pub giver: UserId,
    pub recipient: UserId,
    pub description: String
}

pub async fn add(stamp: Stamp, db: &Surreal<Client>) -> Option<Stamp> {
    db.create::<Option<Stamp>>("ledger").content(stamp).await.expect("query should work")
}

pub async fn of_user(username: &str, db: &Surreal<Client>) -> impl Iterator<Item = Stamp> {
    let mut result = db
        .query("SELECT * FROM ledger WHERE recipient.username=$name OR giver.username=$name")
        .bind(("name", username))
        .await
        .expect("Query failed");
    let ledger: Vec<Stamp> = result.take(0).unwrap_or_default();
    ledger.into_iter()
}
