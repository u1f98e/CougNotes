"use strict"

import * as api from "./api";
import studentId from "./index";

let formApi = api;

const createRoom = document.getElementById('createRoom');
const roomName = document.getElementById('roomName');
const sectionNumber = document.getElementById('sectionNumber');

const student_uid = studentId;

createRoom.addEventListener('click', () => {
    formApi.gen_class_code(sectionNumber.value, function(){});
    formApi.create_class(roomName.value, sectionNumber.value, function(){});
    formApi.student_add_class(student_uid.value, sectionNumber.value, function(){});
    
    document.location.href = "room.html";
});