use axum::Router;
use axum::routing::get;
use axum_extra::response::{Html, Css, JavaScript};
use maud::{DOCTYPE, html, Markup};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(|| async { Html(svelteTest().await) }))
        // .route("/favicon.png", get(|| async { /* TODO: Image handler */} ))
        .route("/style.css", get(|| async { Css(getCss().await) }))
        .route("/components.js", get(|| async { JavaScript(getJsBundle().await) }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn getCss() -> String {
    tokio::fs::read_to_string("components/dist/style.css").await.expect("file should exist")
}

async fn getJsBundle() -> String {
    tokio::fs::read_to_string("components/dist/components.js").await.expect("file should exist")
}

async fn svelteTest() -> Markup {
    html! {
        (DOCTYPE)
        head {
            title { "Kinbox" }
            link href="favicon.png" rel="icon" type="image/png";
            link href="style.css" rel="stylesheet";
            script defer type="module" src="components.js" { }
        }
        body {
            my-element name="Rust" { }
        }
    }
}
