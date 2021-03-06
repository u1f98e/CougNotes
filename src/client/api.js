function log_callback (status, data) {
	console.log(status);
	console.log(data);
}

function test() {
	get_student(1, log_callback);
	// get_student_by_sid(00123, log_callback);
	// create_student(12345, "tom", "aaa", "aweffa", log_callback);
	// get_class(1, log_callback);
}

// Sending and receiving data in JSON format using POST method
export function get_url(url) {
	var xhr = new XMLHttpRequest();
	xhr.open("GET", url, true);
	xhr.responseType = 'text';
	xhr.onreadystatechange = function() {
		if (xhr.readyState === 4 && xhr.status === 200) {
			callback(xhr.status, JSON.parse(xhr.responseText))
		}
    };
	xhr.send();
}

export function post_json(url, data, callback) {
	var xhr = new XMLHttpRequest();
	xhr.open("POST", url, true);
	xhr.setRequestHeader("Content-Type", "application/json");
	xhr.onreadystatechange = function () {
		if (xhr.readyState === 4 && xhr.status === 200) {
			callback(xhr.status, JSON.parse(xhr.responseText));
		}
	};
	xhr.send(JSON.stringify(data));
}

export function get_student(uid, callback) {
	get_url("/get-student/" + uid, callback);
}

export function get_student_by_sid(student_uid, callback) {
	get_url("/get-student-by-sid/" + student_uid, callback);
}

export function get_student_classes(uid, callback) {
	get_url("/get-student-classes/" + uid, callback);
}

export function get_class(id, callback) {
	get_url("/get-class/" + id, callback);
}

export function get_class_from_code(code, callback) {
	get_url("/get-class-by-code/" + code, callback);
}

export function get_next_post(class_id, callback) {
	get_url("/get-next-post/" + class_id, callback);
}

export function get_next_response(post_id, callback) {
	get_url("/get-next-resp/" + post_id, callback);
}

export function reset_posts(callback) {
	get_url("/reset-posts", callback);
}

export function create_student(student_uid, first_name, last_name, img_url, callback) {
	var data = {
		"uid": 0,
		"student_id": student_id,
		"first_name": first_name,
		"last_name": last_name,
		"img_url": img_url,
	};
	post_json("/create-student", data, callback);
}

export function create_class(name, section, callback) {
	var data = {
		"id": 0,
		"class_name": class_name,
		"section": section,
	};
	post_json("/create-class", data, callback);
}

export function create_post(title, content, img_url, student_uid, class_id, callback) {
	var data = {
		"id": 0,
		"title": title,
		"content": content,
		"img_urls": img_url,
		"student_uid": student_uid,
		"class_id": class_id,
	};
	post_json("/create-class", data, callback);
}

export function gen_class_code(cid, callback) {
	get_url("/gen-class-code/" + cid, callback);
}

export function student_add_class(student_uid, class_id, callback) {
	var data = [student_uid, class_id];
	post_json("/create-class", data, callback);
}