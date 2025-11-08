# Nexum

[![Rust](https://img.shields.io/badge/rust-1.78+-orange?style=flat-square)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/github/actions/workflow/status/Evans-Prah/nexum/rust.yml?branch=main&style=flat-square)](https://github.com/Evans-Prah/nexum/actions)
[![Crates.io](https://img.shields.io/crates/v/your-crate-name?style=flat-square)](https://crates.io/)

**Nexum** is a production-ready backend service for managing newsletter subscriptions and notifications, built in **Rust**. The project focuses on writing maintainable, tested, and production-grade Rust code while following modern backend best practices.

---

## Project Overview

Nexum provides:

- Subscription management (subscribe/unsubscribe users)
- Health check endpoints
- Email confirmation flow
- Integration with PostgreSQL for data persistence
- Dockerized environment for easy deployment

---

## Tech Stack

- **Rust** – language for backend development
- **Actix-web / Axum** – web framework
- **SQLx** – async database library
- **Serde / Serde JSON** – for serialization
- **PostgreSQL** – relational database
- **Docker** – containerization
- **GitHub Actions** – CI/CD (future)

---

## Getting Started

### Prerequisites

- Rust toolchain (`rustup`, `cargo`)
- Docker & Docker Compose (for database setup)
- PostgreSQL (or containerized via Docker)

### Run Locally

```bash
git clone https://github.com/Evans-Prah/nexum.git
cd nexum
cargo run
