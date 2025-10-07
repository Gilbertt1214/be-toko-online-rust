# 🛒 Toko Online NUVELLA

<div align="center">

![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg?style=for-the-badge&logo=rust)
![Axum](https://img.shields.io/badge/Axum-0.7-6B46C1?style=for-the-badge)
![GraphQL](https://img.shields.io/badge/GraphQL-API-E10098?style=for-the-badge&logo=graphql)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-16-336791?style=for-the-badge&logo=postgresql)
![Xendit](https://img.shields.io/badge/Xendit-Payment-00BFA5?style=for-the-badge)

**Backend API E-Commerce Modern dengan Rust + Axum + GraphQL + SeaORM + Xendit**

*Super cepat, type-safe, dan siap produksi*

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
│                    🌐 Lapisan Klien                          │
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
│              📊 Lapisan GraphQL (async-graphql)              │
│                                                               │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  Query Resolvers  │  Mutation Resolvers  │  Types   │    │
│  └─────────────────────────────────────────────────────┘    │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                  💼 Lapisan Logika Bisnis                    │
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
│              🗄️ Lapisan Akses Data (SeaORM)                 │
│                                                               │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Model Entitas  │  Migrasi  │  Connection Pool       │   │
│  └──────────────────────────────────────────────────────┘   │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                  🗃️ Basis Data PostgreSQL                   │
│                                                               │
│  Users │ Products │ Orders │ Payments │ Reviews │ dll.      │
└─────────────────────────────────────────────────────────────┘

        ┌────────────────────────────────────┐
        │    🔌 Layanan Eksternal            │
        │                                    │
        │  • Xendit Payment Gateway          │
        │  • Layanan Email (SMTP)            │
        │  • Cloud Storage (Opsional)        │
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

### Prasyarat

Pastikan sudah terinstall:

- **Rust** 1.70+ ([Pasang Rust](https://rustup.rs/))
- **PostgreSQL** 14+ ([Unduh PostgreSQL](https://www.postgresql.org/download/))
- **Cargo** (otomatis dengan Rust)
- **SeaORM CLI** (untuk migrasi)

### Langkah-langkah Instalasi

#### 1️⃣ Clone Repository

```bash
git clone https://github.com/Gilbertt1214/be-toko-online-rust.git
cd be-toko-online-rust
```

#### 2️⃣ Setup Environment Variables

Buat file `.env` di root project:

```env
# Konfigurasi Basis Data
DATABASE_URL=postgresql://postgres:password@localhost:5432/toko_online_nuvella

# Konfigurasi Server
SERVER_HOST=127.0.0.1
SERVER_PORT=8000

# Kunci Keamanan (GENERATE BARU!)
SECRET_KEY=your-secret-key-32-characters-minimum
JWT_SECRET=your-jwt-secret-32-characters-minimum

# Konfigurasi Xendit
XENDIT_API_KEY=xnd_development_your_key_here
XENDIT_WEBHOOK_TOKEN=your_webhook_verification_token
XENDIT_CALLBACK_URL=https://yourdomain.com/webhook/xendit

# Konfigurasi Email (Opsional)
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password

# Logging
RUST_LOG=info,sqlx=warn
```

**🔐 Buat Kunci Rahasia:**

```bash
# Instal openssl jika belum ada
# Buat SECRET_KEY
openssl rand -hex 32

# Buat JWT_SECRET
openssl rand -hex 32
```

**💳 Setup Xendit:**

1. Daftar di [Xendit Dashboard](https://dashboard.xendit.co/)
2. Dapatkan **API Key** dari Settings → Developers
3. Setup **Webhook URL** untuk notifikasi pembayaran
4. Simpan **Token Verifikasi Webhook**

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

#### 5️⃣ Build & Jalankan

**Mode Development:**

```bash
# Jalankan dengan auto-reload (instal cargo-watch)
cargo install cargo-watch
cargo watch -x run

# Atau jalankan biasa
cargo run
```

**Mode Produksi:**

```bash
# Build binary yang dioptimasi
cargo build --release

# Jalankan binary
./target/release/be-toko-online-rust
```

#### 6️⃣ Verifikasi Instalasi

Server berjalan di `http://127.0.0.1:8000` 🎉

**Tes endpoints:**

```bash
# Cek kesehatan
curl http://127.0.0.1:8000/health

# Buka Apollo Sandbox
open http://127.0.0.1:8000/graphql
```

---

## 🔄 Flow Diagram Development

### 🎯 Alur Pemrosesan Request

```mermaid
graph TB
    A[Request Klien] -->|HTTP/HTTPS| B[Router Axum]
    B -->|GraphQL Query/Mutation| C[Handler GraphQL]
    B -->|REST Endpoint| D[Handler REST]
    B -->|Webhook Xendit| E[Handler Webhook]
    
    C --> F[Lapisan Middleware]
    D --> F
    E --> F
    
    F -->|Cek Auth| G[Validasi JWT]
    G -->|Valid| H[Lapisan Service]
    G -->|Invalid| Z[401 Tidak Terotorisasi]
    
    H --> I{Logika Bisnis}
    
    I -->|Service Auth| J[Register/Login/Logout]
    I -->|Service Produk| K[CRUD Produk]
    I -->|Service Keranjang| L[Operasi Keranjang]
    I -->|Service Pesanan| M[Proses Pesanan]
    I -->|Service Pembayaran| N[Integrasi Xendit]
    
    J --> O[Lapisan Database]
    K --> O
    L --> O
    M --> O
    N --> O
    
    O -->|SeaORM| P[(PostgreSQL)]
    
    N -->|Panggilan API| Q[API Xendit]
    Q -->|Pembayaran Dibuat| R[Respon Pembayaran]
    R --> S[Redirect Klien]
    
    Q -.->|Webhook| E
    E -->|Verifikasi Tanda Tangan| T{Valid?}
    T -->|Ya| U[Update Status Pembayaran]
    T -->|Tidak| V[Tolak]
    U --> O
    
    P --> W[Pembuat Respon]
    W --> X[Respon JSON]
    X --> Y[Klien]
```

### 💳 Alur Pembayaran dengan Xendit

```mermaid
sequenceDiagram
    participant C as Klien/Frontend
    participant A as API Axum
    participant S as Service Pembayaran
    participant X as API Xendit
    participant W as Webhook
    participant D as Basis Data
    
    C->>A: createOrder(metode_pembayaran)
    A->>S: Proses Pesanan
    S->>D: Simpan Pesanan (PENDING)
    D-->>S: Pesanan Dibuat
    
    S->>X: Buat Invoice/VA/EWallet
    X-->>S: URL Pembayaran/Nomor VA
    S->>D: Simpan Pembayaran (PENDING)
    D-->>S: Pembayaran Tersimpan
    S-->>A: Info Pembayaran
    A-->>C: URL/Detail Pembayaran
    
    C->>C: User Menyelesaikan Pembayaran
    
    X->>W: Webhook: payment.paid
    W->>W: Verifikasi Tanda Tangan
    W->>D: Update Pembayaran (SUCCESS)
    W->>D: Update Pesanan (PAID)
    D-->>W: Diperbarui
    W-->>X: 200 OK
    
    Note over W: Kirim Notifikasi Email
    W->>C: Notifikasi Push (Opsional)
    
    C->>A: checkOrderStatus(orderId)
    A->>D: Ambil Status Pesanan
    D-->>A: Status Pesanan: PAID
    A-->>C: Pesanan Dikonfirmasi
```

### 🔄 Alur Autentikasi Pengguna

```mermaid
sequenceDiagram
    participant C as Klien/Frontend
    participant A as API Axum
    participant Auth as Service Auth
    participant Hash as Service Argon2
    participant JWT as Service JWT
    participant DB as Basis Data PostgreSQL
    
    rect rgb(200, 230, 255)
        Note over C,DB: 📝 Alur Registrasi
        C->>A: POST /graphql: register(email, password, nama)
        A->>Auth: Proses Request Registrasi
        
        Auth->>Auth: Validasi Input (format email, kekuatan password)
        
        Auth->>DB: Cek apakah email sudah ada
        DB-->>Auth: Hasil Query
        
        alt Email Sudah Terdaftar
            Auth-->>A: Error: Email sudah terdaftar
            A-->>C: 400 Bad Request
        else Email Tersedia
            Auth->>Hash: Hash Password (Argon2id)
            Hash-->>Auth: Password Ter-hash
            
            Auth->>DB: INSERT INTO users (nama, email, password_hash, role)
            DB-->>Auth: Record User Dibuat (ID: 123)
            
            Auth->>JWT: Buat Access Token (kadaluarsa: 15menit)
            Auth->>JWT: Buat Refresh Token (kadaluarsa: 7hari)
            JWT-->>Auth: Pasangan Token
            
            Auth->>DB: Simpan Refresh Token
            DB-->>Auth: Token Tersimpan
            
            Auth-->>A: Objek User + Token
            A-->>C: 201 Created + Set-Cookie(refreshToken)
            
            Note over C: Pengguna Berhasil Terdaftar
        end
    end
    
    rect rgb(220, 255, 220)
        Note over C,DB: 🔑 Alur Login
        C->>A: POST /graphql: login(email, password)
        A->>Auth: Proses Request Login
        
        Auth->>Auth: Validasi Input
        
        Auth->>DB: SELECT * FROM users WHERE email = ?
        DB-->>Auth: Record User Ditemukan
        
        alt User Tidak Ditemukan
            Auth-->>A: Error: Kredensial tidak valid
            A-->>C: 401 Unauthorized
        else User Ditemukan
            Auth->>Hash: Verifikasi Password(plain, hashed)
            Hash-->>Auth: Hasil Pencocokan Password
            
            alt Password Salah
                Auth-->>A: Error: Kredensial tidak valid
                A-->>C: 401 Unauthorized
            else Password Benar
                Auth->>JWT: Buat Access Token Baru
                Auth->>JWT: Buat Refresh Token Baru
                JWT-->>Auth: Pasangan Token
                
                Auth->>DB: UPDATE users SET last_login = NOW()
                Auth->>DB: UPSERT refresh_token
                DB-->>Auth: Diperbarui
                
                Auth-->>A: Objek User + Token
                A-->>C: 200 OK + Set-Cookie(refreshToken)
                
                Note over C: Pengguna Berhasil Login
            end
        end
    end
    
    rect rgb(255, 245, 220)
        Note over C,DB: 🔒 Alur Request Terproteksi
        C->>A: POST /graphql: getProfile() + Authorization Header
        A->>Auth: Ekstrak JWT dari Header
        
        Auth->>JWT: Verifikasi Tanda Tangan Token
        JWT-->>Auth: Token Valid/Invalid
        
        alt Token Invalid/Kadaluarsa
            Auth-->>A: Error: Token tidak valid
            A-->>C: 401 Unauthorized
        else Token Valid
            Auth->>JWT: Decode Klaim Token
            JWT-->>Auth: User ID: 123
            
            Auth->>DB: SELECT * FROM users WHERE id = 123
            DB-->>Auth: Data User
            
            alt User Tidak Ditemukan
                Auth-->>A: Error: User tidak ditemukan
                A-->>C: 404 Not Found
            else User Ditemukan
                Auth-->>A: Data Profil User
                A-->>C: 200 OK + JSON Profil User
                
                Note over C: Profil Berhasil Diambil
            end
        end
    end
    
    rect rgb(255, 220, 220)
        Note over C,DB: 🔄 Alur Refresh Token
        C->>A: POST /graphql: refreshToken() + Cookie(refreshToken)
        A->>Auth: Ekstrak Refresh Token dari Cookie
        
        Auth->>JWT: Verifikasi Refresh Token
        JWT-->>Auth: Token Valid/Invalid
        
        alt Token Invalid/Kadaluarsa
            Auth-->>A: Error: Refresh token tidak valid
            A-->>C: 401 Unauthorized
            Note over C: User Harus Login Lagi
        else Token Valid
            Auth->>DB: Cek apakah refresh token ada & tidak dicabut
            DB-->>Auth: Status Token
            
            alt Token Dicabut/Tidak Ditemukan
                Auth-->>A: Error: Token dicabut
                A-->>C: 401 Unauthorized
            else Token Aktif
                Auth->>JWT: Buat Access Token Baru
                JWT-->>Auth: Access Token Baru
                
                Auth->>JWT: Rotasi Refresh Token (Opsional)
                JWT-->>Auth: Refresh Token Baru
                
                Auth->>DB: UPDATE refresh_token SET token = ?, updated_at = NOW()
                DB-->>Auth: Diperbarui
                
                Auth-->>A: Pasangan Token Baru
                A-->>C: 200 OK + Set-Cookie(refreshTokenBaru)
                
                Note over C: Token Berhasil Di-refresh
            end
        end
    end
    
    rect rgb(255, 230, 230)
        Note over C,DB: 🚪 Alur Logout
        C->>A: POST /graphql: logout() + Authorization Header
        A->>Auth: Ekstrak JWT & Proses Logout
        
        Auth->>JWT: Decode Token untuk Dapatkan User ID
        JWT-->>Auth: User ID: 123
        
        Auth->>DB: DELETE FROM refresh_tokens WHERE user_id = 123
        DB-->>Auth: Token Dicabut
        
        Auth-->>A: Logout Sukses
        A-->>C: 200 OK + Clear-Cookie(refreshToken)
        
        Note over C: Pengguna Berhasil Logout
    end
```

### 🔐 Strategi Token

**Access Token:**
- ⏱️ **Masa Berlaku**: 15 menit
- 📦 **Penyimpanan**: Memory/LocalStorage (Frontend)
- 🎯 **Tujuan**: Otorisasi request API
- 🔒 **Keamanan**: Berumur pendek, tanda tangan JWT

**Refresh Token:**
- ⏱️ **Masa Berlaku**: 7 hari
- 🍪 **Penyimpanan**: HTTP-only Cookie
- 🎯 **Tujuan**: Mendapatkan access token baru
- 🔒 **Keamanan**: Disimpan di DB, bisa dicabut

### 🛡️ Fitur Keamanan

| Fitur | Implementasi | Perlindungan Terhadap |
|---------|---------------|-------------------|
| **Password Hashing** | Argon2id (memory-hard) | Rainbow tables, brute force |
| **Tanda Tangan JWT** | HS256 dengan secret key | Manipulasi token |
| **Rotasi Refresh Token** | Token baru setiap refresh | Serangan replay token |
| **HTTP-Only Cookies** | Secure, SameSite=Strict | Serangan XSS |
| **Pencabutan Token** | Blacklist berbasis DB | Token yang dikompromikan |
| **Rate Limiting** | Batas per-IP dan per-user | Serangan brute force |

### 📋 Penanganan Error

```rust
// Error Autentikasi Umum
pub enum AuthError {
    InvalidCredentials,        // 401
    EmailAlreadyExists,        // 400
    TokenExpired,              // 401
    TokenInvalid,              // 401
    TokenRevoked,              // 401
    UserNotFound,              // 404
    UnauthorizedAccess,        // 403
    InternalServerError,       // 500
}
```

### 🔄 Strategi Refresh Token

**Kapan melakukan refresh:**
- Access token kadaluarsa dalam < 5 menit
- Respon 401 dari API
- Saat startup aplikasi (jika refresh token valid)

**Implementasi Frontend:**
```typescript
// Contoh interceptor Axios
axios.interceptors.response.use(
  (response) => response,
  async (error) => {
    if (error.response?.status === 401) {
      try {
        // Coba refresh token
        await refreshAccessToken();
        // Ulangi request asli
        return axios(error.config);
      } catch (refreshError) {
        // Redirect ke halaman login
        window.location.href = '/login';
      }
    }
    return Promise.reject(error);
  }
);
```

### 🎯 Best Practices yang Diimplementasikan

✅ **Persyaratan Password:**
- Minimal 8 karakter
- Minimal 1 huruf besar
- Minimal 1 huruf kecil
- Minimal 1 angka
- Minimal 1 karakter spesial

✅ **Rate Limiting:**
- Percobaan login: 5 per 15 menit per IP
- Registrasi: 3 per jam per IP
- Refresh token: 10 per jam per user

✅ **Manajemen Sesi:**
- Login satu perangkat (opsional)
- Dukungan multi-perangkat dengan pelacakan token
- Kemampuan logout paksa semua perangkat

✅ **Audit Logging:**
- Log semua percobaan autentikasi
- Lacak login berhasil/gagal
- Monitor aktivitas mencurigakan

### 🛒 Alur Keranjang Belanja ke Pesanan

```mermaid
stateDiagram-v2
    [*] --> TelusurProduk: User Membuka Toko
    
    TelusurProduk --> TambahKeKeranjang: Pilih Produk
    TambahKeKeranjang --> LihatKeranjang: Lihat Keranjang
    
    LihatKeranjang --> UpdateJumlah: Ubah Jumlah
    UpdateJumlah --> LihatKeranjang
    
    LihatKeranjang --> HapusItem: Hapus Item
    HapusItem --> LihatKeranjang
    
    LihatKeranjang --> Checkout: Lanjut Checkout
    
    Checkout --> PilihAlamat: Pilih Alamat
    PilihAlamat --> PilihPembayaran: Pilih Pembayaran
    
    PilihPembayaran --> BuatPesanan: Buat Pesanan
    
    BuatPesanan --> PesananPending: Pesanan Dibuat
    PesananPending --> PembayaranDibuat: Generate Pembayaran
    
    PembayaranDibuat --> MenungguPembayaran: Redirect ke Pembayaran
    
    MenungguPembayaran --> PembayaranSukses: User Bayar
    MenungguPembayaran --> PembayaranKadaluarsa: Timeout
    
    PembayaranSukses --> PesananDibayar: Update Status
    PesananDibayar --> PesananDiproses: Mulai Proses
    PesananDiproses --> PesananDikirim: Dikirim
    PesananDikirim --> PesananSampai: Sampai
    PesananSampai --> PesananSelesai: Selesai
    
    PembayaranKadaluarsa --> PesananDibatalkan: Pesanan Dibatalkan
    
    PesananSelesai --> [*]
    PesananDibatalkan --> [*]
```

---

## 📖 Dokumentasi API

### GraphQL Playground

Akses Apollo Sandbox di: **`http://127.0.0.1:8000/graphql`**

Fitur:
- ✅ Dokumentasi schema otomatis
- ✅ Auto-completion
- ✅ Riwayat query
- ✅ Syntax highlighting
- ✅ Eksekusi real-time

### Ringkasan Endpoint

| Endpoint | Method | Deskripsi |
|----------|--------|-----------|
| `/graphql` | POST | Endpoint API GraphQL |
| `/health` | GET | Cek kesehatan server |
| `/webhook/xendit` | POST | Webhook pembayaran Xendit |
| `/` | GET | Halaman informasi API |

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

### Metode Pembayaran Tersedia

| Metode | Kode | Deskripsi |
|--------|------|-----------|
| 💳 Kartu Kredit | `CREDIT_CARD` | Visa, Mastercard, JCB |
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
# Tes Virtual Account BCA
curl -X POST http://localhost:8000/graphql \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer TOKEN_JWT_ANDA" \
  -d '{
    "query": "mutation { createOrder(input: { addressId: 1, paymentMethod: \"VIRTUAL_ACCOUNT\", bankCode: \"BCA\" }) { payment { accountNumber } } }"
  }'
```

**Tes Pembayaran dengan Xendit Simulator:**
- BCA VA: `https://simulator.xendit.co/`

---

## 🧪 Pengujian

### Unit Tests

```bash
# Jalankan semua tes
cargo test

# Jalankan dengan output detail
cargo test -- --show-output --nocapture

# Jalankan tes spesifik
cargo test test_user_registration

# Jalankan tes di module tertentu
cargo test services::auth::tests
```

### Integration Tests

```bash
# Jalankan integration tests
cargo test --test '*'

# Dengan coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html --output-dir coverage
```

### Load Testing

```bash
# Instal tools
sudo apt install apache2-utils  # untuk ab
cargo install drill             # HTTP load testing

# Tes endpoint health
ab -n 10000 -c 100 http://127.0.0.1:8000/health

# Tes GraphQL
drill --benchmark benchmark.yml --stats
```

---

## 🐳 Deployment Docker

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

# Jalankan container
docker run -d \
  --name toko-nuvella \
  -p 8000:8000 \
  --env-file .env.production \
  toko-nuvella:latest
```

---

## 🚀 Deployment Produksi

### Platform Rekomendasi

| Platform | Kesulitan | Biaya | Terbaik Untuk |
|----------|-----------|------|----------|
| 🚂 Railway | ⭐ Mudah | Gratis tier | Prototipe/MVP |
| 🌊 Fly.io | ⭐⭐ Sedang | Bayar sesuai pemakaian | Produksi |
| ☁️ AWS ECS | ⭐⭐⭐ Lanjutan | $ | Enterprise |
| 🔷 DigitalOcean | ⭐⭐ Sedang | $ | Bisnis kecil |

### Environment Variables Produksi

```env
# PENGATURAN PRODUKSI
DATABASE_URL=postgresql://user:pass@prod-db:5432/db
SECRET_KEY=<GENERATE_KUNCI_AMAN_BARU>
JWT_SECRET=<GENERATE_KUNCI_AMAN_BARU>
XENDIT_API_KEY=xnd_production_<KUNCI_PROD_ANDA>
SERVER_HOST=0.0.0.0
SERVER_PORT=8000
RUST_LOG=warn
```

---

## 🛡️ Best Practices Keamanan

✅ **Sudah Diimplementasikan:**
- Password hashing dengan Argon2id
- JWT dengan waktu kadaluarsa
- Pencegahan SQL injection (SeaORM)
- Konfigurasi CORS
- Rate limiting
- Validasi & sanitasi input
- Verifikasi tanda tangan webhook (Xendit)

⚠️ **Rekomendasi Produksi:**
- Aktifkan HTTPS/TLS
- Setup aturan firewall
- Enkripsi basis data at rest
- Audit keamanan rutin
- Implementasi versioning API
- Setup monitoring & alerting

---

## 🎯 Roadmap

### ✅ Selesai
- [x] GraphQL API dengan async-graphql
- [x] Autentikasi & Otorisasi (JWT)
- [x] CRUD Produk & Kategori
- [x] Manajemen Keranjang Belanja
- [x] Sistem Pemrosesan Pesanan
- [x] Integrasi Pembayaran Xendit
- [x] Handler Webhook
- [x] Sistem Review & Rating

### 🔄 Sedang Dikerjakan
- [ ] Notifikasi Email (SMTP)
- [ ] API Dashboard Admin
- [ ] Upload Gambar Produk (S3/Cloud Storage)
- [ ] Pencarian Lanjutan (Full-text search)

### 📋 Direncanakan
- [ ] Notifikasi Real-time (WebSocket)
- [ ] Rekomendasi Produk (ML)
- [ ] Dukungan Multi-bahasa (i18n)
- [ ] Rate Limiting API per User
- [ ] Integrasi Elasticsearch
- [ ] Lapisan Caching Redis
- [ ] Integrasi Pengiriman (J&T, JNE, SiCepat)
- [ ] Sistem Promo & Diskon
- [ ] Program Poin Loyalitas
- [ ] Chat Customer Service

---

## 🤝 Berkontribusi

Kontribusi sangat diterima! Berikut cara berkontribusi:

### Langkah Kontribusi

1. **Fork** repository ini
2. Buat **feature branch** (`git checkout -b feature/FiturKeren`)
3. **Commit** perubahan (`git commit -m 'Menambahkan fitur keren'`)
4. **Push** ke branch (`git push origin feature/FiturKeren`)
5. Buat **Pull Request**

### Panduan

- ✅ Ikuti konvensi penamaan Rust
- ✅ Tulis unit tests untuk fitur baru
- ✅ Update dokumentasi jika perlu
- ✅ Pastikan `cargo fmt && cargo clippy` bersih
- ✅ Commit message yang jelas

---

## 📄 Lisensi

Project ini menggunakan **MIT License** - lihat file [LICENSE](LICENSE) untuk detail.

---

## 👨‍💻 Pembuat

<div align="center">

**Gilbertt1214**

[![GitHub](https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white)](https://github.com/Gilbertt1214)
[![LinkedIn](https://img.shields.io/badge/LinkedIn-0077B5?style=for-the-badge&logo=linkedin&logoColor=white)](https://www.linkedin.com/in/fahriana-nurzukhruf-45986a308/)

*Membangun masa depan e-commerce dengan Rust* 🦀

</div>

---

## 🙏 Ucapan Terima Kasih

Terima kasih kepada:

- 🦀 **Rust Community** - Untuk tools dan library yang luar biasa
- 🎯 **Tokio Team** - Async runtime yang powerful
- 🌐 **Axum** - Web framework yang ergonomis
- 🗄️ **SeaORM** - ORM solution yang excellent
- 🎨 **async-graphql** - GraphQL implementation terbaik
- 💳 **Xendit** - Payment gateway Indonesia terbaik
- 💡 **Open Source Contributors** - Untuk inspirasi dan panduan

---

## 📞 Dukungan & Kontak

Butuh bantuan? Silakan hubungi:

- 🐛 **Laporan Bug**: [Buka Issue](https://github.com/Gilbertt1214/be-toko-online-rust/issues)
- 💬 **Diskusi**: [GitHub Discussions](https://github.com/Gilbertt1214/be-toko-online-rust/discussions)
- 📧 **Email**: gilbertt@example.com
- 💼 **LinkedIn**: [Fahriana Nurzukhruf](https://www.linkedin.com/in/fahriana-nurzukhruf-45986a308/)

---

<div align="center">

### ⭐ Jika project ini bermanfaat, berikan bintang! ⭐

**Dibuat dengan ❤️ menggunakan 🦀 Rust**

**Selamat Ngoding! 🚀**

![Rust Logo](https://www.rust-lang.org/logos/rust-logo-512x512.png)

</div>