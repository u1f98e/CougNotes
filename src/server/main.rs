mod student;
mod class;
mod post;
mod sql;

use std::path::PathBuf;
use std::{path::Path, sync::Arc};

use async_std;
use sqlx::mysql::MySqlPoolOptions as PoolOptions;

#[macro_use] extern crate rocket;
use rocket::fs::NamedFile;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("src/client/index.html")).await.ok()
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    println!("{:?}", file);
    NamedFile::open(Path::new("src/client/").join(file)).await.ok()
}

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![index, files])
}