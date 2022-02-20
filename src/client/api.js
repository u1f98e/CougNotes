// Sending and receiving data in JSON format using POST method
function get_url(url) {
	var xhr = new XMLHttpRequest();
	xhr.open("GET", url, true);
	xhr.responseType = 'json';
	xhr.onreadystatechange = function() {
		if (xhr.readyState === 4 && xhr.status === 200) {
			callback(true, JSON.parse(xhr.responseText))
		} else {
			callback(false, null)
		}
    };
}

function post_json(url, data, callback) {
	var xhr = new XMLHttpRequest();
	xhr.open("POST", url, true);
	xhr.setRequestHeader("Content-Type", "application/json");
	xhr.onreadystatechange = function () {
		if (xhr.readyState === 4 && xhr.status === 200) {
			callback(true, xhr.responseText);
		}
		else {
			callback(false, null);
		}
	};
	var data = JSON.stringify({"email": "hey@mail.com", "password": "101010"});
	xhr.send(data);
}

function get_student(id, callback) {
	get_url("/get-student/" + id, callback);
}

function get_student_by_sid(student_id, callback) {
	get_url("/get-student-by-sid/" + student_id, callback);
}

function get_student_classes(id, callback) {
	get_url("/get-student-classes/" + id, callback);
}

function get_class(id, callback) {
	get_url("/get-class/" + id, callback);
}

function get_class_from_code(code, callback) {
	get_url("/get-class-by-code/" + code, callback);
}

function get_next_post(class_id, callback) {
	get_url("/get-next-post/" + class_id, callback);
}

function get_next_response(post_id, callback) {
	get_url("/get-next-resp/" + post_id, callback);
}

function reset_posts(callback) {
	get_url("/reset-posts", callback);
}

function create_student(student_id, first_name, last_name, img_url, callback) {
	var data = {
		"id": 0,
		"student_id": student_id,
		"first_name": first_name,
		"last_name": last_name,
		"img_url": img_url,
		"classes": []
	};
	post_json("/create-student", data, callback);
}

function create_class(name, section, callback) {
	var data = {
		"id": 0,
		"name": name,
		"section": section,
		"students": [],
	};
	post_json("/create-class", data, callback);
}

function create_post(title, text, img_urls, student_id, class_id, callback) {
	var data = {
		"id": 0,
		"title": title,
		"text": text,
		"img_urls": img_urls,
		"student_id": student_id,
		"class_id": class_id,
	};
	post_json("/create-class", data, callback);
}

function gen_class_code(cid, callback) {
	get_url("/gen-class-code/" + code, callback);
}

function student_add_class(student_id, class_id, callback) {
	var data = [student_id, class_id];
	post_json("/create-class", data, callback);
}