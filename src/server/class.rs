use crate::{post::Post, student::Student};

pub struct Class {
	id: String,
	name: String,
	section: String,
	active_class_codes: Vec<String>,
	posts: Vec<Post>,
}

impl Class {
	
}