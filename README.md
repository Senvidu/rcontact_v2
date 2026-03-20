# 📇 Contact Management Service API

A RESTful Contact Management Service built with **Rust**, **Axum**,
**PostgreSQL**, and **SeaORM**.

------------------------------------------------------------------------

## 🏗️ Architecture

-   Handler → HTTP handling\
-   Service → Business logic\
-   Repository → Database access\
-   Entity → Models\
-   Route → API routes

------------------------------------------------------------------------

## ⚙️ Tech Stack

-   Rust
-   Axum
-   PostgreSQL
-   SeaORM
-   Tokio
-   Serde
-   dotenvy

------------------------------------------------------------------------

## 🚀 Features

-   CRUD for persons
-   CRUD for mobiles
-   CRUD for emails
-   Health checks
-   PostgreSQL integration
-   SeaORM migrations

------------------------------------------------------------------------

## 🌐 API Endpoints

### Health

    GET /livez
    GET /readyz

### Persons

    GET    /api/v1/persons
    POST   /api/v1/persons
    GET    /api/v1/persons/{id}
    PUT    /api/v1/persons/{id}
    DELETE /api/v1/persons/{id}

------------------------------------------------------------------------

## 🗄️ Database

-   persons
-   mobiles
-   emails

------------------------------------------------------------------------

## 🔧 Setup

### 1. Create database

``` bash
createdb -p 5433 contact_management
```

------------------------------------------------------------------------

### 2. Run migrations

``` bash
DATABASE_URL=postgres://YOUR_USERNAME@localhost:5433/contact_management sea-orm-cli migrate up
```

### Reset migrations (optional)

``` bash
DATABASE_URL=postgres://YOUR_USERNAME@localhost:5433/contact_management sea-orm-cli migrate refresh
```

------------------------------------------------------------------------

### 3. Run project

``` bash
cargo run
```

Server: http://127.0.0.1:3000

------------------------------------------------------------------------

## ⚠️ Common Issues

### Port already in use

``` bash
lsof -i :3000
kill -9 <PID>
```

### Tables not found

Run migrations again:

``` bash
sea-orm-cli migrate up
```

------------------------------------------------------------------------

## 👨‍💻 Author

Chanithu Senvidu
