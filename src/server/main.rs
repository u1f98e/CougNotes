use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};
use std::{path::Path};

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

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

#[derive(Serialize, Deserialize, FromRow)]
struct Class {
    id: u32,
    class_name: String,
    section: String,
}

#[derive(Serialize, Deserialize, FromRow)]
struct Post {
    id: u32,
    class_id: u32,
    student_uid: u32,
    title: String,
    content: String,
    img_url: String,
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
SELECT uid, first_name, last_name, img_url, student_id
FROM Students
WHERE uid = ?;
            "
    )
        .bind(uid)
        .fetch_one(&state.pool)
        .await;
    
    match student {
        Ok(s) => Some(Json(s)),
        Err(err) => {
            println!("{err}");
            None
        }
    }
}

#[get("/get-student-by-sid/<student_id>")]
async fn get_student_by_sid(student_id: &str, state: &State<AppState>) -> Option<Json<Student>> {
    let student = sqlx::query_as::<_, Student>(
        "
SELECT uid, first_name, last_name, img_url, student_id
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
    let classes = sqlx::query_as::<_, Class>(
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
        .fetch_all(&state.pool)
        .await;
    
    match classes {
        Ok(s) => Some(Json(s)),
        Err(_) => None
    }
}

#[get("/get-class/<cid>")]
async fn get_class(cid: &str, state: &State<AppState>) -> Option<Json<Class>> {
    let id: u32 = match cid.parse::<u32>().ok() {
        Some(n) => n,
        None => return None
    };

    let class = sqlx::query_as::<_, Class>(
        "
SELECT id, class_name, section
FROM Classes
WHERE
	id = ?;
            "
    )
        .bind(id)
        .fetch_one(&state.pool)
        .await;
    
    match class {
        Ok(s) => Some(Json(s)),
        Err(_) => None
    }
}

#[get("/get-class-by-code/<code>")]
async fn get_class_from_code(code: &str, state: &State<AppState>) -> Option<Json<Class>> {
    let class = sqlx::query_as::<_, Class>(
        "
SELECT Classes.id, class_name, section
FROM
	Classes JOIN ClassCodes ON Classes.id = ClassCodes.class_id
WHERE
	ClassCodes.code = ?;
            "
    )
        .bind(code)
        .fetch_one(&state.pool)
        .await;
    
    match class {
        Ok(s) => Some(Json(s)),
        Err(_) => None
    }
}

#[get("/get-next-post/<cid>")]
async fn get_next_post(cid: &str, state: &State<AppState>) -> Option<Json<Post>> {
    let post = sqlx::query_as::<_, Post>(
        "
SELECT id, class_id, student_uid, title, content, img_url
FROM
	Notes
WHERE
	Notes.class_id = ?
    AND (Notes.id > ?);
        "
    )
        .bind(&cid)
        .bind(&state.last_loaded_post_id.load(Ordering::Relaxed))
        .fetch_one(&state.pool)
        .await;

        
    match post {
        Ok(s) => {
            state.last_loaded_post_id.store(s.id, Ordering::Relaxed);
            Some(Json(s))
        },
        Err(_) => None
    }

}

#[get("/get-resp/<pid>")]
async fn get_responses(pid: &str, state: &State<AppState>) -> Option<Json<Vec<Post>>> {
    let post = sqlx::query_as::<_, Post>(
        "
SELECT id, class_id, student_uid, title, content, img_url
FROM
	Notes
WHERE
	Notes.post_id = ?;
        "
    )
        .bind(&pid)
        .fetch_all(&state.pool)
        .await;
    
    match post {
        Ok(s) => Some(Json(s)),
        Err(_) => None
    }
}

#[get("/reset-posts")]
async fn reset_posts(state: &State<AppState>) -> Json<bool> {
    println!("reset posts");
    state.last_loaded_post_id.store(0, Ordering::Relaxed);
    Json(true)
}

#[post("/create-student", data = "<student>")]
async fn create_student(student: Json<Student>, state: &State<AppState>) -> Json<bool> {
    let result = sqlx::query(
        "
INSERT INTO Students (first_name, last_name, img_url, student_id)
    VALUES (?, ?, ?, ?);            
            "
    )
        .bind(&student.first_name)
        .bind(&student.last_name)
        .bind(&student.img_url)
        .bind(&student.student_id)
        .execute(&state.pool)
        .await;
    
    match result {
        Ok(_) => Json(true),
        Err(err) => {
            println!("{err}");
            Json(false)
        }
    }
}

#[post("/create-class", data = "<class>")]
async fn create_class(class: Json<Class>, state: &State<AppState>) -> Json<bool> {
    let result = sqlx::query(
        "
INSERT INTO Classes (class_name, section)
    VALUES (?, ?);            
            "
    )
        .bind(&class.class_name)
        .bind(&class.section)
        .execute(&state.pool)
        .await;
    
    match result {
        Ok(_) => Json(true),
        Err(_) => Json(false)
    }
}

#[post("/create-post", data = "<post>")]
async fn create_post(post: Json<Post>, state: &State<AppState>) -> Json<bool> {
    let result = sqlx::query(
        "
INSERT INTO Notes (class_id, student_uid, title, content, img_url)
    VALUES (?, ?, ?, ?, ?);            
        "
    )
        .bind(&post.class_id)
        .bind(&post.student_uid)
        .bind(&post.title)
        .bind(&post.content)
        .bind(&post.img_url)
        .execute(&state.pool)
        .await;
    
    match result {
        Ok(_) => Json(true),
        Err(_) => Json(false)
    }
}

#[get("/gen-class-code")]
async fn gen_class_code(state: &State<AppState>) -> Json<String> {
    let mut code;
    loop {
        code = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect();

        let result = sqlx::query(
            "
SELECT code FROM ClassCodes WHERE code = ?;
            "
        )
            .bind(&code)
            .fetch_one(&state.pool)
            .await;
        
        match result {
            Ok(_) => (),
            Err(_) => break
        };
    }
    
    Json(code)
}

#[post("/student_add_class", data = "<sid_cid>")]
async fn student_add_class(sid_cid: Json<[u32; 2]>, state: &State<AppState>) -> Json<bool> {
    let result = sqlx::query(
        "
INSERT INTO StudentClasses (student_id, class_id)
    VALUES (?, ?);            
            "
    )
        .bind(&sid_cid[0])
        .bind(&sid_cid[1])
        .execute(&state.pool)
        .await;
    
    match result {
        Ok(_) => Json(true),
        Err(_) => Json(false)
    }
}

struct AppState {
    pool: Pool,
    last_loaded_post_id: AtomicU32
}

#[launch]
async fn launch() -> _ {
    // Create the connection pool for the sql server
    let pool = PoolOptions::new()
        .max_connections(5)
        .connect("sql://localhost:3306/CougNotes").await.unwrap();
    
    let routes = routes![
        index,
        files,
        get_student,
        get_student_by_sid,
        get_student_classes,
        get_class,
        get_class_from_code,
        get_next_post,
        get_responses,
        reset_posts,
        create_student,
        create_class,
        create_post,
        gen_class_code,
        student_add_class
    ];

    rocket::build()
        .manage(AppState { pool, last_loaded_post_id: AtomicU32::new(0) })
        .mount("/", routes)
}