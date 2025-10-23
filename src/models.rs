use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[schema(example = json!({
    "amount": 100.50,
    "iban": "SK9611000000002918599669",
    "currency": "EUR",
    "beneficiary_name": "John Doe",
    "variable_symbol": "1234567890",
    "note": "Payment for invoice"
}))]
pub struct PaymentRequest {
    /// Payment amount (must be greater than 0)
    #[validate(range(min = 0.01))]
    pub amount: f64,

    /// IBAN of the beneficiary account
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 34))]
    pub iban: Option<String>,

    /// Alternative: list of bank accounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_accounts: Option<Vec<BankAccount>>,

    /// Currency code (default: EUR)
    #[serde(default = "default_currency")]
    pub currency: String,

    /// SWIFT/BIC code
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 11))]
    pub swift: Option<String>,

    /// Payment date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,

    /// Payment due date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_due_date: Option<NaiveDate>,

    /// Invoice ID (max 10 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 10))]
    pub invoice_id: Option<String>,

    /// Beneficiary name (max 70 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 70))]
    pub beneficiary_name: Option<String>,

    /// Beneficiary address line 1 (max 70 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 70))]
    pub beneficiary_address_1: Option<String>,

    /// Beneficiary address line 2 (max 70 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 70))]
    pub beneficiary_address_2: Option<String>,

    /// Variable symbol (max 10 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 10))]
    pub variable_symbol: Option<String>,

    /// Constant symbol (max 4 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 4))]
    pub constant_symbol: Option<String>,

    /// Specific symbol (max 10 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 10))]
    pub specific_symbol: Option<String>,

    /// SEPA reference information (max 35 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 35))]
    pub originators_reference_information: Option<String>,

    /// Note/message for beneficiary (max 140 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 140))]
    pub note: Option<String>,

    /// Payment options (payment order, standing order, direct debit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_options: Option<Vec<PaymentOption>>,

    /// Standing order details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standing_order: Option<StandingOrder>,

    /// Direct debit details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_debit: Option<DirectDebit>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BankAccount {
    /// IBAN of the bank account
    pub iban: String,

    /// SWIFT/BIC code (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentOption {
    PaymentOrder,
    StandingOrder,
    DirectDebit,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StandingOrder {
    /// Day of the month (1-31)
    pub day: u8,

    /// Months when payment should be executed (1-12)
    pub month: Vec<u8>,

    /// Periodicity of the standing order
    pub periodicity: Periodicity,

    /// Last execution date
    pub last_date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Periodicity {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    HalfYearly,
    Yearly,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DirectDebit {
    /// Direct debit scheme
    pub scheme: DirectDebitScheme,

    /// Type of direct debit
    pub debit_type: DirectDebitType,

    /// Mandate ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_id: Option<String>,

    /// Creditor ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creditor_id: Option<String>,

    /// Maximum amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_amount: Option<f64>,

    /// Valid until date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_till_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DirectDebitScheme {
    Sepa,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DirectDebitType {
    OneOff,
    Recurrent,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct QrOptions {
    /// Include frame around QR code (default: true)
    #[serde(default = "default_with_frame")]
    pub with_frame: bool,

    /// QR code size in pixels (default: 300)
    #[serde(default = "default_qr_size")]
    pub qr_size: u32,
}

impl Default for QrOptions {
    fn default() -> Self {
        Self {
            with_frame: true,
            qr_size: 300,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CodeResponse {
    /// PayBySquare code as text string
    pub code: String,
}

fn default_currency() -> String {
    "EUR".to_string()
}

fn default_with_frame() -> bool {
    true
}

fn default_qr_size() -> u32 {
    300
}
