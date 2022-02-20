use crate::{post::Post, student::Student};

#[derive(Debug)]
pub struct Class {
	id: String,
	name: String,
	section: String,
	active_class_codes: Vec<String>,
	posts: Vec<Post>,
	students: Vec<Student>
}

impl Class {
	
}