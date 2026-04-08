# Mapster Backend

**Rust-powered backend for Mapster** — a smart social travel platform that generates personalized routes, cultural insights, and AR experiences.

---

## Overview

This repository contains the backend service for Mapster, implemented in **Rust**, following a modular multi-crate architecture.

---

## Database Setup (Docker)

Run PostgreSQL 17 in a container:

```sh
docker run --rm --name pg -p 5433:5432     -e POSTGRES_PASSWORD=welcome     postgres:17
```

Optional — connect to the container:

```sh
docker exec -it -u postgres pg psql
```

---

## Development

Hot reload setup using `cargo-watch`.

```sh
# Terminal 1 — start the web server
cargo watch -q -c -w crates/services/web-server/src/    -w crates/libs/ -w .cargo/    -x "run -p web-server"

# Terminal 2 — run quick_dev example
cargo watch -q -c -w crates/services/web-server/examples/    -x "run -p web-server --example auth"
```

---

## Unit Tests

Run all tests:

```sh
cargo test -- --nocapture
```

Watch specific tests:

```sh
cargo watch -q -c -x "test -p lib-core model::post::tests::test_create -- --nocapture"
```

Or filtered test names:

```sh
cargo watch -q -c -x "test model::post::tests::test_c"
```

---

## Tools

### Generate application keys

```sh
cargo run -p gen-key
```

---

## Manual Run (without watch)

```sh
# Terminal 1 - Start web server
cargo run

# Terminal 2 - Run development example
cargo run --example quick_dev
```

---

## Tech Stack

| Layer | Technology |
|-------|-------------|
| **Language** | Rust |
| **Framework** | Axum + Tokio |
| **Database** | PostgreSQL via SQLx / SeaQuery |
| **Email** | Lettre (SMTP async) |
| **Config** | dotenv / lib-utils |
| **Architecture** | Multi-crate, modular microservices |

---

## Key Features

- User registration and email verification  
- Password hashing & secure login  
- Config-driven environment setup  
- RESTful API via Axum  
- Modular libraries (`lib-core`, `lib-auth`, `lib-tmail`)  
- Dockerized PostgreSQL support  
- Test automation via `cargo-watch`  

---

## Access & Repository Policy

This repository is **private**.  
Access is restricted to core development.  
Do not distribute, mirror, or publish this code externally.

---

## Developer Notes

- Keep consistent Rust edition (2024) across all crates.  
- Use `cargo fmt` and `cargo clippy` before commits.  
- Prefer async/await patterns and non-blocking DB calls.  
- Follow existing naming conventions (`UserBmc`, `Ctx`, etc.)  
- Sensitive configuration values (SMTP credentials, API keys) must **never** be committed.

---

**Status:** Active Development  
**Visibility:** Private Internal Use Only  
