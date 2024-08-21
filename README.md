

---

# Actix-Web Rust Server

## Overview

This project is a simple web server built with Actix-web, a powerful and flexible web framework for Rust. The server demonstrates basic CRUD operations, including handling JSON input and generating random values. It also includes logging functionality to track requests.

## Features

- **Greeting Endpoint**: Returns a welcome message.
- **Echo Endpoint**: Echoes back a JSON message provided in the request.
- **Reverse Endpoint**: Reverses a string provided in the request.
- **Random Endpoint**: Generates a random number and returns it in a JSON response.
- **Request Logging**: Logs request details using Actix-web's logging middleware.

## Prerequisites

- Rust (1.70.0 or later)
- Cargo (Rust's package manager)

## Getting Started

### Clone the Repository

```sh
git clone https://github.com/yourusername/your-repo.git
cd your-repo
```

### Install Dependencies

Make sure you have Rust and Cargo installed, then run:

```sh
cargo build
```

This will download and compile the dependencies.

### Run the Server

To start the server, use:

```sh
cargo run
```

The server will start and listen on `http://127.0.0.1:8080`.

### Endpoints

- **GET /**: Returns a welcome message.
  - **Response**: `200 OK` with body `"Welcome to the Rust Web server!"`

- **POST /echo**: Echoes back a JSON message.
  - **Request Body**:
    ```json
    {
      "id": 1,
      "name": "John Doe",
      "rollno": 1234,
      "marks": 85,
      "grade": "A"
    }
    ```
  - **Response**: `200 OK` with the same JSON message

- **POST /reverse**: Reverses a string provided in a JSON message.
  - **Request Body**:
    ```json
    {
      "text": "hello"
    }
    ```
  - **Response**: `200 OK` with body `{"text": "olleh"}`

- **GET /random**: Generates and returns a random number.
  - **Response**: `200 OK` with a JSON body containing the random number

### Logging

The server logs request details using Actix-web's logging middleware. To see the logs:

1. Ensure you have set the logging environment variable. You can configure this in your shell or a `.env` file.

   ```sh
   export RUST_LOG=actix_web=info
   ```

2. Run the server and observe the logs in your console.

## Dependencies

- `actix-web`: Web framework for Rust
- `serde`: Serialization and deserialization framework
- `rand`: Library for generating random numbers
- `env_logger`: Logger initialization for environment-based configuration

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.


