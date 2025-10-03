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
            description: Set(Some("Smartphone flagship Apple dengan chip A17 Pro dan kamera 48MP".to_string())),
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
            description: Set(Some("Flagship Samsung dengan S Pen dan kamera 200MP".to_string())),
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
            description: Set(Some("Laptop powerful untuk profesional dengan chip M3".to_string())),
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
            description: Set(Some("Headphone wireless premium dengan noise cancelling terbaik".to_string())),
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
            description: Set(Some("Tablet premium dengan layar Liquid Retina XDR".to_string())),
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
            description: Set(Some("Laptop ultraportable dengan desain premium".to_string())),
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
            description: Set(Some("Earbuds wireless dengan Active Noise Cancellation".to_string())),
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
            description: Set(Some("Smartwatch dengan monitoring kesehatan lengkap".to_string())),
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
            description: Set(Some("Kamera mirrorless full-frame untuk fotografer profesional".to_string())),
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
            description: Set(Some("Smart TV OLED 4K dengan teknologi self-lit pixel".to_string())),
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
            description: Set(Some("Sepatu running dengan air cushioning maksimal".to_string())),
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
            description: Set(Some("Running shoes premium dengan Boost technology".to_string())),
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
            description: Set(Some("Celana jeans klasik straight fit".to_string())),
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
            description: Set(Some("Pakaian dalam dengan teknologi penghangat".to_string())),
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
            description: Set(Some("Blazer formal dengan potongan modern".to_string())),
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
            description: Set(Some("Sepatu kasual ikonik yang timeless".to_string())),
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
            description: Set(Some("Kaos katun basic isi 3 pcs".to_string())),
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
            description: Set(Some("Sneakers retro dengan kenyamanan maksimal".to_string())),
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
            description: Set(Some("Jaket denim klasik untuk gaya kasual".to_string())),
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
            description: Set(Some("Sepatu skate ikonik dengan side stripe".to_string())),
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
            description: Set(Some("Rangka tempat tidur modern dengan storage".to_string())),
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
            description: Set(Some("Lampu pintar color changing dengan hub".to_string())),
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
            description: Set(Some("Pembersih udara dengan HEPA filter".to_string())),
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
            description: Set(Some("Vacuum cleaner cordless dengan laser detection".to_string())),
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
            description: Set(Some("Diffuser aromaterapi dengan desain minimalis".to_string())),
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
            description: Set(Some("Buku resmi Rust by Steve Klabnik".to_string())),
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
            description: Set(Some("Handbook agile software craftsmanship".to_string())),
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
            description: Set(Some("Panduan membangun kebiasaan baik".to_string())),
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
            description: Set(Some("Matras yoga anti slip dengan carrying strap".to_string())),
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
            description: Set(Some("Set dumbell adjustable untuk home workout".to_string())),
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
    
    println!("  Seeded 30 products across 5 categories");
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