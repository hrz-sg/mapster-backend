# 🗺️ Mapster Backend

**Rust-powered backend for Mapster** — a smart social travel platform that generates personalized routes, cultural insights, and AR experiences.

---

## ⚙️ Overview

This repository contains the backend service for Mapster, implemented in **Rust**, following a modular multi-crate architecture.

### Architecture Summary

```
mapster-backend/
├── .cargo/                # Cargo configuration (build, aliases, fmt)
├── crates/                # Main workspace modules
│   ├── libs/              # Core libraries used across services
│   │   ├── lib-core/      # Domain models, DB layer, base types, context, errors
│   │   ├── lib-auth/      # Authentication, password hashing, token generation
│   │   ├── lib-tmail/     # Email verification, SMTP sending, templates
│   │   └── lib-utils/     # Environment variables, configuration parsing, helpers
│   ├── services/          # Application-level services
│   │   └── web-server/    # Axum-based HTTP REST API
│   └── tools/             # Developer utilities
│       └── gen-key/       # CLI tool for generating app secrets (JWT, API keys)
│
├── sql/                   # SQL initialization and migration files
├── web-folder/            # Static web assets or mock frontend integration
├── target/                # Build artifacts (ignored by Git)
│
├── Cargo.toml             # Workspace configuration
├── Cargo.lock             # Dependency lock file
├── .gitignore             # Ignored files configuration
└── README.md              # Project documentation
```

---

## 🐘 Database Setup (Docker)

Run PostgreSQL 17 in a container:

```sh
docker run --rm --name pg -p 5433:5432     -e POSTGRES_PASSWORD=welcome     postgres:17
```

Optional — connect to the container:

```sh
docker exec -it -u postgres pg psql
```

---

## 🚀 Development

Hot reload setup using `cargo-watch`.

```sh
# Terminal 1 — start the web server
cargo watch -q -c -w crates/services/web-server/src/    -w crates/libs/ -w .cargo/    -x "run -p web-server"

# Terminal 2 — run quick_dev example
cargo watch -q -c -w crates/services/web-server/examples/    -x "run -p web-server --example quick_dev"
```

---

## 🧪 Unit Tests

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

## 🧰 Tools

### Generate application keys

```sh
cargo run -p gen-key
```

---

## 📦 Manual Run (without watch)

```sh
# Terminal 1 - Start web server
cargo run

# Terminal 2 - Run development example
cargo run --example quick_dev
```

---

## 🧠 Tech Stack

| Layer | Technology |
|-------|-------------|
| **Language** | Rust |
| **Framework** | Axum + Tokio |
| **Database** | PostgreSQL via SQLx / SeaQuery |
| **Email** | Lettre (SMTP async) |
| **Config** | dotenv / lib-utils |
| **Architecture** | Multi-crate, modular microservices |

---

## 🧩 Key Features

- User registration and email verification  
- Password hashing & secure login  
- Config-driven environment setup  
- RESTful API via Axum  
- Modular libraries (`lib-core`, `lib-auth`, `lib-tmail`)  
- Dockerized PostgreSQL support  
- Test automation via `cargo-watch`  

---

## 🛡️ Access & Repository Policy

This repository is **private**.  
Access is restricted to core development and AI/ML research contributors.  
Do not distribute, mirror, or publish this code externally.

---

## 🪄 Developer Notes

- Keep consistent Rust edition (2024) across all crates.  
- Use `cargo fmt` and `cargo clippy` before commits.  
- Prefer async/await patterns and non-blocking DB calls.  
- Follow existing naming conventions (`UserBmc`, `Ctx`, etc.)  
- Sensitive configuration values (SMTP credentials, API keys) must **never** be committed.

---

✅ **Status:** Active Development  
🔒 **Visibility:** Private Internal Use Only  
