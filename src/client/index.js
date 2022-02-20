"use strict"

import * as api from './api.js';

const studentApi = api;

const loginBtn = document.getElementById('login');

loginBtn.addEventListener('click', login);

export const studentId = document.getElementById('studentId');

function login() {

    console.log(studentId.value);

    if (studentId.value * 10 >= 110000000 && studentId.value * 10 <= 120000000) {
        let isRegistered = studentApi.get_student_by_sid(studentId.value, () => {
            return 1;
        })
        if (!isRegistered) {
            studentApi.create_student(studentId.value, () => {
                let alphabet = 'abcdefghijklmnopqrstuvwxyz';
                let firstName;
                for (let i = 0; i < Math.floor(Math.random()) * 5 + 2; ++i) {
                    firstName += alphabet.charAt(Math.floor(Math.random()) * 26);
                }
                return firstName;
            }, () => {
                let alphabet = 'abcdefghijklmnopqrstuvwxyz';
                let lastName;
                for (let i = 0; i < Math.floor(Math.random()) * 5 + 2; ++i) {
                    lastName += alphabet.charAt(Math.floor(Math.random()) * 26);
                }
                return lastName;
            }, './img/profile.png', () => {
                return 1;
            })
        }

        document.location.href = "home.html";
    }
}