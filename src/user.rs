use std::fmt::{Debug, Formatter, Pointer};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::ledger::{self, Stamp};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: &'static str,
    page: Vec<String>,
    sent: Vec<Stamp>,
}

impl User {
    pub fn new(username: &'static str) -> Self {
        Self {
            username,
            page: Vec::new(),
            sent: Vec::new(),
        }
    }

    pub fn page(&self) -> impl Iterator<Item = &String> {
        self.page.iter()
    }

    pub fn add_to_page(&mut self, body: String) {
        self.page.push(body)
    }

    pub fn send(&mut self, stamp: usize, user: &User) {
        let body = self.page.remove(stamp);
        let stamp = Stamp {
            giver: self.username.to_string(),
            recipient: user.username.to_string(),
            description: body.to_string()
        };
        self.sent.push(stamp);
    }

    pub fn sent(&self) -> impl Iterator<Item = &Stamp> {
        self.sent.iter()
    }

    pub async fn accept(&mut self, stamp: usize, db: &Surreal<Client>) {
        let body = self.sent.remove(stamp);
        ledger::add(body, &db).await;
    }
}
