use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use bigdecimal::BigDecimal;
use std::str::FromStr;
use chrono::Utc;

use crate::models::{category, product, product_image, user};
use crate::models::prelude::{Category, Product, ProductImage, User};

pub async fn seed_all(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Starting Database Seeding ===\n");
    
    clear_data(db).await?;
    seed_categories(db).await?;
    seed_users(db).await?;
    seed_products(db).await?;
    seed_product_images(db).await?;
    
    println!("\n=== Seeding Completed Successfully! ===");
    Ok(())
}

async fn clear_data(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("Clearing existing data...");
    
    ProductImage::delete_many().exec(db).await?;
    Product::delete_many().exec(db).await?;
    Category::delete_many().exec(db).await?;
    User::delete_many().exec(db).await?;
    
    println!("  Data cleared\n");
    Ok(())
}

async fn seed_categories(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("Seeding categories...");
    
    let categories = vec![
        category::ActiveModel {
            id: Set(1),
            name: Set("Electronics".to_string()),
            slug: Set("electronics".to_string()),
            created_at: Set(Some(Utc::now().naive_utc())),
            updated_at: Set(Some(Utc::now().naive_utc())),
        },
        category::ActiveModel {
            id: Set(2),
            name: Set("Fashion".to_string()),
            slug: Set("fashion".to_string()),
            created_at: Set(Some(Utc::now().naive_utc())),
            updated_at: Set(Some(Utc::now().naive_utc())),
        },
        category::ActiveModel {
            id: Set(3),
            name: Set("Home & Living".to_string()),
            slug: Set("home-living".to_string()),
            created_at: Set(Some(Utc::now().naive_utc())),
            updated_at: Set(Some(Utc::now().naive_utc())),
        },
        category::ActiveModel {
            id: Set(4),
            name: Set("Books".to_string()),
            slug: Set("books".to_string()),
            created_at: Set(Some(Utc::now().naive_utc())),
            updated_at: Set(Some(Utc::now().naive_utc())),
        },
        category::ActiveModel {
            id: Set(5),
            name: Set("Sports".to_string()),
            slug: Set("sports".to_string()),
            created_at: Set(Some(Utc::now().naive_utc())),
            updated_at: Set(Some(Utc::now().naive_utc())),
        },
    ];
    
    for category in categories {
        category.insert(db).await?;
    }
    
    println!("  Seeded 5 categories");
    Ok(())
}

async fn seed_users(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("Seeding users...");
    
    let hashed_password = bcrypt::hash("admin123", bcrypt::DEFAULT_COST)?;
    
    let users = vec![
        user::ActiveModel {
            id: Set(1),
            username: Set("admin".to_string()),
            email: Set("admin@tokoonline.com".to_string()),
            password: Set(Some(hashed_password.clone())),
            role: Set(user::UserRole::Admin),
        },
        user::ActiveModel {
            id: Set(2),
            username: Set("pengusaha1".to_string()),
            email: Set("pengusaha1@tokoonline.com".to_string()),
            password: Set(Some(hashed_password.clone())),
            role: Set(user::UserRole::Pengusaha),
        },
        user::ActiveModel {
            id: Set(3),
            username: Set("pengguna1".to_string()),
            email: Set("pengguna1@tokoonline.com".to_string()),
            password: Set(Some(hashed_password)),
            role: Set(user::UserRole::Pengguna),
        },
    ];
    
    for user in users {
        user.insert(db).await?;
    }
    
    println!("  Seeded 3 users (password: admin123)");
    println!("    - admin@tokoonline.com (role: admin)");
    println!("    - pengusaha1@tokoonline.com (role: pengusaha)");
    println!("    - pengguna1@tokoonline.com (role: pengguna)");
    Ok(())
}

async fn seed_products(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("Seeding products...");
    
    let now = Utc::now().naive_utc();
    
    let products = vec![
        // ELECTRONICS (1-10)
        product::ActiveModel {
            id: Set(1),
            name: Set("iPhone 15 Pro Max 256GB".to_string()),
            slug: Set("iphone-15-pro-max-256gb".to_string()),
            short_description: Set(Some("Smartphone flagship terbaru dengan chip A17 Pro".to_string())),
            description: Set(Some("iPhone 15 Pro Max dilengkapi dengan chip A17 Pro yang powerful, kamera 48MP dengan teknologi computational photography terbaru, layar Super Retina XDR 6.7 inch, dan baterai yang tahan seharian. Tersedia dalam 256GB storage untuk menyimpan semua foto dan video berkualitas tinggi Anda.".to_string())),
            price: Set(BigDecimal::from_str("19999000")?),
            stock: Set(50),
            category_id: Set(Some(1)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(2),
            name: Set("Samsung Galaxy S24 Ultra".to_string()),
            slug: Set("samsung-s24-ultra".to_string()),
            short_description: Set(Some("Flagship Samsung dengan S Pen dan AI features".to_string())),
            description: Set(Some("Samsung Galaxy S24 Ultra hadir dengan kamera 200MP yang luar biasa, S Pen terintegrasi untuk produktivitas maksimal, layar Dynamic AMOLED 2X 6.8 inch, dan fitur Galaxy AI untuk membantu aktivitas sehari-hari Anda.".to_string())),
            price: Set(BigDecimal::from_str("18499000")?),
            stock: Set(35),
            category_id: Set(Some(1)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(3),
            name: Set("MacBook Pro 14 inch M3".to_string()),
            slug: Set("macbook-pro-14-m3".to_string()),
            short_description: Set(Some("Laptop powerful dengan chip M3 untuk profesional".to_string())),
            description: Set(Some("MacBook Pro 14 inch dengan chip M3 memberikan performa luar biasa untuk video editing, 3D rendering, dan coding. Dilengkapi layar Liquid Retina XDR, baterai hingga 22 jam, dan desain premium aluminum.".to_string())),
            price: Set(BigDecimal::from_str("32999000")?),
            stock: Set(20),
            category_id: Set(Some(1)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(4),
            name: Set("Sony WH-1000XM5".to_string()),
            slug: Set("sony-wh-1000xm5".to_string()),
            short_description: Set(Some("Headphone wireless premium dengan noise cancelling terbaik".to_string())),
            description: Set(Some("Sony WH-1000XM5 adalah headphone wireless dengan teknologi noise cancelling industry-leading, kualitas audio Hi-Res, battery life hingga 30 jam, dan desain ultra comfortable untuk penggunaan sepanjang hari.".to_string())),
            price: Set(BigDecimal::from_str("5000000")?),
            stock: Set(100),
            category_id: Set(Some(1)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(5),
            name: Set("iPad Pro 12.9 inch M2".to_string()),
            slug: Set("ipad-pro-129-m2".to_string()),
            short_description: Set(Some("Tablet premium untuk kreativitas dan produktivitas".to_string())),
            description: Set(Some("iPad Pro 12.9 inch dengan chip M2 dan layar Liquid Retina XDR yang menakjubkan. Sempurna untuk digital art dengan Apple Pencil, video editing, dan multitasking dengan iPadOS.".to_string())),
            price: Set(BigDecimal::from_str("16999000")?),
            stock: Set(30),
            category_id: Set(Some(1)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(6),
            name: Set("Dell XPS 13 Plus".to_string()),
            slug: Set("dell-xps-13-plus".to_string()),
            short_description: Set(Some("Laptop ultraportable dengan desain premium dan performa tinggi".to_string())),
            description: Set(Some("Dell XPS 13 Plus menggabungkan desain futuristik dengan performa powerful. Dilengkapi prosesor Intel Core generasi terbaru, layar InfinityEdge 13.4 inch, dan berat hanya 1.24kg untuk mobilitas maksimal.".to_string())),
            price: Set(BigDecimal::from_str("22999000")?),
            stock: Set(25),
            category_id: Set(Some(1)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(7),
            name: Set("AirPods Pro 2nd Gen".to_string()),
            slug: Set("airpods-pro-2".to_string()),
            short_description: Set(Some("Earbuds premium dengan Active Noise Cancellation".to_string())),
            description: Set(Some("AirPods Pro generasi kedua dengan chip H2 untuk ANC yang lebih baik, Adaptive Transparency, Spatial Audio dengan dynamic head tracking, dan charging case dengan speaker built-in untuk Find My.".to_string())),
            price: Set(BigDecimal::from_str("3499000")?),
            stock: Set(150),
            category_id: Set(Some(1)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(8),
            name: Set("Samsung Galaxy Watch 6".to_string()),
            slug: Set("galaxy-watch-6".to_string()),
            short_description: Set(Some("Smartwatch dengan health monitoring lengkap".to_string())),
            description: Set(Some("Galaxy Watch 6 melacak tidur, heart rate, stress level, dan aktivitas fitness Anda. Dengan layar AMOLED yang cerah, battery life hingga 40 jam, dan kompatibilitas dengan Android dan iOS.".to_string())),
            price: Set(BigDecimal::from_str("4299000")?),
            stock: Set(80),
            category_id: Set(Some(1)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(9),
            name: Set("Canon EOS R6 Mark II".to_string()),
            slug: Set("canon-eos-r6-mark-ii".to_string()),
            short_description: Set(Some("Kamera mirrorless full-frame untuk fotografer pro".to_string())),
            description: Set(Some("Canon EOS R6 Mark II dengan sensor 24.2MP full-frame, continuous shooting 40fps, video 6K oversampled 4K, dan sistem AF Dual Pixel CMOS II yang canggih untuk menangkap momen perfect setiap saat.".to_string())),
            price: Set(BigDecimal::from_str("42999000")?),
            stock: Set(15),
            category_id: Set(Some(1)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(10),
            name: Set("LG OLED C3 55 inch".to_string()),
            slug: Set("lg-oled-c3-55".to_string()),
            short_description: Set(Some("Smart TV OLED 4K dengan teknologi self-lit pixel".to_string())),
            description: Set(Some("LG OLED C3 menghadirkan pengalaman menonton terbaik dengan self-lit pixel OLED, prosesor α9 Gen6 AI, Dolby Vision IQ & Dolby Atmos, dan fitur gaming 120Hz VRR untuk PlayStation 5 dan Xbox Series X.".to_string())),
            price: Set(BigDecimal::from_str("18999000")?),
            stock: Set(40),
            category_id: Set(Some(1)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },

        // FASHION (11-20)
        product::ActiveModel {
            id: Set(11),
            name: Set("Nike Air Max 270".to_string()),
            slug: Set("nike-air-max-270".to_string()),
            short_description: Set(Some("Sepatu running dengan air cushioning maksimal".to_string())),
            description: Set(Some("Nike Air Max 270 menampilkan unit Air yang terbesar untuk kenyamanan maksimal sepanjang hari. Desain sleek dan modern cocok untuk running maupun casual wear.".to_string())),
            price: Set(BigDecimal::from_str("2500000")?),
            stock: Set(120),
            category_id: Set(Some(2)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(12),
            name: Set("Adidas Ultraboost 23".to_string()),
            slug: Set("adidas-ultraboost-23".to_string()),
            short_description: Set(Some("Running shoes premium dengan Boost technology".to_string())),
            description: Set(Some("Adidas Ultraboost 23 dengan teknologi Boost yang memberikan energy return optimal, Primeknit upper yang breathable, dan Continental rubber outsole untuk traction maksimal di berbagai permukaan.".to_string())),
            price: Set(BigDecimal::from_str("2800000")?),
            stock: Set(100),
            category_id: Set(Some(2)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(13),
            name: Set("Levi's 501 Original Jeans".to_string()),
            slug: Set("levis-501-jeans".to_string()),
            short_description: Set(Some("Celana jeans klasik yang timeless".to_string())),
            description: Set(Some("Levi's 501 adalah jeans original straight fit yang iconic. Terbuat dari denim berkualitas premium, button fly, dan potongan klasik yang cocok untuk berbagai body type.".to_string())),
            price: Set(BigDecimal::from_str("1200000")?),
            stock: Set(200),
            category_id: Set(Some(2)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(14),
            name: Set("Uniqlo Heattech Innerwear".to_string()),
            slug: Set("uniqlo-heattech".to_string()),
            short_description: Set(Some("Pakaian dalam dengan teknologi penghangat".to_string())),
            description: Set(Some("Uniqlo Heattech menggunakan teknologi fabric yang menyerap kelembaban dan mengubahnya menjadi panas, menjaga tubuh tetap hangat bahkan di cuaca dingin. Tipis, nyaman, dan tidak terlihat dari luar.".to_string())),
            price: Set(BigDecimal::from_str("299000")?),
            stock: Set(500),
            category_id: Set(Some(2)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(15),
            name: Set("Zara Oversized Blazer".to_string()),
            slug: Set("zara-oversized-blazer".to_string()),
            short_description: Set(Some("Blazer formal dengan potongan modern".to_string())),
            description: Set(Some("Zara Oversized Blazer dengan potongan contemporary yang memberikan siluet modern. Cocok untuk acara formal maupun smart casual dengan tampilan sophisticated.".to_string())),
            price: Set(BigDecimal::from_str("1500000")?),
            stock: Set(80),
            category_id: Set(Some(2)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(16),
            name: Set("Converse Chuck Taylor All Star".to_string()),
            slug: Set("converse-chuck-taylor".to_string()),
            short_description: Set(Some("Sepatu kasual ikonik yang timeless".to_string())),
            description: Set(Some("Converse Chuck Taylor All Star adalah sepatu kasual iconic yang telah menjadi favorit sejak 1917. Canvas upper berkualitas, rubber sole yang tahan lama, dan style yang never out of fashion.".to_string())),
            price: Set(BigDecimal::from_str("800000")?),
            stock: Set(300),
            category_id: Set(Some(2)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(17),
            name: Set("H&M Cotton T-Shirt Pack".to_string()),
            slug: Set("hm-cotton-tshirt".to_string()),
            short_description: Set(Some("Kaos katun basic isi 3 pcs".to_string())),
            description: Set(Some("H&M Cotton T-Shirt Pack berisi 3 kaos basic dari 100% cotton yang soft dan breathable. Essential wardrobe item yang cocok untuk layering atau dipakai sendiri.".to_string())),
            price: Set(BigDecimal::from_str("250000")?),
            stock: Set(600),
            category_id: Set(Some(2)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(18),
            name: Set("New Balance 574 Core".to_string()),
            slug: Set("new-balance-574".to_string()),
            short_description: Set(Some("Sneakers retro dengan kenyamanan maksimal".to_string())),
            description: Set(Some("New Balance 574 Core menggabungkan style retro dengan teknologi modern. ENCAP midsole untuk support dan comfort, suede dan mesh upper, dan iconic silhouette yang versatile.".to_string())),
            price: Set(BigDecimal::from_str("1300000")?),
            stock: Set(150),
            category_id: Set(Some(2)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(19),
            name: Set("Pull & Bear Denim Jacket".to_string()),
            slug: Set("pullbear-denim-jacket".to_string()),
            short_description: Set(Some("Jaket denim klasik untuk gaya kasual".to_string())),
            description: Set(Some("Pull & Bear Denim Jacket dengan classic trucker style, terbuat dari denim berkualitas tinggi dengan wash yang sempurna. Versatile piece yang cocok dengan berbagai outfit.".to_string())),
            price: Set(BigDecimal::from_str("650000")?),
            stock: Set(120),
            category_id: Set(Some(2)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(20),
            name: Set("Vans Old Skool".to_string()),
            slug: Set("vans-old-skool".to_string()),
            short_description: Set(Some("Sepatu skate ikonik dengan side stripe".to_string())),
            description: Set(Some("Vans Old Skool dengan signature side stripe yang iconic. Canvas dan suede upper, padded collar untuk support, dan waffle outsole untuk grip maksimal. Cocok untuk skateboarding dan casual wear.".to_string())),
            price: Set(BigDecimal::from_str("900000")?),
            stock: Set(250),
            category_id: Set(Some(2)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },

        // HOME & LIVING (21-25)
        product::ActiveModel {
            id: Set(21),
            name: Set("IKEA MALM Bed Frame Queen".to_string()),
            slug: Set("ikea-malm-bed".to_string()),
            short_description: Set(Some("Rangka tempat tidur modern dengan storage".to_string())),
            description: Set(Some("IKEA MALM Bed Frame Queen size dengan desain minimalis dan clean lines. Dilengkapi 4 storage drawer untuk penyimpanan extra, konstruksi solid, dan mudah dirakit.".to_string())),
            price: Set(BigDecimal::from_str("4500000")?),
            stock: Set(30),
            category_id: Set(Some(3)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(22),
            name: Set("Philips Hue Smart Bulb Starter".to_string()),
            slug: Set("philips-hue-starter".to_string()),
            short_description: Set(Some("Lampu pintar color changing dengan hub".to_string())),
            description: Set(Some("Philips Hue Starter Kit dengan 3 smart bulbs dan bridge hub. Kontrol warna dan brightness via app, voice control dengan Alexa/Google, dan integrasi dengan smart home ecosystem.".to_string())),
            price: Set(BigDecimal::from_str("2500000")?),
            stock: Set(100),
            category_id: Set(Some(3)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(23),
            name: Set("Xiaomi Air Purifier 4 Pro".to_string()),
            slug: Set("xiaomi-air-purifier-4".to_string()),
            short_description: Set(Some("Pembersih udara dengan HEPA filter untuk ruangan besar".to_string())),
            description: Set(Some("Xiaomi Air Purifier 4 Pro dengan true HEPA filter yang menangkap 99.97% partikel hingga 0.3 mikron. CADR 500m³/h, cocok untuk ruangan hingga 60m², display OLED, dan kontrol via Mi Home app.".to_string())),
            price: Set(BigDecimal::from_str("3200000")?),
            stock: Set(60),
            category_id: Set(Some(3)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(24),
            name: Set("Dyson V15 Detect".to_string()),
            slug: Set("dyson-v15-detect".to_string()),
            short_description: Set(Some("Vacuum cleaner cordless dengan laser detection".to_string())),
            description: Set(Some("Dyson V15 Detect dengan laser yang mengungkap debu mikroskopis, LCD screen yang menampilkan particle count, hingga 60 menit runtime, dan powerful suction yang membersihkan berbagai permukaan.".to_string())),
            price: Set(BigDecimal::from_str("12999000")?),
            stock: Set(25),
            category_id: Set(Some(3)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(25),
            name: Set("Muji Aroma Diffuser".to_string()),
            slug: Set("muji-aroma-diffuser".to_string()),
            short_description: Set(Some("Diffuser aromaterapi dengan desain minimalis".to_string())),
            description: Set(Some("Muji Aroma Diffuser dengan desain simple dan elegant. Ultrasonic technology yang silent, timer settings, LED mood light, dan kapasitas 350ml untuk aromaterapi sepanjang malam.".to_string())),
            price: Set(BigDecimal::from_str("850000")?),
            stock: Set(150),
            category_id: Set(Some(3)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },

        // BOOKS (26-28)
        product::ActiveModel {
            id: Set(26),
            name: Set("The Rust Programming Language".to_string()),
            slug: Set("rust-book".to_string()),
            short_description: Set(Some("Buku resmi Rust programming oleh Steve Klabnik".to_string())),
            description: Set(Some("Buku comprehensive tentang Rust programming language, mencakup ownership, borrowing, lifetimes, dan advanced concepts. Ditulis oleh Steve Klabnik dan Carol Nichols, perfect untuk beginners hingga intermediate programmers.".to_string())),
            price: Set(BigDecimal::from_str("500000")?),
            stock: Set(200),
            category_id: Set(Some(4)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(27),
            name: Set("Clean Code by Robert Martin".to_string()),
            slug: Set("clean-code".to_string()),
            short_description: Set(Some("Handbook untuk menulis code yang clean dan maintainable".to_string())),
            description: Set(Some("Clean Code oleh Robert C. Martin (Uncle Bob) mengajarkan prinsip-prinsip menulis code yang readable, maintainable, dan elegant. Must-read untuk setiap software developer yang ingin meningkatkan code quality.".to_string())),
            price: Set(BigDecimal::from_str("600000")?),
            stock: Set(150),
            category_id: Set(Some(4)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(28),
            name: Set("Atomic Habits by James Clear".to_string()),
            slug: Set("atomic-habits".to_string()),
            short_description: Set(Some("Panduan praktis membangun kebiasaan baik".to_string())),
            description: Set(Some("Atomic Habits mengajarkan cara membangun good habits dan menghilangkan bad habits dengan strategi praktis dan scientific approach. Buku self-improvement terlaris yang telah mengubah jutaan hidup.".to_string())),
            price: Set(BigDecimal::from_str("120000")?),
            stock: Set(500),
            category_id: Set(Some(4)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },

        // SPORTS (29-30)
        product::ActiveModel {
            id: Set(29),
            name: Set("Yoga Mat Premium 6mm".to_string()),
            slug: Set("yoga-mat-premium".to_string()),
            short_description: Set(Some("Matras yoga anti slip dengan carrying strap".to_string())),
            description: Set(Some("Yoga Mat Premium 6mm dengan material TPE eco-friendly yang non-toxic, double-sided non-slip texture untuk stability maksimal, dan lightweight design. Dilengkapi carrying strap untuk portability.".to_string())),
            price: Set(BigDecimal::from_str("350000")?),
            stock: Set(400),
            category_id: Set(Some(5)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
        product::ActiveModel {
            id: Set(30),
            name: Set("Dumbell Set 20kg Adjustable".to_string()),
            slug: Set("dumbell-20kg".to_string()),
            short_description: Set(Some("Set dumbell adjustable untuk home workout".to_string())),
            description: Set(Some("Dumbell Set Adjustable 20kg dengan quick-change weight system, textured grip handles untuk comfort, dan compact design perfect untuk home gym. Range dari 2.5kg hingga 20kg per dumbell.".to_string())),
            price: Set(BigDecimal::from_str("1500000")?),
            stock: Set(80),
            category_id: Set(Some(5)),
            seller_id: Set(Some(2)),
            is_active: Set(Some(true)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        },
    ];
    
    for product in products {
        product.insert(db).await?;
    }
    
    println!("  Seeded 30 products with descriptions across 5 categories");
    Ok(())
}

async fn seed_product_images(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("Seeding product images...");
    
    let images: Vec<product_image::ActiveModel> = (1..=30).map(|i| {
        product_image::ActiveModel {
            id: Set(i),
            product_id: Set(i),
            image_url: Set(format!("https://via.placeholder.com/800/667eea/fff?text=Product+{}", i)),
            is_primary: Set(Some(true)),
        }
    }).collect();
    
    for image in images {
        image.insert(db).await?;
    }
    
    println!("  Seeded 30 product images");
    Ok(())
}