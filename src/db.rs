use surrealdb::{engine::remote::ws::{Client, Ws}, opt::auth::Root, Surreal, Result};

const ADDRESS: &str = "127.0.0.1:8000";
const CREDENTIALS: Root = Root {username: "root", password: "root"};
const NS: &str = "Surrealist";
const DB: &str = "CLI";

pub struct DB {
    db: Surreal<Client>
}

impl DB {
    async fn new() -> Self {
        let db: Result<Surreal<Client>> = async {
            let db = Surreal::new::<Ws>(ADDRESS).await?;
            db.signin(CREDENTIALS).await?;
            db.use_ns(NS).use_db(DB).await?;
            Ok(db)
        }.await;

        Self { db: db.expect("Database should exist") }
    }
}