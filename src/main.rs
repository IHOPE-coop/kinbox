mod user;
mod stamp;
mod handlers;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Router;
use axum::routing::get;
use axum_extra::response::{Html, Css, JavaScript};
use maud::{DOCTYPE, html, Markup};
use crate::stamp::Ledger;
use crate::user::User;

#[tokio::main]
async fn main() {
    let users = Context {
        nathan: User::new("Nathan"),
        harley: User::new("Harley"),
        ledger: Ledger::default()
    };

    let router = Router::new()
        .route("/", get(svelteTest))
        // .route("/favicon.png", get(|| async { /* TODO: Image handler */} ))
        .route("/style.css", get(getCss))
        .route("/bundles/:bundle", get(getJsBundle))
        .route("/user/:username", get(show_view))
        .route("/hx-needs/:username", get(handlers::hx_needs))
        .route("/hx-notifs/:username", get(handlers::hx_notifs))
        .route("/hx-ledger/:username", get(handlers::hx_ledger))
        .with_state(users);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

#[derive(Clone)]
struct Context {
    nathan: User,
    harley: User,
    ledger: Ledger
}

impl Context {
    fn current(&self, username: &str) -> Option<&User> {
        match username {
            "nathan" => Some(&self.nathan),
            "harley" => Some(&self.harley),
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
}

async fn getCss() -> Css<String> {
    Css(tokio::fs::read_to_string("components/dist/style.css").await.expect("file should exist"))
}

async fn getJsBundle(Path(bundle): Path<String>) -> (StatusCode, JavaScript<String>) {
    let path = format!("components/dist/{}", bundle);
    match tokio::fs::read_to_string(path).await {
        Ok(bundle) => (StatusCode::OK, JavaScript(bundle)),
        Err(_) => (StatusCode::NOT_FOUND, JavaScript("".to_owned()))
    }
}

async fn show_view(Path(username): Path<String>, State(state): State<Context>) -> Html<Markup> {
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

async fn svelteTest() -> Html<Markup> {
    Html(html! {
        (DOCTYPE)
        head {
            title { "Kinbox" }
            link href="favicon.png" rel="icon" type="image/png";
            link href="style.css" rel="stylesheet";
            script defer type="module" src="/bundles/components.js" { }
        }
        body {
            div id="app" { }
        }
    })
}
