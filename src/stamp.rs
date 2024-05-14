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

    pub fn giver(&self) -> &str {
        &self.giver
    }

    pub fn recipient(&self) -> &str {
        &self.recipient
    }

    pub fn description(&self) -> &str {
        &self.description
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

    pub fn of_user(&self, username: &str) -> impl Iterator<Item = &Stamp> {
        let username = username.to_owned();
        self.records.iter().filter(move |&record| {
            record.giver == username
            || record.recipient == username
        })
    }
}
