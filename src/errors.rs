use actix_web::{HttpResponse, ResponseError};
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PayBySquareError {
    #[error("Invalid IBAN format: {0}")]
    InvalidIban(String),

    #[error("Invalid SWIFT/BIC format: {0}")]
    InvalidSwift(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Missing required field: either 'iban' or 'bank_accounts' must be provided")]
    MissingBankAccount,

    #[error("Amount must be greater than 0")]
    InvalidAmount,

    #[error("Field too long: {field} (max: {max}, got: {actual})")]
    FieldTooLong {
        field: String,
        max: usize,
        actual: usize,
    },

    #[error("Compression failed: {0}")]
    CompressionError(String),

    #[error("QR generation failed: {0}")]
    QrError(String),

    #[error("Image processing failed: {0}")]
    ImageError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl ResponseError for PayBySquareError {
    fn error_response(&self) -> HttpResponse {
        match self {
            PayBySquareError::ValidationError(_)
            | PayBySquareError::InvalidIban(_)
            | PayBySquareError::InvalidSwift(_)
            | PayBySquareError::MissingBankAccount
            | PayBySquareError::InvalidAmount
            | PayBySquareError::FieldTooLong { .. } => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": self.to_string()
                }))
            }
            _ => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })),
        }
    }
}

pub type Result<T> = std::result::Result<T, PayBySquareError>;
