use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Form;
use axum_extra::response::Html;
use maud::{html, Markup};
use serde::Deserialize;
use crate::Context;

pub async fn hx_needs_get(Path(username): Path<String>, State(state): State<Context>) -> (StatusCode, Html<Markup>) {
    return if let Some(current) = state.current(username.as_str()) {
        println!("{:?}", current);
        let iter = current.page();
        (StatusCode::OK, Html(html! {
            ul {
                @for need in iter {
                    li class="list-item" {
                        span class="item-text" {"item" (*need)}
                        button class="duplicate-button" {"Duplicate"}
                        button class="delete-button" {"Delete"}
                    }
                }
            }
        }))
    } else {
        (StatusCode::NOT_FOUND, Html(html! {"Invalid username"}))
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct NeedForm {
    need: String
}
pub async fn hx_needs_post(Path(username): Path<String>, State(mut state): State<Context>, Form(form): Form<NeedForm>) -> (StatusCode, Html<Markup>) {
    return match state.current_mut(username.as_str()) {
        None => return (StatusCode::NOT_FOUND, Html(html! {"Invalid username"})),
        Some(&mut ref mut user) => {
            user.add_to_page(form.need.as_str());
            hx_needs_get(Path(username), State(state)).await
        }
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
