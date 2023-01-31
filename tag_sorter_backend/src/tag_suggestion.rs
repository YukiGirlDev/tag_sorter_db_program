use std::collections::HashSet;

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::{get_db::get_db, item::Item};

pub fn get_tag_suggestions(
    db: &Vec<Item>,
    current_tags: &HashSet<String>,
    entering: &String,
) -> Vec<String> {
    let mut out = Vec::new();
    for i in db {
        for e in &i.tags {
            if (!current_tags.contains(e)) && e.starts_with(entering) && !out.contains(e) {
                out.push(e.clone());
            }
        }
        if out.len() == 5 {
            break;
        }
    }
    out
}

#[derive(Deserialize)]
pub struct Request {
    current_tags: HashSet<String>,
    entering: String,
}
#[derive(Serialize)]
struct ResponseOrError {
    is_ok: bool,
    tags: Option<Vec<String>>,
    error: Option<String>,
}
#[allow(clippy::needless_pass_by_value)]
#[post("/tag_suggestions", format = "application/json", data = "<tags>")]
pub async fn tag_suggestions(tags: Json<Request>) -> String {
    serde_json::to_string(&run_tag_suggestions(tags).await)
        .unwrap_or_else(|err| format!("{{\"is_ok\":false,\"error\":{:?}}}", err.to_string()))
}
async fn run_tag_suggestions(tags: Json<Request>) -> ResponseOrError {
    let db = match get_db().await {
        Ok(ok) => ok,
        Err(err) => {
            return ResponseOrError {
                is_ok: false,
                tags: None,
                error: Some(err.to_string()),
            }
        }
    };
    ResponseOrError {
        is_ok: true,
        tags: Some(get_tag_suggestions(&db, &tags.current_tags, &tags.entering)),
        error: None,
    }
}
