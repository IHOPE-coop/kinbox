use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Form;
use maud::{html, Markup};
use serde::Deserialize;
use crate::{ledger, user};
use crate::Context;

type Response = Result<Markup, (StatusCode, &'static str)>;

pub async fn hx_needs_get(Path(username): Path<String>, State(state): State<Context>) -> Response {
    match user::get(&username, &state.db).await {
        None => Err((StatusCode::NOT_FOUND, "Invalid user")),
        Some(user) => {
            let iter = user.page();
            Ok(html! {
                ul {
                    @for need in iter {
                        li class="list-item" {
                            span class="item-text" {(*need)}
                            button class="duplicate-button" {"Duplicate"}
                            button class="delete-button" {"Delete"}
                        }
                    }
                }
            })
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct NeedForm {
    need: String
}

pub async fn hx_needs_post(Path(username): Path<String>, mut state: State<Context>, Form(form): Form<NeedForm>) -> Response {
    match user::get(&username, &state.db).await {
        None => Err((StatusCode::NOT_FOUND, "Invalid username")),
        Some(mut user) => {
            user.add_to_page(form.need, &state.db);
            hx_needs_get(Path(username), state).await
        }
    }
}

pub async fn hx_notifs(Path(username): Path<String>, State(state): State<Context>) -> Response {
    match user::get(&username, &state.db).await {
        None => Err((StatusCode::NOT_FOUND, "Invalid username")),
        Some(user) => {
            let iter = user.notifs();
            Ok(
                html! {
                    ul {
                        @for stamp in iter {
                            li {(stamp.giver)"->"(stamp.recipient)": "(stamp.description)}
                        }
                    }
                }
            )
        }
    }
}

pub async fn hx_ledger(Path(username): Path<String>, State(state): State<Context>) -> Response {
    // todo!(state.ledger.of_user(username.as_str()));
    let iter = ledger::of_user(&username, &state.db).await;
    Ok(html! {
        ul {
            @for stamp in iter {
                li {(stamp.giver)"->"(stamp.recipient)": "(stamp.description)}
            }
        }
    })
}
