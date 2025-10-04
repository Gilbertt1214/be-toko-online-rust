# í»’ Toko Online - Rust Backend API

> High-performance e-commerce backend built with Rust, GraphQL, and PostgreSQL

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![GraphQL](https://img.shields.io/badge/GraphQL-API-E10098?logo=graphql)](https://graphql.org/)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-Database-336791?logo=postgresql)](https://www.postgresql.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A blazingly fast and secure RESTful/GraphQL API backend for e-commerce applications. Built with Rust for maximum performance, safety, and reliability.

## âœ¨ Features

- **í´ Authentication & Authorization** - JWT-based secure authentication
- **í»ï¸ Product Management** - Full CRUD operations for products and categories
- **í»’ Shopping Cart** - Real-time cart management
- **í³¦ Order Processing** - Complete order lifecycle management
- **í²³ Payment Integration** - Secure payment processing
- **â­ Reviews & Ratings** - Customer review system
- **í±¤ User Management** - Comprehensive user profiles and addresses
- **íº€ GraphQL API** - Modern, efficient API with GraphQL
- **í³Š Database Migrations** - Version-controlled schema management
- **í´’ Type-Safe** - Leveraging Rust's type system for maximum safety

## í» ï¸ Tech Stack

- **Language**: Rust í¶€
- **Web Framework**: Actix-web / Warp
- **GraphQL**: Async-GraphQL
- **Database**: PostgreSQL
- **ORM**: SeaORM / Diesel
- **Authentication**: JWT (jsonwebtoken)
- **Serialization**: Serde
- **Password Hashing**: Argon2

## í³‹ Prerequisites

Before running this project, make sure you have:

- Rust 1.70 or higher
- PostgreSQL 14 or higher
- Cargo (comes with Rust)

## íº€ Quick Start

### 1. Clone the repository

```bash
git clone https://github.com/Gilbertt1214/be-toko-online-rust.git
cd be-toko-online-rust
```

### 2. Setup environment variables

Create a `.env` file in the root directory:

```env
DATABASE_URL=postgresql://postgres:password@localhost:5432/toko-online-rust
SECRET_KEY=your-secret-key-here
JWT_SECRET=your-jwt-secret-here
SERVER_HOST=127.0.0.1
SERVER_PORT=8000
```

**âš ï¸ Security Note**: Generate secure secrets using:
```bash
openssl rand -hex 32
```

### 3. Setup the database

```bash
# Create database
createdb toko-online-rust

# Run migrations
cargo run --bin migration
```

### 4. Build and run

```bash
# Development mode
cargo run

# Production mode
cargo build --release
./target/release/be-toko-online-rust
```

The server will start at `http://127.0.0.1:8000` í¾‰

## í³ Project Structure

```
be-toko-online-rust/
â”œâ”€â”€ src/
â”‚   â”œâ”€â”€ db/              # Database connection setup
â”‚   â”‚   â”œâ”€â”€ connection.rs
â”‚   â”‚   â”œâ”€â”€ mod.rs
â”‚   â”‚   â””â”€â”€ seeder.rs
â”‚   â”œâ”€â”€ models/          # Data models
â”‚   â”‚   â”œâ”€â”€ address.rs
â”‚   â”‚   â”œâ”€â”€ cart.rs
â”‚   â”‚   â”œâ”€â”€ cart_item.rs
â”‚   â”‚   â”œâ”€â”€ category.rs
â”‚   â”‚   â”œâ”€â”€ order.rs
â”‚   â”‚   â”œâ”€â”€ order_item.rs
â”‚   â”‚   â”œâ”€â”€ payment.rs
â”‚   â”‚   â”œâ”€â”€ product.rs
â”‚   â”‚   â”œâ”€â”€ review.rs
â”‚   â”‚   â””â”€â”€ user.rs
â”‚   â”œâ”€â”€ services/        # Business logic
â”‚   â”‚   â”œâ”€â”€ auth.rs
â”‚   â”‚   â”œâ”€â”€ cart.rs
â”‚   â”‚   â”œâ”€â”€ order.rs
â”‚   â”‚   â”œâ”€â”€ product.rs
â”‚   â”‚   â””â”€â”€ user.rs
â”‚   â”œâ”€â”€ graphql/         # GraphQL schema & resolvers
â”‚   â”‚   â”œâ”€â”€ mod.rs
â”‚   â”‚   â”œâ”€â”€ query.rs
â”‚   â”‚   â””â”€â”€ mutation.rs
â”‚   â”œâ”€â”€ schema/          # Database schema
â”‚   â”œâ”€â”€ utils/           # Utility functions
â”‚   â””â”€â”€ main.rs          # Application entry point
â”œâ”€â”€ Cargo.toml           # Rust dependencies
â”œâ”€â”€ Cargo.lock
â””â”€â”€ README.md
```

## í´Œ API Endpoints

### GraphQL Playground

Access the GraphQL playground at: `http://127.0.0.1:8000/graphql`

### Example Queries

**Get all products:**
```graphql
query {
  products {
    id
    name
    price
    description
    category {
      name
    }
  }
}
```

**Create user:**
```graphql
mutation {
  createUser(input: {
    name: "John Doe"
    email: "john@example.com"
    password: "securepassword"
  }) {
    id
    name
    email
  }
}
```

**Add to cart:**
```graphql
mutation {
  addToCart(productId: 1, quantity: 2) {
    id
    items {
      product {
        name
      }
      quantity
    }
  }
}
```

## í·ª Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

## í´’ Security Features

- Password hashing with Argon2
- JWT token authentication
- SQL injection prevention via ORM
- CORS configuration
- Request rate limiting
- Input validation and sanitization

## í³Š Database Schema

Key entities:
- **Users** - Customer accounts
- **Products** - Product catalog
- **Categories** - Product categorization
- **Cart** - Shopping cart
- **Orders** - Purchase orders
- **Payments** - Payment transactions
- **Reviews** - Product reviews
- **Addresses** - User addresses

## í´ Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## í³ License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## í±¨â€í²» Author

**Gilbertt1214**
- GitHub: [@Gilbertt1214](https://github.com/Gilbertt1214)

## í¹ Acknowledgments

- Rust community for amazing tools and libraries
- SeaORM/Diesel for excellent ORM solutions
- Actix-web for the fast web framework

---

â­ If you find this project useful, please consider giving it a star!

**Happy Coding! íº€**
