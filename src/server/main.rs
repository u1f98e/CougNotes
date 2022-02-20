mod student;
mod class;
mod post;
mod sql;

use std::{path::Path, sync::Arc};

use async_std;
use sqlx::mysql::MySqlPoolOptions as PoolOptions;
use web_server::{self, Request};


#[async_std::main]
async fn main() {
    // Create the connection pool for the sql server
    // let pool = PoolOptions::new()
    //     .max_connections(5)
    //     .connect("").await.unwrap();
    //
    // *sql::sql_pool.lock().unwrap() = Some(pool);

    let server = web_server::new()
        .get("/", Box::new(|request, mut default_response|
            Path::new("src/client/index.html").into()))
        .get("/:page", Box::new(|request, mut default_response| {
            println!("Request: {}", request.params.get("page").unwrap());
            let page = request.params.get("page").unwrap();
            Path::new(&format!("src/client/{page}")).into()
        }))
        .post("/create-student/:json", Box::new(move |request: Request, response| 
            student::from_json(request.params.get("json").unwrap()).into() ));
        
    server.launch(8080); // Run the server
}

