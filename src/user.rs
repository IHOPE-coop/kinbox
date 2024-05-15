use std::fmt::{Debug, Formatter, Pointer};
use crate::stamp::{Ledger, Stamp};

#[derive(Debug, Default, PartialEq, Clone)]
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

    pub fn add_to_page(&mut self, body: &str) {
        self.page.push(body.to_string())
    }

    pub fn send(&mut self, stamp: usize, user: &User) {
        let body = self.page.remove(stamp);
        let stamp = Stamp::new(self.username, user.username, body.as_str());
        self.sent.push(stamp);
    }

    pub fn sent(&self) -> impl Iterator<Item = &Stamp> {
        self.sent.iter()
    }

    pub fn accept(&mut self, stamp: usize, ledger: &mut Ledger) {
        let body = self.sent.remove(stamp);
        ledger.add(body);
    }
}
