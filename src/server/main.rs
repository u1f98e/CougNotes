use std::path::PathBuf;
use std::{path::Path, sync::Arc};
use std::sync::Mutex;

use async_std;

#[macro_use] extern crate rocket;
use rocket::fs::NamedFile;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::State;

use sqlx::mysql::MySqlPoolOptions as PoolOptions;
use sqlx::mysql::MySqlPool as Pool;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
struct Student {
    uid: u32,
    student_id: u32,
    first_name: String,
    last_name: String,
    img_url: String,
}

#[derive(Serialize, Deserialize)]
struct Class {
    id: u32,
    class_name: String,
    section: String,
}

#[derive(Serialize, Deserialize)]
struct Post {
    id: u32,
    title: String,
    content: String,
    img_url: String,
    student_uid: u32,
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

#[get("/get-student/<uid>")]
async fn get_student(uid: &str, state: &State<AppState>) -> Option<Json<Student>> {
    let student = sqlx::query_as::<_, Student>(
        "
SELECT uid as id, first_name, last_name, img_url, student_id
FROM Students
WHERE uid = ?;
            "
    )
        .bind(uid)
        .fetch_one(&state.pool)
        .await;
    
    match student {
        Ok(s) => Some(Json(s)),
        Err(_) => None
    }
}

#[get("/get-student-by-sid/<student_id>")]
async fn get_student_by_sid(student_id: &str, state: &State<AppState>) -> Option<Json<Student>> {
    let student = sqlx::query_as::<_, Student>(
        "
SELECT uid as id, first_name, last_name, img_url, student_id
FROM Students
WHERE student_id = ?;
            "
    )
        .bind(student_id)
        .fetch_one(&state.pool)
        .await;
    
    match student {
        Ok(s) => Some(Json(s)),
        Err(_) => None
    }
}

#[get("/get-student-classes/<uid>")]
async fn get_student_classes(uid: &str, state: &State<AppState>) -> Option<Json<Vec<Class>>> {
    let student = sqlx::query_as::<_, Student>(
        "
SELECT class_id, class_name, section
FROM 
	(Students JOIN StudentClasses
		ON Students.uid = StudentClasses.student_id)
	JOIN Classes
		On Classes.id = StudentClasses.class_id
WHERE
	Students.uid = 1;
            "
    )
        .bind(uid)
        .fetch_one(&state.pool)
        .await;
    
    match student {
        Ok(s) => Some(Json(s)),
        Err(_) => None
    }
}

#[get("/get-class/<cid>")]
async fn get_class(cid: &str, state: &State<AppState>) -> Option<Json<Class>> {

}

#[get("/get-class-by-code/<code>")]
async fn get_class_from_code(code: &str, state: &State<AppState>) -> Option<Json<Class>> {

}

#[get("/get-next-post/<cid>")]
async fn get_next_post(cid: &str, state: &State<AppState>) -> Option<Json<Post>> {

}

#[get("/get-next-resp/<pid>")]
async fn get_next_response(pid: &str, state: &State<AppState>) -> Option<Json<Post>> {

}

#[get("/reset-posts")]
async fn reset_posts(state: &State<AppState>) -> Json<bool> {
    Json(true)
}

#[post("/create-student", data = "<student>")]
async fn create_student(student: Json<Student>, state: &State<AppState>) -> Json<bool> {

}

#[post("/create-class", data = "<class>")]
async fn create_class(class: Json<Class>, state: &State<AppState>) -> Json<bool> {

}

#[post("/create-post", data = "<post>")]
async fn create_post(post: Json<Post>, state: &State<AppState>) -> Json<bool> {

}

#[get("/gen-class-code/<cid>")]
async fn gen_class_code(cid: u32, state: &State<AppState>) -> Json<String> {

}

#[post("/student_add_class", data = "<sid_cid>")]
async fn student_add_class(sid_cid: Json<[u32; 2]>, state: &State<AppState>) -> Json<bool> {

}

struct AppState {
    pool: Pool,
    last_loaded_post_id: u32
}

#[launch]
async fn launch() -> _ {
    // Create the connection pool for the sql server
    let pool = PoolOptions::new()
        .max_connections(5)
        .connect("localhost:3306").await.unwrap();
    
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
        create_post,
        gen_class_code,
        student_add_class
    ];

    rocket::build()
        .manage(AppState { pool, last_loaded_post_id: 0 })
        .mount("/", routes)
}