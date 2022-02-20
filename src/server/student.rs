use std::sync::Arc;
use sqlx::mysql::MySqlPool as Pool;

use serde::{Serialize, Deserialize};
use serde_json;

use crate::{class::Class, post::Post};

#[derive(Serialize, Deserialize)]
pub struct StudentRec {
	student_id: String,
	first_name: String,
	last_name: String,
}

#[derive(Debug)]
pub struct Student {
	id: String,
	student_id: String,
	first_name: String,
	last_name: String,
	profile_img_url: String,
	classes: Vec<Arc<Class>>,
	posts: Vec<Arc<Post>>
}

pub fn from_json(json: &str) -> &str {
	let rec: StudentRec = serde_json::from_str(json).unwrap();
	"sucess"
}

impl Student {
}