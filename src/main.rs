use axum::response::Html;
use axum::Router;
use axum::routing::get;
use axum_extra::response::Css;
use rscx::{CollectFragment, component, html, props};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(|| async { Html(app().await) }))
        .route("/styles.css", get(|| async { Css(getCss().await) }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn getCss() -> String {
    tokio::fs::read_to_string("src/styles.css").await.expect("file should exist")
}

// simple function returning a String
// it will call the Items() function
async fn app() -> String {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <link href="styles.css" rel="stylesheet" />
            </head>
            <body>
                // call a component with no props
                <Section />

                // call a component with props and children
                <Section title="Hello">
                    <Items />
                </Section>
            </body>
        </html>
    }
}

#[component]
/// mark functions with #[component] to use them as components inside html! macro
fn Section(
    // you can use `builder` attributes to specify a default value (makes this prop optional)
    #[builder(default = "Default title".into(), setter(into))] title: String,
    #[builder(default)] children: String,
) -> String {
    html! {
        <div>
            <h1>{ title }</h1>
            { children }
        </div>
    }
}

#[component]
async fn Items() -> String {
    let data = load_data_async().await;
    html! {
        <ul>
            {
                data.into_iter()
                    .map(|item| html! { <li>{ item }</li> })
                    .collect_fragment() // helper method to collect a list of components into a String
            }
        </ul>
    }
}

/// async functions can be easily used in the body of a component, as every component is an async
/// function
async fn load_data_async() -> Vec<String> {
    vec!["a".to_string(), "b".to_string(), "c".to_string()]
}
