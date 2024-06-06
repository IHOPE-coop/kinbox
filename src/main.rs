mod user;
mod ledger;
mod handlers;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Router;
use axum::routing::{get, post};
use axum_extra::response::{Html, Css, JavaScript};
use maud::{DOCTYPE, html, Markup};
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use crate::user::User;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = init_db().await.expect("Panic if the database fails");
    let users = Context {
        nathan: User::new("Nathan"),
        harley: User::new("Harley"),
        db
    };

    // Create a new person with a random id
    let created: Vec<Record> = users.db
        .create("person")
        .content(Person {
            title: "Founder & CEO",
            name: Name {
                first: "Tobie",
                last: "Morgan Hitchcock",
            },
            marketing: true,
        })
    .await?;
    dbg!(created);

    // Select all people records
    let people: Vec<Record> = users.db.select("person").await?;
    dbg!(people);

    let router = Router::new()
        .route("/", get(svelteTest))
        // .route("/favicon.png", get(|| async { /* TODO: Image handler */} ))
        // .route("/pages/*page", get(getPage))
        .route("/bundles/:bundle", get(getJsBundle))
        .route("/style.css", get(getCss))
        .route("/user/style.css", get(getCss))
        .route("/login", get(login))
        .route("/user/:username", get(show_view))
        .route("/hx-needs/:username", get(handlers::hx_needs_get).post(handlers::hx_needs_post))
        .route("/hx-notifs/:username", get(handlers::hx_notifs))
        .route("/hx-ledger/:username", get(handlers::hx_ledger));
    let router = router.with_state(users);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
    Ok(())
}

#[derive(Debug, Serialize)]
struct Name<'a> {
    first: &'a str,
    last: &'a str,
}

#[derive(Debug, Serialize)]
struct Person<'a> {
    title: &'a str,
    name: Name<'a>,
    marketing: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

async fn init_db() -> surrealdb::Result<Surreal<Client>> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    }).await?;
    db.use_ns("Surrealist").use_db("CLI").await?;
    Ok(db)
}

#[derive(Clone)]
struct Context {
    nathan: User,
    harley: User,
    db: Surreal<Client>,
}

impl Context {
    fn current(&self, username: &str) -> Option<&User> {
        match username {
            "nathan" => Some(&self.nathan),
            "harley" => Some(&self.harley),
            _ => None
        }
    }

    fn current_mut(&mut self, username: &str) -> Option<&mut User> {
        match username {
            "nathan" => Some(&mut self.nathan),
            "harley" => Some(&mut self.harley),
            _ => None
        }
    }

    fn other(&self, username: &str) -> Option<&User> {
        match username {
            "nathan" => Some(&self.harley),
            "harley" => Some(&self.nathan),
            _ => None
        }
    }

    fn other_mut(&mut self, username: &str) -> Option<&mut User> {
        match username {
            "nathan" => Some(&mut self.harley),
            "harley" => Some(&mut self.nathan),
            _ => None
        }
    }
}

// async fn getPage(Path(page): Path<String>) -> (StatusCode, Html<String>) {
//     let path = format!("components/dist/{}", page);
//     match tokio::fs::read_to_string(path).await {
//         Ok(page) => (StatusCode::OK, Html(page)),
//         Err(_) => (StatusCode::NOT_FOUND, Html("".to_owned()))
//     }
// }

async fn getJsBundle(Path(bundle): Path<String>) -> (StatusCode, JavaScript<String>) {
    let path = format!("components/dist/{}", bundle);
    match tokio::fs::read_to_string(path).await {
        Ok(bundle) => (StatusCode::OK, JavaScript(bundle)),
        Err(_) => (StatusCode::NOT_FOUND, JavaScript("".to_owned()))
    }
}

async fn getCss() -> Css<String> {
    Css(tokio::fs::read_to_string("components/dist/style.css").await.expect("file should exist"))
}

async fn login() -> Html<Markup> {
    Html(html! {
        (DOCTYPE)
        head {
            title { "Kinbox" }
            link href="favicon.png" rel="icon" type="image/png";
            link href="style.css" rel="stylesheet";
            script defer type="module" src="/bundles/login.js" { }
        }
        body {
            div id="app" { }
        }
    })
}

async fn show_view(Path(username): Path<String>, State(state): State<Context>) -> Html<Markup> {
    let other = match state.other(username.as_str()) {
        None => "",
        Some(user) => user.username,
    };
    Html(html! {
        (DOCTYPE)
        head {
            title { "Kinbox" }
            link href="favicon.png" rel="icon" type="image/png";
            link href="style.css" rel="stylesheet";
            script defer type="module" src="/bundles/app.js" { }
        }
        body {
            div id="app" current=(username) other=(other) { }
        }
    })
}

async fn svelteTest() -> Html<Markup> {
    Html(html! {
        (DOCTYPE)
        head {
            title { "Kinbox" }
            link href="favicon.png" rel="icon" type="image/png";
            link href="style.css" rel="stylesheet";
            script defer type="module" src="/bundles/starter.js" { }
        }
        body {
            div id="app" message="Rust" { }
        }
    })
}
