# VieShare - E-commerce API

VieShare is a complete RESTful API system for e-commerce built with Rust. The project uses [SQLite](https://sqlite.org/) database and [Axum](https://github.com/tokio-rs/axum) HTTP framework, with PocketBase-compatible design.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (latest version)
- SQLite (built-in)

## Features

### Core Features
- **User Authentication**: JWT-based authentication with bcrypt password hashing
- **Store Management**: Multi-store support with permission system
- **Product Management**: Complete catalogue with categories/subcategories
- **Shopping Cart**: Session-based and user-based cart management
- **Order Processing**: Complete order processing workflow
- **Customer Management**: Customer management per store

### Technical Features
- **Database**: SQLite with SQLx ORM and migration system
- **Configuration**: Layered configuration with [config-rs](https://github.com/mehcode/config-rs)
- **Logging**: Structured logging with [tracing](https://github.com/tokio-rs/tracing)
- **Error handling**: Custom error types with proper HTTP responses
- **Validation**: Request validation with [validator](https://github.com/Keats/validator)
- **Pagination**: Built-in pagination support
- **OpenAPI**: API documentation with OpenAPI 3.1

## Project Structure

```bash
├── Cargo.toml           # Dependencies and project metadata
├── README.md
├── Makefile            # Build và test commands
├── config/             # Configuration files
│   ├── default.json    # Development configuration
│   ├── production.json # Production settings
│   └── test.json       # Test environment settings
├── db/
│   └── db.rs           # Database connection and utilities
├── migrations/
│   └── 20240901000000_create_vieshare_tables.sql
├── src/
│   ├── main.rs         # Application entry point
│   ├── app.rs          # Axum app setup
│   ├── settings.rs     # Configuration management
│   ├── errors.rs       # Error handling
│   ├── controllers/    # Request handlers
│   │   ├── users.rs    # User management
│   │   ├── stores.rs   # Store management
│   │   ├── categories.rs # Product categories
│   │   └── pocketbase.rs # PocketBase compatibility
│   ├── models/         # Data models
│   │   ├── user.rs     # User model
│   │   ├── auth.rs     # Authentication models
│   │   └── pocketbase.rs # PocketBase-style models
│   ├── forms/          # Request validation
│   │   ├── auth.rs     # Auth form validation
│   │   └── validator.rs # Custom validators
│   ├── routes/         # Route definitions
│   │   ├── status.rs   # Health check endpoints
│   │   └── pocketbase.rs # PocketBase API routes
│   └── utils/          # Shared utilities
│       └── pagination.rs # Pagination helpers
└── openapi.yaml        # API documentation
```

## Database Schema

The database system is designed to be PocketBase-compatible and includes:

- **Users**: User authentication and profiles
- **Stores**: Multi-store support with subscription plans
- **Categories/Subcategories**: Product organization
- **Products**: Product catalog with inventory tracking
- **Carts/Cart Items**: Shopping cart functionality
- **Orders**: Order processing and tracking
- **Addresses**: Customer addresses
- **Customers**: Store-specific customer management
- **Notifications**: Email notification preferences

## Installation and Setup

1. **Clone repository**:
   ```bash
   git clone https://github.com/khieu-dv/vieshare-axum
   cd vieshare-axum
   ```

2. **Install dependencies**:
   ```bash
   cargo build
   ```

3. **Run migrations**:
   ```bash
   # Database will be automatically created and migrated on startup
   ```

4. **Run development server**:
   ```bash
   cargo run
   ```

   Server will run at `http://127.0.0.1:8080`

## Testing

To run tests:
```bash
make test
```

Tests are designed to run sequentially to avoid database state conflicts.

## API Documentation

API documentation is available at `/openapi.yaml`. Main endpoints:

- **Authentication**: `/auth/*` 
- **Users**: `/users/*`
- **Stores**: `/stores/*`
- **Categories**: `/categories/*`
- **Products**: `/products/*`
- **Orders**: `/orders/*`

## Configuration

The project uses a layered configuration system:

- `config/default.json`: Base configuration
- `config/production.json`: Production overrides
- `config/test.json`: Test environment settings

Environment variables can override config values.

## Development

To develop the project:

1. Install Rust toolchain
2. Use `cargo watch` for hot-reload:
   ```bash
   cargo install cargo-watch
   cargo watch -x run
   ```

## Contributing

Contributions are welcome! Please fork the repository and submit pull requests.