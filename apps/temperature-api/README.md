# Temperature-API

There is the simplest REST API application to return rand temperature value.

## Installation

### Local launching without docker
1. Clone the repository
2. Run `cargo run --bin temperature-api` to build project

### Local launching with docker
1. Cone the repository
2. Run `docker build -t temperature-api:latest .`
3. Run `docker run --name temperatire-api-service -p 8081:8081 temperature-api:latest`
