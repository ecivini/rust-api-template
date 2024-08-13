# Rust API Template

![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.67%2B-orange.svg)

## Overview

This project provides simple and scalable template for building RESTful APIs using Rust. It is designed to streamline the development of APIs by offering a well-structured starting point with best practices for code organization, error handling, and testing.

### Libraries

This template is based on multiple libraries:

- warp: composable web framework
- log: logging information (and currently with colog as implementation)
- dotenv: read environment variables from .env

## Getting Started

### Prerequisites

- **Rust**: Ensure you have Rust installed. You can install it from [rustup.rs](https://rustup.rs/).
- **Cargo**: Cargo, the Rust package manager, should be installed with Rust.

### Installation

1. **Clone the repository**:

    ```bash
    git clone https://github.com/ecivini/rust-api-template.git
    cd rust-api-template
    ```

2. **Build the project**:

    ```bash
    cargo build
    ```

3. **Run the tests**:

    ```bash
    cargo test
    ```

4. **Run the application**:

    ```bash
    cargo run
    ```

### Configuration

Configuration is handled via environment variables. You can set up a `.env` file at the root of the project to manage your configurations:

```env
# .env file example

DATABASE_URL=postgres://user:password@localhost:5432/mydatabase
API_PORT=8080
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request with any improvements, bug fixes, or new features.

## License

This project is licensed under the Apache 2.0 License. See the [LICENSE.md](LICENSE) file for details.
