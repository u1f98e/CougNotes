use std::sync::Arc;

use crate::student::Student;

#[derive(Debug)]
pub struct Post {
	id: String,
	text_content: String,
	img_urls: Vec<String>,
	student: Arc<Student>,
	responses: Vec<Post>
}

impl Post {
	
}