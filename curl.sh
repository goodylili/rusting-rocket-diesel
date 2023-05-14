#!/bin/bash

# Get all students
curl http://localhost:8000/students

# Delete a student with ID 1
curl -X DELETE http://localhost:8000/students/1

# Create a new student with first name "John", last name "Doe" and age 17
curl -X POST http://localhost:8000/student -H 'Content-Type: application/json' -d '{"first_name": "John", "last_name": "Doe", "age": 17}'

# Update a student with ID 1 with first name "Jane", last name "Doe" and age 18
curl -X PUT http://localhost:8000/students/1 -H 'Content-Type: application/json' -d '{"first_name": "Jane", "last_name": "Doe", "age": 18}'
