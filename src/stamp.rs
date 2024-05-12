#[derive(Clone, PartialEq)]
pub struct Stamp {
    giver: String,
    recipient: String,
    description: String
}

impl Stamp {
    pub fn new(giver: &str, recipient: &str, description: &str) -> Self {
        Self {
            giver: giver.to_owned(),
            recipient: recipient.to_owned(),
            description: description.to_owned()
        }
    }
}

#[derive(Default, Clone)]
pub struct Ledger {
    records: Vec<Stamp>
}

impl Ledger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, record: Stamp) {
        self.records.push(record)
    }

    pub fn of_user(&self, username: &str) -> impl Iterator {
        self.records.iter().filter(|&record| {
            record.giver == username
            || record.recipient == username
        })
    }
}
