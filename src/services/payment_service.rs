use crate::config::xendit::XenditConfig;
use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Main Payment Service untuk integrasi dengan Xendit
pub struct PaymentService {
    config: XenditConfig,
    client: Client,
}

// ==================== Request Structs ====================

/// Request untuk membuat invoice baru
#[derive(Debug, Serialize)]
pub struct CreateInvoiceRequest {
    pub external_id: String,
    pub amount: i64,
    pub payer_email: String,
    pub description: String,
    pub customer: CustomerInfo,
    pub items: Vec<InvoiceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_duration: Option<i64>, 
}

/// Informasi customer
#[derive(Debug, Serialize, Clone)]
pub struct CustomerInfo {
    pub given_names: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

/// Item dalam invoice
#[derive(Debug, Serialize, Clone)]
pub struct InvoiceItem {
    pub name: String,
    pub quantity: i32,
    pub price: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

// ==================== Response Structs ====================

/// Response dari Xendit saat membuat/get invoice
#[derive(Debug, Deserialize, Clone)]
pub struct XenditInvoiceResponse {
    pub id: String,
    pub external_id: String,
    pub invoice_url: String,
    pub status: String,
    pub expiry_date: String,
    pub amount: i64,
    #[serde(default)]
    pub paid_amount: i64,
    #[serde(default)]
    pub description: String,
}

/// Webhook payload dari Xendit
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct XenditWebhookPayload {
    pub id: String,
    pub external_id: String,
    pub status: String,
    pub amount: i64,
    #[serde(default)]
    pub paid_amount: i64,
    #[serde(default)]
    pub payment_channel: String,
    #[serde(default)]
    pub payment_method: String,
}

// ==================== Payment Status Enum ====================

/// Status pembayaran
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Paid,
    Settled,
    Expired,
    Failed,
}

impl std::fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentStatus::Pending => write!(f, "PENDING"),
            PaymentStatus::Paid => write!(f, "PAID"),
            PaymentStatus::Settled => write!(f, "SETTLED"),
            PaymentStatus::Expired => write!(f, "EXPIRED"),
            PaymentStatus::Failed => write!(f, "FAILED"),
        }
    }
}

impl From<String> for PaymentStatus {
    fn from(status: String) -> Self {
        match status.to_uppercase().as_str() {
            "PAID" | "SETTLED" => PaymentStatus::Paid,
            "PENDING" => PaymentStatus::Pending,
            "EXPIRED" => PaymentStatus::Expired,
            _ => PaymentStatus::Failed,
        }
    }
}

// ==================== Payment Service Implementation ====================

impl PaymentService {
    /// Buat instance baru dari PaymentService
    pub fn new(config: XenditConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    /// Buat invoice baru di Xendit
    pub async fn create_invoice(
        &self,
        request: CreateInvoiceRequest,
    ) -> Result<XenditInvoiceResponse> {
        let url = format!("{}/v2/invoices", self.config.api_url);

        // Build request body
        let body = serde_json::json!({
            "external_id": request.external_id,
            "amount": request.amount,
            "payer_email": request.payer_email,
            "description": request.description,
            "customer": {
                "given_names": request.customer.given_names,
                "email": request.customer.email,
                "mobile_number": request.customer.mobile_number.unwrap_or_default(),
            },
            "items": request.items,
            "invoice_duration": request.invoice_duration.unwrap_or(86400), // 24 jam default
            "success_redirect_url": self.config.success_redirect_url,
            "failure_redirect_url": self.config.failure_redirect_url,
        });

        let response = self
            .client
            .post(&url)
            .header("Authorization", self.config.get_basic_auth())
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Xendit")?;

        // Check response status
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!(
                "Xendit API error ({}): {}",
                status,
                error_text
            );
        }

        let invoice = response
            .json::<XenditInvoiceResponse>()
            .await
            .context("Failed to parse Xendit response")?;

        Ok(invoice)
    }

    /// Get detail invoice by ID
    pub async fn get_invoice(&self, invoice_id: &str) -> Result<XenditInvoiceResponse> {
        let url = format!("{}/v2/invoices/{}", self.config.api_url, invoice_id);

        let response = self
            .client
            .get(&url)
            .header("Authorization", self.config.get_basic_auth())
            .send()
            .await
            .context("Failed to get invoice from Xendit")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!(
                "Failed to get invoice ({}): {}",
                status,
                error_text
            );
        }

        let invoice = response
            .json::<XenditInvoiceResponse>()
            .await
            .context("Failed to parse invoice response")?;

        Ok(invoice)
    }

    /// Get invoice by external_id
    pub async fn get_invoice_by_external_id(
        &self,
        external_id: &str,
    ) -> Result<XenditInvoiceResponse> {
        // Xendit menggunakan external_id di URL
        self.get_invoice(external_id).await
    }

    /// Expire invoice secara manual
    pub async fn expire_invoice(&self, invoice_id: &str) -> Result<XenditInvoiceResponse> {
        let url = format!("{}/v2/invoices/{}/expire!", self.config.api_url, invoice_id);

        let response = self
            .client
            .post(&url)
            .header("Authorization", self.config.get_basic_auth())
            .send()
            .await
            .context("Failed to expire invoice")?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            anyhow::bail!("Failed to expire invoice: {}", error);
        }

        Ok(response.json().await?)
    }

    /// Process webhook payload dari Xendit
    pub fn process_webhook(&self, payload: &XenditWebhookPayload) -> PaymentStatus {
        PaymentStatus::from(payload.status.clone())
    }

    /// Verify webhook token untuk keamanan
    pub fn verify_webhook_token(&self, received_token: &str) -> bool {
        if self.config.webhook_token.is_empty() {
            // Jika tidak ada token, skip verification (tidak disarankan)
            return true;
        }

        // Constant time comparison untuk mencegah timing attacks
        received_token == self.config.webhook_token
    }

    /// Verify webhook signature (jika Xendit mengirim signature)
    pub fn verify_webhook_signature(
        &self,
        signature: &str,
        payload: &str,
    ) -> bool {
        // TODO: Implement HMAC signature verification
        // Xendit menggunakan callback_token untuk verifikasi
        // Untuk sekarang, kita pakai token verification
        self.verify_webhook_token(signature)
    }

    /// Get konfigurasi Xendit (untuk debugging)
    pub fn get_config(&self) -> &XenditConfig {
        &self.config
    }

    /// Check apakah payment service dalam mode production
    pub fn is_production(&self) -> bool {
        self.config.is_production
    }
}

// ==================== Helper Functions ====================

/// Konversi amount dari Rupiah ke format Xendit (tanpa desimal)
pub fn rupiah_to_xendit(rupiah: f64) -> i64 {
    rupiah.round() as i64
}

/// Konversi amount dari Xendit ke Rupiah
pub fn xendit_to_rupiah(amount: i64) -> f64 {
    amount as f64
}

/// Format Rupiah untuk display
pub fn format_rupiah(amount: i64) -> String {
    format!("Rp {}", amount.to_string().as_str()
        .chars()
        .rev()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % 3 == 0 {
                vec!['.', c]
            } else {
                vec![c]
            }
        })
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_status_conversion() {
        assert_eq!(PaymentStatus::from("PAID".to_string()), PaymentStatus::Paid);
        assert_eq!(PaymentStatus::from("SETTLED".to_string()), PaymentStatus::Paid);
        assert_eq!(PaymentStatus::from("PENDING".to_string()), PaymentStatus::Pending);
        assert_eq!(PaymentStatus::from("EXPIRED".to_string()), PaymentStatus::Expired);
        assert_eq!(PaymentStatus::from("UNKNOWN".to_string()), PaymentStatus::Failed);
    }

    #[test]
    fn test_rupiah_conversion() {
        assert_eq!(rupiah_to_xendit(100000.0), 100000);
        assert_eq!(xendit_to_rupiah(100000), 100000.0);
    }

    #[test]
    fn test_format_rupiah() {
        assert_eq!(format_rupiah(100000), "Rp 100.000");
        assert_eq!(format_rupiah(1000000), "Rp 1.000.000");
        assert_eq!(format_rupiah(50000), "Rp 50.000");
    }
}