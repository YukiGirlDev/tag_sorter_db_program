use std::collections::HashSet;

use crate::{
    quick_sort::{self, sort_pairs},
    tag_search::TagSearcher,
    Item,
};
use rocket::time::{Date, Month};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::get_db::get_db;

#[derive(Deserialize, Debug)]
pub struct Request {
    name: String,
    tags: HashSet<String>,
    entering: String,
    start_year: Option<i32>,
    end_year: Option<i32>,
    start_month: Option<u8>,
    end_month: Option<u8>,
    start_day: Option<u8>,
    end_day: Option<u8>,
    page: usize,
    items_per_page: usize,
    sort: bool,
}
#[derive(Debug, Error)]
enum Error {
    #[error("{0}")]
    FailedToReadDb(#[from] crate::get_db::Error),
    #[error("Attemped to search database with page size of 0, this does not work for reasons im sure your smart enough to figure out.")]
    PageSizeIsZero,
}
#[derive(Serialize)]
struct Response {
    items: Vec<Item>,
    tags: Vec<String>,
    page_count: usize,
}
#[derive(Serialize)]
struct ResponseOrError {
    is_ok: bool,
    response: Option<Response>,
    error: Option<String>,
}
use rocket::serde::json::Json;
#[allow(clippy::needless_pass_by_value)]
#[post("/api", format = "application/json", data = "<search>")]
pub async fn search(search: Json<Request>) -> String {
    let response = match run_search(&search).await {
        Ok(resp) => ResponseOrError {
            is_ok: true,
            response: Some(resp),
            error: None,
        },
        Err(error) => ResponseOrError {
            is_ok: false,
            response: None,
            error: Some(error.to_string()),
        },
    };

    serde_json::to_string(&response).unwrap_or_else(|err| format!("{{\"is_ok\": false, \"error\": {:?}}}", err.to_string()))
}
async fn run_search(search: &Json<Request>) -> Result<Response, Error> {
    let db = get_db().await?;
    // Filter database
    let (mut db, tag_recommends): (Vec<Item>, TagSearcher) = filter_db(db, search);
    // Sort database
    quick_sort::quick_sort(&mut db);
    if search.sort {
        db.reverse();
    }
    if search.items_per_page == 0 {
        return Err(Error::PageSizeIsZero)
    }
    let page_count = {
        
        if db.len() % search.items_per_page == 0 {
            db.len() / search.items_per_page
        } else {
            (db.len() / search.items_per_page) + 1
        }
    };
    // Create page
    let paged = {
        let mut out = Vec::new();
        let x = db.len();
        for i in db
            .into_iter()
            .take({
                let y = (search.page) * search.items_per_page;
                if x < y {
                    x
                } else {
                    y
                }
            })
            .skip((search.page - 1) * search.items_per_page)
        {
            out.push(i);
        }
        out
    };
    // Find best tag recomendations
    let mut pairs = tag_recommends
        .get()
        .into_iter()
        .collect::<Vec<(String, usize)>>();
    sort_pairs(&mut pairs);
    let recommended_tags = pairs.into_iter().map(|x| x.0).collect::<Vec<String>>();
    Ok(Response {
        items: paged,
        tags: recommended_tags,
        page_count,
    })
}
fn filter_db(db: Vec<Item>, search: &Json<Request>) -> (Vec<Item>, TagSearcher) {
    let date1 = (|| {
        {
            Date::from_calendar_date(
                search.start_year?,
                from_number(search.start_month?),
                search.start_day?,
            )
            .ok()
        }
    })();
    let date2 = (|| {
        Date::from_calendar_date(
            search.end_year?,
            from_number(search.end_month?),
            search.end_day?,
        )
        .ok()
    })();
    let mut tag_recommends = TagSearcher::default();
    (
        db.into_iter()
            .filter(|x| {
                let date = Date::from_calendar_date(x.year, from_number(x.month), x.day).ok();
                (if search.name.is_empty() {
                    true
                } else {
                    x.path
                        .file_name()
                        .and_then(std::ffi::OsStr::to_str)
                        .unwrap_or("")
                        .contains(&search.name)
                }) && tag_recommends.check(&search.tags, &search.entering, &x.tags)
                    && (if date1.is_some() || date2.is_some() {
                        date.map_or(false, |date| {
                            let a = date1.map_or(true, |date1| date1 <= date);
                            let b = date2.map_or(true, |date2| date2 >= date);
                            a && b
                        })
                    } else {
                        true
                    })
            })
            .collect::<Vec<Item>>(),
        tag_recommends,
    )
}

pub const fn from_number(n: u8) -> Month {
    match n {
        2 => Month::February,
        3 => Month::March,
        4 => Month::April,
        5 => Month::May,
        6 => Month::June,
        7 => Month::July,
        8 => Month::August,
        9 => Month::September,
        10 => Month::October,
        11 => Month::November,
        12 => Month::December,
        _ => Month::January,
    }
}
