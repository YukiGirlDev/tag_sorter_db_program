#![deny(unused_must_use)]
#![warn(
    clippy::unwrap_used,
    clippy::all,
    clippy::nursery,
    clippy::pedantic,
    clippy::todo,
    clippy::unimplemented,
    clippy::style
)]
#![allow(clippy::let_unit_value)] // macros are fun
#[macro_use]
extern crate rocket;

mod clap_struct;
mod cores_headers;
mod edit;
mod get_db;
mod get_settings;
mod item;
mod quick_sort;
mod search;
mod select;
mod tag_search;
mod tag_suggestion;

use std::io::Write;

use ::clap::Parser;
use clap_struct::Cli;
use cores_headers::Cors;
use item::Item;
use rocket::fs::FileServer;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

const SETTINGSPATH: &str = "settings.DO.NOT.TOUCH";

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let cli = Cli::parse();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::WARN)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut file = std::fs::File::create(SETTINGSPATH).unwrap_or_else(|err| {
        panic!("Failed to reload settings. Please restart the server! More info: {err}")
    });
    file.write_all(cli.path.display().to_string().as_bytes()).unwrap_or_else(|err| panic!("Failed to write settings to file this may lead to corrupted settings. Please restart the server! More info: {err}"));
    #[allow(clippy::no_effect_underscore_binding)] // Macros are fun!
    let _rocket = rocket::build()
        .attach(Cors)
        .mount(
            "/",
            routes![
                get_source,
                search::search,
                cores_headers::cors,
                cores_headers::cors_suggestions,
                cores_headers::cors_edit,
                tag_suggestion::tag_suggestions,
                select::action,
                cores_headers::cors_action,
            ],
        )
        .mount("/db", FileServer::from(cli.path))
        .launch()
        .await?;
    std::fs::remove_file(SETTINGSPATH).unwrap_or_else(|err| {
        panic!("Unable to delete setting file to finsish shutdown. (OS error {err})")
    });
    Ok(())
}
#[get("/source")]
fn get_source() -> String {
    String::from("https://github.com/YukiGirlDev/tag_sorter_db_program")
}
