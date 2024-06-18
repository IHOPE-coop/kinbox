mod user;
mod ledger;
mod handlers;
mod db;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Router;
use axum::routing::{get, post};
use axum_extra::response::{Html, Css, JavaScript};
use maud::{DOCTYPE, html, Markup};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use surrealdb::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let db = init_db().await.expect("Panic if the database fails");
    let users = Context {
        db
    };

    let router = Router::new()
        .route("/", get(svelteTest))
        // .route("/favicon.png", get(|| async { /* TODO: Image handler */} ))
        // .route("/pages/*page", get(getPage))
        .route("/bundles/:bundle", get(getJsBundle))
        .route("/style.css", get(getCss))
        .route("/user/style.css", get(getCss))
        .route("/login", get(login))
        .route("/user/:username/:other", get(show_view))
        .route("/hx-needs/:username", get(handlers::hx_needs_get).post(handlers::hx_needs_post))
        .route("/hx-notifs/:username", get(handlers::hx_notifs))
        .route("/hx-ledger/:username", get(handlers::hx_ledger));
    let router = router.with_state(users);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
    Ok(())
}

async fn init_db() -> Result<Surreal<Client>> {
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
    db: Surreal<Client>,
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

async fn show_view(Path((username, other)): Path<(String, String)>, State(state): State<Context>) -> Markup {
    html! {
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
    }
}

async fn svelteTest() -> Markup {
    html! {
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
    }
}
