"use strict"

let isInRoom = document.getElementById('isInRoom');

let studentId = document.getElementById('studentId');

function login() {
    const studentId = document.getElementById('studentId');

    console.log(studentId.value);

    if (studentId.value * 10 >= 110000000 && studentId.value * 10 <= 120000000) {
        document.location.href = "home.html";
    }
}