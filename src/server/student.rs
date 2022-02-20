use std::sync::Arc;
use sqlx::mysql::MySqlPool as Pool;

use rocket::serde::{Serialize, Deserialize};
use serde_json;

use crate::{class::Class, post::Post};

#[derive(Serialize, Deserialize)]
pub struct Student {
	id: String,
	student_id: String,
	first_name: String,
	last_name: String,
	profile_img_url: String,
}