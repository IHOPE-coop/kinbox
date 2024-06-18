use std::fmt::Debug;
use maud::{html, Markup, Render};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, opt::PatchOp, sql::Thing, Result, Surreal};

use crate::ledger::{self, Stamp};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserId(pub Thing);

impl Render for UserId {
    fn render(&self) -> Markup {
        let text = format!("{}:{}", self.0.tb, self.0.id);
        html! {
            (text)
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    page: Vec<String>,
    notifs: Vec<Stamp>,
}

impl User {
    pub async fn add_to_page(&self, body: String, db: &Surreal<Client>) -> Option<User> {
        let mut result = db.query("UPDATE $user SET page += $body")
                                     .bind(("user", &self.id.0))
                                     .bind(("body", body))
                                     .await.expect("query should work");
        result.take(0).expect("User should exist")
    }

    pub fn page(&self) -> impl Iterator<Item = &String> {
        self.page.iter()
    }

    pub fn notifs(&self) -> impl Iterator<Item = &Stamp> {
        self.notifs.iter()
    }
}

pub async fn get(username: &str, db: &Surreal<Client>) -> Option<User> {
    let mut result = db
        .query("SELECT * FROM user WHERE username=$name")
        .bind(("name", username)).await
        .expect("Query failed");
    result.take(0).ok().flatten()
}


pub async fn send(stamp: Stamp, db: &Surreal<Client>) -> Option<User> {
    let mut result = db.query("UPDATE $user SET notifs += $body")
        .bind(("user", &stamp.recipient.0.id))
        .bind(("body", stamp))
        .await.expect("query should work");
    result.take(0).expect("User should exist")
}

pub async fn accept(stamp: Stamp, db: &Surreal<Client>) -> Vec<Stamp> {
    let _ = db.query("UPDATE $user SET notifs -= $stamp")
        .bind(("user", &stamp.recipient.0))
        .bind(("stamp", &stamp))
        .await.expect("query should work");

    ledger::add(stamp, &db).await
}