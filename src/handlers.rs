use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum_extra::response::Html;
use maud::{html, Markup};
use crate::Context;

pub async fn hx_needs(Path(username): Path<String>, State(state): State<Context>) -> (StatusCode, Html<Markup>) {
    return if let Some(current) = state.current(username.as_str()) {
        let iter = current.page();
        (StatusCode::OK, Html(html! {
            ul {
                @for need in iter {
                    li {(need)}
                }
            }
        }))
    } else {
        (StatusCode::NOT_FOUND, Html(html! {"Invalid username"}))
    }
}

pub async fn hx_notifs(Path(username): Path<String>, State(state): State<Context>) -> (StatusCode, Html<Markup>) {
    return if let Some(other) = state.other(username.as_str()) {
        let iter = other.sent();
        (StatusCode::OK, Html(html! {
            ul {
                @for stamp in iter {
                    li {(stamp.giver())"->"(stamp.recipient())": "(stamp.description())}
                }
            }
        }))
    } else {
        (StatusCode::NOT_FOUND, Html(html! {"Invalid username"}))
    }
}

pub async fn hx_ledger(Path(username): Path<String>, State(state): State<Context>) -> Html<Markup> {
    // todo!(state.ledger.of_user(username.as_str()));
    let iter = state.ledger.of_user(username.as_str());
    Html(html! {
        ul {
            @for stamp in iter {
                li {(stamp.giver())"->"(stamp.recipient())": "(stamp.description())}
            }
        }
    })
}
