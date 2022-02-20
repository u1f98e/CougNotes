# CougNotes


The collaborative Note sharing App for Cougs, by Cougs!

Between COVID quarantines and inclement weather, through no fault of their own Cougars have been missing class more than ever. Sometimes, professors do not, or are not able to provide zoom feeds during lectures and this can cause students to fall behind just because of one positive COVID test, or one pass closed due to weather. Enter CougNotes - a collaborative note sharing platform where students can post their lecture notes to help each other learn. With CougNotes, either professors or students can create a classroom page (called a “room”) as a central repository for notes. An alphanumeric access key is then used to log in to the page, and any student can then upload their notes as a text file, pdf, or image. After a student has logged into a class once, it appears in a “classes” list on their profile - allowing for quick access in the future. 

# Project Architecture

<b> Database: </b>

A MariaDB database forms the foundation of the CougNotes backend API. 5 tables provide storage for student accounts, classes, notes, a mapping of students to classes, and a mapping of classes to the alphanumeric class codes used to access CougNotes rooms. A provisioning script (tables.sql) is provided, which stands up a CougNotes database prepopulated with all necessary tables. Default display queries used by the API are also included, and available as templates for future extensions. 

<b> API & Backend: </b>

CougNotes provides an API as a bridge between the database interface code (written in Rust) and the javascript runtime API which ties into the website. The API converts the JSON files used by the website into HTTP POST and GET requests that are then converted into SQL commands and queries by the backend. 

<b> UI/Frontend: </b>
All visual elements of the UI were written in HTML and styled with CSS. Interactive elements use JavaScript and call functions from the CougNotes API. 

