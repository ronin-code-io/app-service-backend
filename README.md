# App Service

## Setup & Building

```bash
export AUTH_SERVICE_HOST="localhost:3000"
cargo install cargo-watch
cargo build
```

## Run service locally (Manually)

```bash
export AUTH_SERVICE_HOST="localhost:3000"
cargo watch -q -c -w src/ -w assets/ -w templates/ -x run
```

visit <http://localhost:8000>

## Run servers locally (Docker)

```bash
export AUTH_SERVICE_HOST="localhost/auth"
docker compose build
docker compose up
```

visit <http://localhost:8000>
