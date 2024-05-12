pub struct Record {
    giver: String,
    recipient: String,
    description: String
}

impl Record {
    pub fn new(giver: &str, recipient: &str, description: &str) -> Self {
        Self {
            giver: giver.to_owned(),
            recipient: recipient.to_owned(),
            description: description.to_owned()
        }
    }
}

#[derive(Default)]
pub struct Ledger {
    records: Vec<Record>
}

impl Ledger {
    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, record: Record) {
        self.records.push(record)
    }

    fn of_user(&self, username: &str) -> impl Iterator {
        self.records.iter().filter(|&record| {
            record.giver == username
            || record.recipient == username
        })
    }
}
