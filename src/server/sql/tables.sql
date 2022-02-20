CREATE DATABASE CougNotes;
USE CougNotes;

CREATE TABLE students (
	uid SMALLINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	firstName VARCHAR(200) NOT NULL,
	lastName  VARCHAR(200) NOT NULL,
	profilePic TEXT,
	bio text,
	studentID SMALLINT UNSIGNED NOT NULL
	) ENGINE = InnoDB;

CREATE TABLE Notes (
	id SMALLINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	classid SMALLINT UNSIGNED NOT NULL,
	content text, 
	imageURLs text,
	title VARCHAR(200), 
	students_uid SMALLINT UNSIGNED NOT NULL,
	CONSTRAINT `fk_studentUID`  FOREIGN KEY (students_uid) REFERENCES students (uid)
		ON DELETE CASCADE
		ON UPDATE RESTRICT
	) ENGINE = InnoDB;

CREATE TABLE classCodes (
	id SMALLINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	codes VARCHAR(100) NOT NULL
);


CREATE TABLE classes (
	id SMALLINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	className VARCHAR(100) NOT NULL,
	section VARCHAR(10) NOT NULL,
	classCodes_id SMALLINT UNSIGNED NOT NULL,
	CONSTRAINT `fk_classCode` FOREIGN KEY (classCodes_id) REFERENCES classCodes (id)
		ON DELETE CASCADE
		ON UPDATE RESTRICT
	) Engine = InnoDB;

SHOW tables;


/*queries below */

/*Return UID from class ID*/
select codes from classCodes INNER JOIN classes ON classCodes.id = classes.id where
classes.id = 2;

/* Return Notes.id from class ID*/
SELECT Notes.id FROM Notes INNER JOIN classes ON Notes.classid = classes.id where classes.id=2;


