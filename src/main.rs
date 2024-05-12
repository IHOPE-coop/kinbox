mod user;
mod stamp;

use axum::extract::{Path, State};
use axum::Router;
use axum::routing::get;
use axum_extra::response::{Html, Css, JavaScript};
use maud::{DOCTYPE, html, Markup};
use crate::stamp::Ledger;
use crate::user::User;

#[derive(Clone)]
struct Users {
    nathan: User,
    harley: User,
    ledger: Ledger
}

impl Users {
    fn current(&self, username: &'static str) -> Option<&User> {
        match username.as_str() {
            "nathan" => Some(&self.nathan),
            "harley" => Some(&self.harley),
            _ => None
        }
    }

    fn other(&self, username: &'static str) -> Option<&User> {
        match username.as_str() {
            "nathan" => Some(&self.harley),
            "harley" => Some(&self.nathan),
            _ => None
        }
    }
}

#[tokio::main]
async fn main() {
    let users = Users {
        nathan: User::new("Nathan"),
        harley: User::new("Harley"),
        ledger: Ledger::default()
    };

    let router = Router::new()
        .route("/", get(svelteTest))
        // .route("/favicon.png", get(|| async { /* TODO: Image handler */} ))
        .route("/style.css", get(getCss))
        .route("/components.js", get(getJsBundle))
        .route("/user/:username", get(show_view))
        .route("/hx-needs/:username", get(needs))
        .route("/hx-notifs/:username", get(notifs))
        .route("/hx-ledger/:username", get(ledger))
        .with_state(users);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn getCss() -> Css<String> {
    Css(tokio::fs::read_to_string("components/dist/style.css").await.expect("file should exist"))
}

async fn getJsBundle() -> JavaScript<String> {
    JavaScript(tokio::fs::read_to_string("components/dist/components.js").await.expect("file should exist"))
}

async fn show_view(Path(username): Path<String>, State(state): State<Users>) -> Html<Markup> {
    Html(html! {
        p {"Current: " (match state.current(username.as_str()) {
                None => "Invalid user",
                Some(user) => user.username
            })
        }

        p {"Other: " (match state.other(username.as_str()) {
                None => "No other",
                Some(user) => user.username
            })
        }
    })
}

async fn needs(Path(username): Path<String>, State(state): State<Users>) -> Html<Markup> {
    let current = state.current(username.as_str()).unwrap();
    todo!(current.page());
    Html(html! { })
}

async fn notifs(Path(username): Path<String>, State(state): State<Users>) -> Html<Markup> {
    let other = state.other(username.as_str()).unwrap();
    todo!(other.sent());
    Html(html! { })
}

async fn ledger(Path(username): Path<String>, State(state): State<Users>) -> Html<Markup> {
    todo!(state.ledger.of_user(username.as_str()));
    Html(html! { })
}

async fn svelteTest() -> Html<Markup> {
    Html(html! {
        (DOCTYPE)
        head {
            title { "Kinbox" }
            link href="favicon.png" rel="icon" type="image/png";
            link href="style.css" rel="stylesheet";
            script defer type="module" src="components.js" { }
        }
        body {
            div id="app" { }
        }
    })
}
