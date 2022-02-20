CREATE DATABASE CougNotes;
USE CougNotes;

CREATE TABLE Students (
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
	CONSTRAINT `fk_studentUID`  FOREIGN KEY (students_uid) REFERENCES Students (uid)
		ON DELETE CASCADE
		ON UPDATE RESTRICT
	) ENGINE = InnoDB;

CREATE TABLE ClassCodes (
	id SMALLINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	code VARCHAR(100) NOT NULL,
    class_id SMALLINT UNSIGNED NOT NULL,
    CONSTRAINT `fk_classCode` FOREIGN KEY (class_id) REFERENCES ClassCodes (id)
		ON DELETE CASCADE
		ON UPDATE RESTRICT
	) Engine = InnoDB;

CREATE TABLE Classes (
	id SMALLINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	class_name VARCHAR(100) NOT NULL,
	section VARCHAR(10) NOT NULL
    ) Engine = InnoDB;
	

CREATE TABLE StudentClasses (
	id SMALLINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	student_id SMALLINT UNSIGNED NOT NULL,
    class_id SMALLINT UNSIGNED NOT NULL,
    CONSTRAINT `fk_student_id` FOREIGN KEY (student_id) REFERENCES Students (uid)
		ON DELETE CASCADE
        ON UPDATE RESTRICT,
	CONSTRAINT `fk_class_id` FOREIGN KEY (class_id) REFERENCES Classes (id)
		ON DELETE CASCADE
        ON UPDATE RESTRICT
    ) Engine = InnoDB;

SHOW tables;

/*queries below */

/* Create student */
INSERT INTO Students (first_name, last_name, img_url, student_id)
	VALUES ("Dave", "idk", "www.com", 0012345);

INSERT INTO Classes (class_name, section)
	VALUES("Cool class", "aaaa");

SELECT * FROM Students;

/* Get Student by id */
SELECT uid as id, first_name, last_name, img_url, student_id
FROM Students
WHERE uid = 1;

INSERT INTO StudentClasses (student_id, class_id)
	VALUES(1, 1);
    
SELECT * FROM StudentClasses;

SELECT class_id, class_name, section
FROM 
	(Students JOIN StudentClasses
		ON Students.uid = StudentClasses.student_id)
	JOIN Classes
		On Classes.id = StudentClasses.class_id
WHERE
	Students.uid = 1;
    
SELECT class_id, class_name, section
FROM
	Classes JOIN ClassCodes ON Classes.id = ClassCodes.class_id
WHERE
	class_id = ?

/*Return Code from class ID*/
select codes from classCodes INNER JOIN classes ON classCodes.id = classes.id where
classes.id = 2;

/* Return Notes.id from class ID*/
SELECT Notes.id FROM Notes INNER JOIN classes ON Notes.classid = classes.id where classes.id=2;
