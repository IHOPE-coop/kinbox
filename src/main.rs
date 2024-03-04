use axum::Router;
use axum::routing::get;
use axum_extra::response::Css;
use maud::{DOCTYPE, html, Markup};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(kinbox))
        .route("/styles.css", get(|| async { Css(getCss().await) }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn getCss() -> String {
    tokio::fs::read_to_string("src/styles.css").await.expect("file should exist")
}

fn load_kinbox_buttons() -> Vec<String> {
    vec!["<< Kinpad".to_owned(), "Kinshop >>".to_owned()]
}

fn MenuButton(text: &str) -> Markup {
    html! {
        button class="menu-button text" {
            (text)
        }
    }
}

fn Menu(buttons: Vec<String>) -> Markup {
    html! {
        div class="menu" {
            @for button in &buttons {
                (MenuButton(button));
            }
        }
    }
}

async fn kinbox() -> Markup {
    let buttons = load_kinbox_buttons();

    html! {
        (DOCTYPE)
        head {
            link href="styles.css" rel="stylesheet";
        }
        body {
            div class="page" {
                div class="title text" { "Kinbox" }
            }
            (Menu(buttons));
        }
    }
}
