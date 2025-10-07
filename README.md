# 🛒 Toko Online NUVELLA

<div align="center">

![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg?style=for-the-badge&logo=rust)
![Axum](https://img.shields.io/badge/Axum-0.7-6B46C1?style=for-the-badge)
![SeaORM](https://img.shields.io/badge/SeaORM-1.0-00BCD4?style=for-the-badge)
![GraphQL](https://img.shields.io/badge/GraphQL-API-E10098?style=for-the-badge&logo=graphql)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-16-336791?style=for-the-badge&logo=postgresql)
![Xendit](https://img.shields.io/badge/Xendit-Payment-00BFA5?style=for-the-badge)

**Backend API E-Commerce Modern dengan Rust + Axum + GraphQL + Seaorm + Xendit**

*Blazingly fast, type-safe, dan production-ready*

[🚀 Mulai Cepat](#-instalasi-cepat) • [📖 Dokumentasi](#-dokumentasi-api) • [🎯 Roadmap](#-roadmap)

</div>

---

## ✨ Fitur Unggulan

<table>
<tr>
<td width="50%">

### 🔐 Keamanan & Autentikasi
- ✅ JWT Authentication dengan refresh token
- ✅ Password hashing Argon2id
- ✅ Role-based Access Control (Admin/User)
- ✅ Session management yang aman
- ✅ Rate limiting untuk API protection

### 🛍️ Manajemen Produk
- ✅ CRUD produk lengkap
- ✅ Kategori & subkategori
- ✅ Upload gambar produk
- ✅ Tracking stok real-time
- ✅ Pencarian & filter produk
- ✅ Produk rekomendasi

### 🛒 Keranjang Belanja
- ✅ Keranjang persisten per user
- ✅ Update quantity real-time
- ✅ Validasi stok otomatis
- ✅ Kalkulasi total otomatis
- ✅ Wishlist functionality

</td>
<td width="50%">

### 📦 Manajemen Pesanan
- ✅ Complete order lifecycle
- ✅ Status tracking (Pending → Completed)
- ✅ Order history
- ✅ Invoice generation
- ✅ Notification system

### 💳 Payment Gateway (Xendit)
- ✅ Multiple payment methods
  - 💳 Credit Card / Debit Card
  - 🏦 Virtual Account (BCA, Mandiri, BNI, BRI)
  - 🏪 E-Wallet (OVO, Dana, LinkAja, ShopeePay)
  - 🏬 Retail Outlet (Alfamart, Indomaret)
- ✅ Webhook handling
- ✅ Payment verification
- ✅ Refund support
- ✅ Transaction history

### ⭐ Engagement Pelanggan
- ✅ Review & rating produk
- ✅ User profile management
- ✅ Multiple shipping address
- ✅ Order notifications
- ✅ Email notifications

</td>
</tr>
</table>

---

## 🏗️ Arsitektur Teknologi

```
┌─────────────────────────────────────────────────────────────┐
│                    🌐 Client Layer                           │
│          (React/Vue/Flutter + GraphQL Client)                │
└────────────────────┬────────────────────────────────────────┘
                     │ GraphQL Queries/Mutations
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                  🚀 API Gateway (Axum)                       │
│                                                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   GraphQL    │  │     REST     │  │   Webhook    │      │
│  │   Endpoint   │  │   /health    │  │   /xendit    │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              📊 GraphQL Layer (async-graphql)                │
│                                                               │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  Query Resolvers  │  Mutation Resolvers  │  Types   │    │
│  └─────────────────────────────────────────────────────┘    │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                  💼 Business Logic Layer                     │
│                                                               │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌──────────┐          │
│  │  Auth   │ │ Product │ │  Cart   │ │  Order   │          │
│  │ Service │ │ Service │ │ Service │ │ Service  │          │
│  └─────────┘ └─────────┘ └─────────┘ └──────────┘          │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌──────────┐          │
│  │ Payment │ │  User   │ │ Review  │ │ Category │          │
│  │ Service │ │ Service │ │ Service │ │ Service  │          │
│  └─────────┘ └─────────┘ └─────────┘ └──────────┘          │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              🗄️ Data Access Layer (SeaORM)                  │
│                                                               │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Entity Models  │  Migrations  │  Connection Pool    │   │
│  └──────────────────────────────────────────────────────┘   │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                  🗃️ PostgreSQL Database                     │
│                                                               │
│  Users │ Products │ Orders │ Payments │ Reviews │ etc.      │
└─────────────────────────────────────────────────────────────┘

        ┌────────────────────────────────────┐
        │    🔌 External Services            │
        │                                    │
        │  • Xendit Payment Gateway          │
        │  • Email Service (SMTP)            │
        │  • Cloud Storage (Optional)        │
        └────────────────────────────────────┘
```

---

## 📁 Struktur Folder

```
be-toko-online-rust/
├── 📂 src/
│   ├── 📂 config/          # Konfigurasi aplikasi
│   │   ├── app.rs          # Config utama
│   │   ├── mod.rs
│   │   └── xendit.rs       # Xendit configuration
│   │
│   ├── 📂 db/              # Database setup
│   │   ├── connection.rs   # Pool connection
│   │   ├── seeder.rs       # Data seeder
│   │   └── mod.rs
│   │
│   ├── 📂 graphql/         # GraphQL layer
│   │   ├── graphql_types.rs  # Custom types
│   │   ├── mod.rs
│   │   ├── mutation.rs     # Mutation resolvers
│   │   └── query.rs        # Query resolvers
│   │
│   ├── 📂 handlers/        # HTTP handlers
│   │   ├── mod.rs
│   │   ├── status.rs       # Health check handler
│   │   └── webhook.rs      # Xendit webhook handler
│   │
│   ├── 📂 models/          # Data models (SeaORM entities)
│   │   ├── address.rs
│   │   ├── cart.rs
│   │   ├── cart_item.rs
│   │   ├── category.rs
│   │   ├── order.rs
│   │   ├── order_item.rs
│   │   ├── payment.rs
│   │   ├── product.rs
│   │   ├── review.rs
│   │   └── user.rs
│   │
│   ├── 📂 scalars/         # Custom GraphQL scalars
│   │   ├── datetime.rs     # DateTime scalar
│   │   └── mod.rs
│   │
│   ├── 📂 schema/          # Database schema
│   │   └── mod.rs
│   │
│   ├── 📂 services/        # Business logic
│   │   ├── auth.rs         # Authentication
│   │   ├── cart.rs         # Cart management
│   │   ├── category.rs     # Category service
│   │   ├── order.rs        # Order processing
│   │   ├── payment.rs      # Payment (Xendit)
│   │   ├── product.rs      # Product CRUD
│   │   ├── review.rs       # Review service
│   │   └── user.rs         # User management
│   │
│   └── main.rs             # Entry point
│
├── 📂 migration/           # Database migrations
│   ├── src/
│   │   ├── m20240101_create_users.rs
│   │   ├── m20240102_create_products.rs
│   │   ├── m20240103_create_categories.rs
│   │   ├── m20240104_create_orders.rs
│   │   ├── m20240105_create_payments.rs
│   │   └── lib.rs
│   └── Cargo.toml
│
├── 📂 templates/           # HTML templates
│   ├── index.html          # Landing page
│   ├── playground.html     # Apollo Sandbox
│   └── webhook_info.html   # Webhook docs
│
├── 📄 .env                 # Environment variables
├── 📄 .gitignore
├── 📄 Cargo.toml           # Dependencies
├── 📄 Cargo.lock
├── 📄 README.md
└── 📄 docker-compose.yml   # Docker setup
```

---

## 🚀 Instalasi Cepat

### Prerequisites

Pastikan sudah terinstall:

- **Rust** 1.70+ ([Install Rust](https://rustup.rs/))
- **PostgreSQL** 14+ ([Download PostgreSQL](https://www.postgresql.org/download/))
- **Cargo** (otomatis dengan Rust)
- **SeaORM CLI** (untuk migrations)

### Langkah-langkah Instalasi

#### 1️⃣ Clone Repository

```bash
git clone https://github.com/Gilbertt1214/be-toko-online-rust.git
cd be-toko-online-rust
```

#### 2️⃣ Setup Environment Variables

Buat file `.env` di root project:

```env
# Database Configuration
DATABASE_URL=postgresql://postgres:password@localhost:5432/toko_online_nuvella

# Server Configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=8000

# Security Keys (GENERATE BARU!)
SECRET_KEY=your-secret-key-32-characters-minimum
JWT_SECRET=your-jwt-secret-32-characters-minimum

# Xendit Configuration
XENDIT_API_KEY=xnd_development_your_key_here
XENDIT_WEBHOOK_TOKEN=your_webhook_verification_token
XENDIT_CALLBACK_URL=https://yourdomain.com/webhook/xendit

# Email Configuration (Optional)
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password

# Logging
RUST_LOG=info,sqlx=warn
```

**🔐 Generate Secret Keys:**

```bash
# Install openssl jika belum ada
# Generate SECRET_KEY
openssl rand -hex 32

# Generate JWT_SECRET
openssl rand -hex 32
```

**💳 Xendit Setup:**

1. Daftar di [Xendit Dashboard](https://dashboard.xendit.co/)
2. Dapatkan **API Key** dari Settings → Developers
3. Setup **Webhook URL** untuk notifikasi pembayaran
4. Simpan **Webhook Verification Token**

#### 3️⃣ Setup Database

```bash
# Buat database baru
createdb toko_online_nuvella

# Atau menggunakan psql
psql -U postgres
CREATE DATABASE toko_online_nuvella;
\q
```

#### 4️⃣ Install SeaORM CLI & Run Migrations

```bash
# Install SeaORM CLI
cargo install sea-orm-cli

# Jalankan migrations
cd migration
sea-orm-cli migrate up

# Atau langsung dari root
sea-orm-cli migrate up -d ./migration
```

#### 5️⃣ Build & Run

**Development Mode:**

```bash
# Run dengan auto-reload (install cargo-watch)
cargo install cargo-watch
cargo watch -x run

# Atau run biasa
cargo run
```

**Production Mode:**

```bash
# Build optimized binary
cargo build --release

# Run binary
./target/release/be-toko-online-rust
```

#### 6️⃣ Verifikasi Instalasi

Server berjalan di `http://127.0.0.1:8000` 🎉

**Test endpoints:**

```bash
# Health check
curl http://127.0.0.1:8000/health

# Buka Apollo Sandbox
open http://127.0.0.1:8000/graphql
```

---

## 🔄 Flow Diagram Development

### 🎯 Alur Request Processing

```mermaid
graph TB
    A[Client Request] -->|HTTP/HTTPS| B[Axum Router]
    B -->|GraphQL Query/Mutation| C[GraphQL Handler]
    B -->|REST Endpoint| D[REST Handler]
    B -->|Xendit Webhook| E[Webhook Handler]
    
    C --> F[Middleware Layer]
    D --> F
    E --> F
    
    F -->|Auth Check| G[JWT Validation]
    G -->|Valid| H[Service Layer]
    G -->|Invalid| Z[401 Unauthorized]
    
    H --> I{Business Logic}
    
    I -->|Auth Service| J[Register/Login/Logout]
    I -->|Product Service| K[CRUD Products]
    I -->|Cart Service| L[Cart Operations]
    I -->|Order Service| M[Order Processing]
    I -->|Payment Service| N[Xendit Integration]
    
    J --> O[Database Layer]
    K --> O
    L --> O
    M --> O
    N --> O
    
    O -->|SeaORM| P[(PostgreSQL)]
    
    N -->|API Call| Q[Xendit API]
    Q -->|Payment Created| R[Payment Response]
    R --> S[Client Redirect]
    
    Q -.->|Webhook| E
    E -->|Verify Signature| T{Valid?}
    T -->|Yes| U[Update Payment Status]
    T -->|No| V[Reject]
    U --> O
    
    P --> W[Response Builder]
    W --> X[JSON Response]
    X --> Y[Client]
```

### 💳 Payment Flow dengan Xendit

```mermaid
sequenceDiagram
    participant C as Client/Frontend
    participant A as Axum API
    participant S as Payment Service
    participant X as Xendit API
    participant W as Webhook
    participant D as Database
    
    C->>A: createOrder(paymentMethod)
    A->>S: Process Order
    S->>D: Save Order (PENDING)
    D-->>S: Order Created
    
    S->>X: Create Invoice/VA/EWallet
    X-->>S: Payment URL/VA Number
    S->>D: Save Payment (PENDING)
    D-->>S: Payment Saved
    S-->>A: Payment Info
    A-->>C: Payment URL/Details
    
    C->>C: User Completes Payment
    
    X->>W: Webhook: payment.paid
    W->>W: Verify Signature
    W->>D: Update Payment (SUCCESS)
    W->>D: Update Order (PAID)
    D-->>W: Updated
    W-->>X: 200 OK
    
    Note over W: Send Email Notification
    W->>C: Push Notification (Optional)
    
    C->>A: checkOrderStatus(orderId)
    A->>D: Get Order Status
    D-->>A: Order Status: PAID
    A-->>C: Order Confirmed
```

### 🔄 User Authentication Flow

```mermaid
sequenceDiagram
    participant C as Client
    participant A as API
    participant Auth as Auth Service
    participant DB as Database
    participant JWT as JWT Service
    
    rect rgb(200, 220, 255)
        Note over C,JWT: Registration Flow
        C->>A: register(email, password, name)
        A->>Auth: Validate Input
        Auth->>DB: Check Email Exists
        DB-->>Auth: Email Available
        Auth->>Auth: Hash Password (Argon2)
        Auth->>DB: Create User
        DB-->>Auth: User Created
        Auth->>JWT: Generate Tokens
        JWT-->>Auth: Access + Refresh Token
        Auth-->>A: User + Tokens
        A-->>C: Success Response
    end
    
    rect rgb(220, 255, 220)
        Note over C,JWT: Login Flow
        C->>A: login(email, password)
        A->>Auth: Validate Credentials
        Auth->>DB: Find User by Email
        DB-->>Auth: User Data
        Auth->>Auth: Verify Password
        Auth->>JWT: Generate Tokens
        JWT-->>Auth: Access + Refresh Token
        Auth-->>A: User + Tokens
        A-->>C: Success + Set Cookies
    end
    
    rect rgb(255, 220, 220)
        Note over C,JWT: Protected Request Flow
        C->>A: getProfile() + JWT
        A->>Auth: Verify Token
        Auth->>JWT: Decode & Validate
        JWT-->>Auth: Valid User ID
        Auth->>DB: Get User Profile
        DB-->>Auth: User Data
        Auth-->>A: Profile Data
        A-->>C: User Profile
    end
```

### 🛒 Shopping Cart to Order Flow

```mermaid
stateDiagram-v2
    [*] --> BrowseProducts: User Membuka Toko
    
    BrowseProducts --> AddToCart: Pilih Produk
    AddToCart --> ViewCart: Lihat Keranjang
    
    ViewCart --> UpdateQuantity: Ubah Jumlah
    UpdateQuantity --> ViewCart
    
    ViewCart --> RemoveItem: Hapus Item
    RemoveItem --> ViewCart
    
    ViewCart --> Checkout: Lanjut Checkout
    
    Checkout --> SelectAddress: Pilih Alamat
    SelectAddress --> SelectPayment: Pilih Payment
    
    SelectPayment --> CreateOrder: Buat Order
    
    CreateOrder --> OrderPending: Order Dibuat
    OrderPending --> PaymentCreated: Generate Payment
    
    PaymentCreated --> WaitingPayment: Redirect ke Payment
    
    WaitingPayment --> PaymentSuccess: User Bayar
    WaitingPayment --> PaymentExpired: Timeout
    
    PaymentSuccess --> OrderPaid: Update Status
    OrderPaid --> OrderProcessing: Mulai Proses
    OrderProcessing --> OrderShipped: Dikirim
    OrderShipped --> OrderDelivered: Sampai
    OrderDelivered --> OrderCompleted: Selesai
    
    PaymentExpired --> OrderCancelled: Order Dibatalkan
    
    OrderCompleted --> [*]
    OrderCancelled --> [*]
```

---

## 📖 Dokumentasi API

### GraphQL Playground

Akses Apollo Sandbox di: **`http://127.0.0.1:8000/graphql`**

Features:
- ✅ Schema documentation otomatis
- ✅ Auto-completion
- ✅ Query history
- ✅ Syntax highlighting
- ✅ Real-time execution

### Endpoint Summary

| Endpoint | Method | Deskripsi |
|----------|--------|-----------|
| `/graphql` | POST | GraphQL API endpoint |
| `/health` | GET | Health check server |
| `/webhook/xendit` | POST | Xendit payment webhook |
| `/` | GET | API information page |

---

## 📝 Contoh Query & Mutation

### 🔐 Authentication

<details>
<summary><b>Register User Baru</b></summary>

```graphql
mutation Register {
  register(input: {
    name: "John Doe"
    email: "john@example.com"
    password: "SecurePass123!"
    phoneNumber: "081234567890"
  }) {
    id
    name
    email
    phoneNumber
    createdAt
  }
}
```
</details>

<details>
<summary><b>Login</b></summary>

```graphql
mutation Login {
  login(input: {
    email: "john@example.com"
    password: "SecurePass123!"
  }) {
    accessToken
    refreshToken
    user {
      id
      name
      email
      role
    }
  }
}
```
</details>

### 🛍️ Products

<details>
<summary><b>Ambil Semua Produk</b></summary>

```graphql
query GetProducts {
  products(
    limit: 20
    offset: 0
    filter: {
      categoryId: 1
      minPrice: 10000
      maxPrice: 1000000
      inStock: true
    }
  ) {
    id
    name
    description
    price
    stock
    imageUrl
    category {
      id
      name
    }
    averageRating
    totalReviews
  }
}
```
</details>

<details>
<summary><b>Tambah Produk Baru (Admin)</b></summary>

```graphql
mutation CreateProduct {
  createProduct(input: {
    name: "iPhone 15 Pro Max"
    description: "Latest iPhone with A17 Pro chip"
    price: 19999000
    stock: 50
    categoryId: 1
    imageUrl: "https://example.com/iphone15.jpg"
  }) {
    id
    name
    price
    stock
    createdAt
  }
}
```
</details>

### 🛒 Shopping Cart

<details>
<summary><b>Tambah ke Keranjang</b></summary>

```graphql
mutation AddToCart {
  addToCart(input: {
    productId: 1
    quantity: 2
  }) {
    id
    totalItems
    totalPrice
    items {
      id
      quantity
      subtotal
      product {
        id
        name
        price
        imageUrl
        stock
      }
    }
  }
}
```
</details>

<details>
<summary><b>Lihat Keranjang</b></summary>

```graphql
query GetCart {
  myCart {
    id
    totalItems
    totalPrice
    items {
      id
      quantity
      subtotal
      product {
        id
        name
        price
        imageUrl
        stock
      }
    }
    updatedAt
  }
}
```
</details>

### 💳 Orders & Payments

<details>
<summary><b>Buat Order Baru</b></summary>

```graphql
mutation CreateOrder {
  createOrder(input: {
    addressId: 1
    paymentMethod: "VIRTUAL_ACCOUNT"
    bankCode: "BCA"
    notes: "Kirim pagi hari"
  }) {
    order {
      id
      orderNumber
      status
      totalAmount
      items {
        product {
          name
        }
        quantity
        price
      }
    }
    payment {
      id
      externalId
      paymentUrl
      accountNumber
      bankCode
      amount
      expiresAt
    }
  }
}
```
</details>

<details>
<summary><b>Cek Status Pembayaran</b></summary>

```graphql
query CheckPaymentStatus($orderId: ID!) {
  order(id: $orderId) {
    id
    orderNumber
    status
    payment {
      id
      status
      method
      paidAt
      accountNumber
      bankCode
    }
  }
}
```
</details>

---

## 💳 Integrasi Xendit

### Payment Methods Tersedia

| Method | Code | Deskripsi |
|--------|------|-----------|
| 💳 Credit Card | `CREDIT_CARD` | Visa, Mastercard, JCB |
| 🏦 Virtual Account | `VIRTUAL_ACCOUNT` | BCA, Mandiri, BNI, BRI, Permata |
| 🏪 E-Wallet | `EWALLET` | OVO, Dana, LinkAja, ShopeePay |
| 🏬 Retail Outlet | `RETAIL_OUTLET` | Alfamart, Indomaret |
| 🏧 QR Code | `QR_CODE` | QRIS |

### Setup Webhook

1. Masuk ke [Xendit Dashboard](https://dashboard.xendit.co/)
2. Buka **Settings → Webhooks**
3. Tambah webhook URL: `https://yourdomain.com/webhook/xendit`
4. Pilih events:
   - `invoice.paid`
   - `invoice.expired`
   - `payment.paid`
   - `payment.failed`

### Testing Xendit (Development)

```bash
# Test Virtual Account BCA
curl -X POST http://localhost:8000/graphql \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "query": "mutation { createOrder(input: { addressId: 1, paymentMethod: \"VIRTUAL_ACCOUNT\", bankCode: \"BCA\" }) { payment { accountNumber } } }"
  }'
```

**Test Payment dengan Xendit Simulator:**
- BCA VA: `https://simulator.xendit.co/`

---

## 🧪 Testing

### Unit Tests

```bash
# Run semua tests
cargo test

# Run dengan output detail
cargo test -- --show-output --nocapture

# Run test spesifik
cargo test test_user_registration

# Run tests di module tertentu
cargo test services::auth::tests
```

### Integration Tests

```bash
# Run integration tests
cargo test --test '*'

# Dengan coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html --output-dir coverage
```

### Load Testing

```bash
# Install tools
sudo apt install apache2-utils  # untuk ab
cargo install drill             # HTTP load testing

# Test health endpoint
ab -n 10000 -c 100 http://127.0.0.1:8000/health

# Test GraphQL
drill --benchmark benchmark.yml --stats
```

---

## 🐳 Docker Deployment

### Development dengan Docker Compose

```bash
# Start semua services (app + postgres)
docker-compose up -d

# Lihat logs
docker-compose logs -f app

# Stop services
docker-compose down

# Rebuild
docker-compose up -d --build
```

### Production Docker

```dockerfile
# Build image
docker build -t toko-nuvella:latest .

# Run container
docker run -d \
  --name toko-nuvella \
  -p 8000:8000 \
  --env-file .env.production \
  toko-nuvella:latest
```

---

## 🚀 Deployment Production

### Platform Rekomendasi

| Platform | Difficulty | Cost | Best For |
|----------|-----------|------|----------|
| 🚂 Railway | ⭐ Easy | Free tier | Prototype/MVP |
| 🌊 Fly.io | ⭐⭐ Medium | Pay as go | Production |
| ☁️ AWS ECS | ⭐⭐⭐ Advanced | $$ | Enterprise |
| 🔷 DigitalOcean | ⭐⭐ Medium | $ | Small business |

### Environment Variables Production

```env
# PRODUCTION SETTINGS
DATABASE_URL=postgresql://user:pass@prod-db:5432/db
SECRET_KEY=<GENERATE_NEW_SECURE_KEY>
JWT_SECRET=<GENERATE_NEW_SECURE_KEY>
XENDIT_API_KEY=xnd_production_<YOUR_PROD_KEY>
SERVER_HOST=0.0.0.0
SERVER_PORT=8000
RUST_LOG=warn
```

---

## 🛡️ Security Best Practices

✅ **Implemented:**
- Password hashing dengan Argon2id
- JWT dengan expiry time
- SQL injection prevention (SeaORM)
- CORS configuration
- Rate limiting
- Input validation & sanitization
- Webhook signature verification (Xendit)

⚠️ **Rekomendasi Production:**
- Aktifkan HTTPS/TLS
- Setup firewall rules
- Database encryption at rest
- Regular security audits
- Implement API versioning
- Setup monitoring & alerting

---

## 🎯 Roadmap

### ✅ Completed
- [x] GraphQL API dengan async-graphql
- [x] Authentication & Authorization (JWT)
- [x] CRUD Produk & Kategori
- [x] Shopping Cart Management
- [x] Order Processing System
- [x] Xendit Payment Integration
- [x] Webhook Handler
- [x] Review & Rating System

### 🔄 In Progress
- [ ] Email Notifications (SMTP)
- [ ] Admin Dashboard API
- [ ] Product Image Upload (S3/Cloud Storage)
- [ ] Advanced Search (Full-text search)

### 📋 Planned
- [ ] Real-time Notifications (WebSocket)
- [ ] Product Recommendations (ML)
- [ ] Multi-language Support (i18n)
- [ ] API Rate Limiting per User
- [ ] Elasticsearch Integration
- [ ] Redis Caching Layer
- [ ] Shipping Integration (J&T, JNE, SiCepat)
- [ ] Promo & Discount System
- [ ] Loyalty Points Program
- [ ] Chat Customer Service

---

## 🤝 Contributing

Kontribusi sangat diterima! Berikut cara berkontribusi:

### Langkah Kontribusi

1. **Fork** repository ini
2. Buat **feature branch** (`git checkout -b feature/FiturKeren`)
3. **Commit** perubahan (`git commit -m 'Menambahkan fitur keren'`)
4. **Push** ke branch (`git push origin feature/FiturKeren`)
5. Buat **Pull Request**

### Guidelines

- ✅ Ikuti konvensi penamaan Rust
- ✅ Tulis unit tests untuk fitur baru
- ✅ Update dokumentasi jika perlu
- ✅ Pastikan `cargo fmt && cargo clippy` clean
- ✅ Commit message yang jelas

---

## 📄 License

Project ini menggunakan **MIT License** - lihat file [LICENSE](LICENSE) untuk detail.

---

## 👨‍💻 Author

<div align="center">

**Gilbertt1214**

[![GitHub](https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white)](https://github.com/Gilbertt1214)
[![LinkedIn](https://img.shields.io/badge/LinkedIn-0077B5?style=for-the-badge&logo=linkedin&logoColor=white)](https://www.linkedin.com/in/fahriana-nurzukhruf-45986a308/)

*Membangun masa depan e-commerce dengan Rust* 🦀

</div>

---

## 🙏 Acknowledgments

Terima kasih kepada:

- 🦀 **Rust Community** - Untuk tools dan library yang luar biasa
- 🎯 **Tokio Team** - Async runtime yang powerful
- 🌐 **Axum** - Web framework yang ergonomis
- 🗄️ **SeaORM** - ORM solution yang excellent
- 🎨 **async-graphql** - GraphQL implementation terbaik
- 💳 **Xendit** - Payment gateway Indonesia terbaik
- 💡 **Open Source Contributors** - Untuk inspirasi dan guidance

---

## 📞 Support & Contact

Butuh bantuan? Silakan hubungi:

- 🐛 **Bug Reports**: [Open an Issue](https://github.com/Gilbertt1214/be-toko-online-rust/issues)
- 💬 **Diskusi**: [GitHub Discussions](https://github.com/Gilbertt1214/be-toko-online-rust/discussions)
- 📧 **Email**: gilbertt@example.com
- 💼 **LinkedIn**: [Fahriana Nurzukhruf](https://www.linkedin.com/in/fahriana-nurzukhruf-45986a308/)

---

<div align="center">

### ⭐ Jika project ini bermanfaat, berikan star! ⭐

**Dibuat dengan ❤️ menggunakan 🦀 Rust**

**Selamat Ngoding! 🚀**

![Rust Logo](https://www.rust-lang.org/logos/rust-logo-512x512.png)

</div>