# Rust Backend with SurrealDB

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This is a simple backend project written in Rust. It provides a foundation for building web applications, APIs, or any other backend services in Rust, using SurrealDB as the database.

## Features

- **HTTP Server**: Utilizes the Actix web framework to handle HTTP requests and responses.
- **Database Interaction**: Demonstrates basic CRUD operations with SurrealDB, a distributed and scalable database solution.
- **Async**: Utilizes asynchronous programming features provided by Rust and Actix to handle concurrent requests efficiently.
- **Error Handling**: Implements error handling mechanisms to ensure robustness and reliability.
- **Logging**: Configured with a logging system to track application events and errors.
- **Configuration**: Supports loading configuration from environment variables or configuration files.

## Getting Started

### Prerequisites

- Rust (nightly)
- SurrealDB

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/anilpandey071999/rust_backend.git
   cd rust_backend
   ```

2. Install dependencies:

   ```bash
   cargo build
   ```

### Configuration

1. Configure SurrealDB: Follow the documentation provided by SurrealDB to set up and configure your SurrealDB instance.

2. Set up environment variables:

   - Copy `.env.example` to `.env` and configure the SurrealDB connection details.

### Usage

Start the server:

```bash
cargo run
```

The server should now be running and accessible at `http://localhost:8080`.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
