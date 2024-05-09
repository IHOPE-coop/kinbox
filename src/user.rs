#[derive(Default, PartialEq, Clone)]
pub struct User {
    pub username: &'static str,
    page: Vec<String>,
    sent: Vec<String>,
    ledger: Vec<String>
}

impl User {
    pub fn new(username: &'static str) -> Self {
        Self {
            username,
            page: Vec::new(),
            sent: Vec::new(),
            ledger: Vec::new()
        }
    }

    pub fn page(&self) -> impl Iterator + '_ {
        self.page.iter()
    }

    pub fn add_to_page(&mut self, body: &str) {
        self.page.push(body.to_string())
    }

    pub fn send(&mut self, stamp: usize) {
        let body = self.page.remove(stamp);
        self.sent.push(body);
    }

    pub fn sent(&self) -> impl Iterator + '_ {
        self.sent.iter()
    }

    pub fn accept(&mut self, stamp: usize) {
        let body = self.sent.remove(stamp);
        self.ledger.push(body);
    }

    pub fn ledger(&self) -> impl Iterator + '_ {
        self.ledger.iter()
    }
}
