use std::path::PathBuf;
use std::{path::Path, sync::Arc};

use async_std;
use rocket::serde::{Serialize, Deserialize};
use sqlx::mysql::MySqlPoolOptions as PoolOptions;

#[macro_use] extern crate rocket;
use rocket::fs::NamedFile;
use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Serialize, Deserialize)]
struct Student {
    id: u32,
    student_id: u32,
    first_name: String,
    last_name: String,
    img_url: String,
    classes: Vec<u32>
}

#[derive(Serialize, Deserialize)]
struct Class {
    id: u32,
    name: String,
    section: String,
    students: Vec<u32>
}

#[derive(Serialize, Deserialize)]
struct Post {
    id: u32,
    title: String,
    text: String,
    img_urls: Vec<String>,
    student_id: u32,
    class_id: u32
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("src/client/index.html")).await.ok()
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    println!("{:?}", file);
    NamedFile::open(Path::new("src/client/").join(file)).await.ok()
}

#[get("/get-student/<id>")]
async fn get_student(id: String) -> Json<Student> {

}

#[get("/get-student-by-sid/<student_id>")]
async fn get_student_by_sid(student_id: String) -> Json<Student> {

}

#[get("/get-student-classes/<id>")]
async fn get_student_classes(id: String) -> Json<Vec<Class>> {

}

#[get("/get-class/<cid>")]
async fn get_class(cid: String) -> Json<Class> {

}

#[get("/get-class-by-code/<code>")]
async fn get_class_from_code(code: String) -> Json<Class> {

}

#[get("/get-next-post/<cid>")]
async fn get_next_post(cid: String) -> Json<Post> {

}

#[get("/get-next-resp/<pid>")]
async fn get_next_response(pid: String) -> Json<Post> {

}

#[get("/reset-posts")]
async fn reset_posts() -> Json<bool> {
    true
}

#[post("/create-student", data = "<student>")]
async fn create_student(student: Json<Student>) -> Json<bool> {

}

#[post("/create-class", data = "<class>")]
async fn create_class(class: Json<Class>) -> Json<bool> {

}

#[post("/create-post", data = "<post>")]
async fn create_post(post: Json<Post>) -> Json<bool> {

}

#[get("/gen-class-code/<cid>")]
async fn gen_class_code(cid: u32) -> Json<String> {
    
}

#[post("/student_add_class", data = "<sid_cid>")]
async fn student_add_class(sid_cid: [u32; 2]) -> Json<bool> {
    
}

#[launch]
fn launch() -> _ {
    // Create the connection pool for the sql server
    let pool = PoolOptions::new()
        .max_connections(5)
        .connect("localhost:3306").await.unwrap();

    *sql::sql_pool.lock().unwrap() = Some(pool);
    
    let routes = routes![
        get_student,
        get_student_by_sid,
        get_student_classes,
        get_class,
        get_class_from_code,
        get_next_post,
        get_next_response,
        reset_posts,
        create_student,
        create_class,
        gen_class_code,
        create_post
    ];

    rocket::build().mount("/", routes)
}