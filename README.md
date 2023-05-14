# Rusting Rocket on Diesel (working on docs)

Rusting Rocket on Diesel is a simple REST API built with Rust, Rocket, and Diesel. It provides CRUD operations for a table of students.

## Getting Started

To get started with this project, you will need to have Rust and Cargo installed. Once you have those installed, you can clone the repository and run the following commands to install the dependencies and start the server:

```sh
git clone https://github.com/Goodnessuc/rusting-rocket-diesel.git
cd rusting-rocket-diesel
cargo install diesel_cli
diesel setup
diesel migration run
cargo run
```

This will start the server on port 8000. You can then use a tool like Postman or curl to test the API.

## API Documentation

The following is a list of the API endpoints available in this project:

- `GET /students` - Get a list of all students
- `GET /students/:id` - Get a specific student by ID
- `POST /students` - Create a new student
- `PUT /students/:id` - Update an existing student
- `DELETE /students/:id` - Delete a student

### Example Requests

The following are some example requests that you can use to test the API:

- `GET /students` - Get a list of all students

```
curl http://localhost:8000/students
```

- `GET /students/1` - Get a specific student by ID

```
curl http://localhost:8000/students/1
```

- `POST /students` - Create a new student

```
curl -X POST http://localhost:8000/students -H 'Content-Type: application/json' -d '{"first_name": "John", "last_name": "Doe"}'
```

- `PUT /students/1` - Update an existing student

```
curl -X PUT http://localhost:8000/students/1 -H 'Content-Type: application/json' -d '{"first_name": "Jane", "last_name": "Doe"}'
```

- `DELETE /students/1` - Delete a student

```
curl -X DELETE http://localhost:8000/students/1
```

## Contributing

If you would like to contribute to this project, please feel free to fork the repository and submit a pull request.

## License

This project is licensed under the MIT License.
