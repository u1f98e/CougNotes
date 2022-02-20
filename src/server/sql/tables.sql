CREATE DATABASE CougNotes;
USE CougNotes;

CREATE TABLE students (
	uid SMALLINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	first_name VARCHAR(200) NOT NULL,
	last_name  VARCHAR(200) NOT NULL,
	img_url TEXT,
	bio text,
	student_id SMALLINT UNSIGNED NOT NULL
	) ENGINE = InnoDB;

CREATE TABLE Notes (
	id SMALLINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	class_id SMALLINT UNSIGNED NOT NULL,
	content text, 
	img_url text,
	title VARCHAR(200), 
	students_uid SMALLINT UNSIGNED NOT NULL,
	CONSTRAINT `fk_studentUID`  FOREIGN KEY (students_uid) REFERENCES students (uid)
		ON DELETE CASCADE
		ON UPDATE RESTRICT
	) ENGINE = InnoDB;

CREATE TABLE classCodes (
	id SMALLINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    class_id SMALLINT UNSIGNED NOT NULL,
	codes VARCHAR(100) NOT NULL
);


CREATE TABLE classes (
	id SMALLINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	class_name VARCHAR(100) NOT NULL,
	section VARCHAR(10) NOT NULL,
	classCodes_id SMALLINT UNSIGNED NOT NULL,
	CONSTRAINT `fk_classCode` FOREIGN KEY (classCodes_id) REFERENCES classCodes (id)
		ON DELETE CASCADE
		ON UPDATE RESTRICT
	) Engine = InnoDB;

SHOW tables;


/*queries below */

/* Create student */
INSERT INTO Students (uid, firstName, lastName, profilePic, studentID)
	VALUES ();

/* Get Student by id */
SELECT uid as id, first_name, last_name, img_url, student_id
FROM Students
WHERE uid = 2;

/*Return Code from class ID*/
select codes from classCodes INNER JOIN classes ON classCodes.id = classes.id where
classes.id = 2;

/* Return Notes.id from class ID*/
SELECT Notes.id FROM Notes INNER JOIN classes ON Notes.classid = classes.id where classes.id=2;



