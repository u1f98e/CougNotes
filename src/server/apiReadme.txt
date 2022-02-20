API
create_class(name, section_name) -> Class
get_class_by_id(id: String) -> Class
add_student(Class, Student)
remove_student(Class, Student)

get_class_by_code(String code) -> Class
gen_class_code(Class) -> code: String

create_student(firstname, lastname, image_url) -> Student
get_student_by_name(String) -> Student
get_student_by_id(String) -> Student

create_post(Class, Student, title: String, text, image(optional))
create_response(Post, Student, text, image(optional))

Student:
get_id()
get_name()
get_profile_img() -> url: String
get_class_list() -> List[id: String]
get_posts() -> List[Post]

Class:
get_id()
get_name()
get_section()
get_posts() -> List[Post]

Post:
get_text() -> String
get_img() -> List[url: String]
get_student() -> Student
get_responses() -> List[Post]
