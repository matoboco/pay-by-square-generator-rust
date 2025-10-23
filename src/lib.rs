pub mod errors;
pub mod generator;
pub mod models;
pub mod qr;
pub mod validation;

pub use errors::{PayBySquareError, Result};
pub use generator::generate_pay_by_square_code;
pub use models::{
    BankAccount, CodeResponse, DirectDebit, DirectDebitScheme, DirectDebitType, PaymentOption,
    PaymentRequest, Periodicity, QrOptions, StandingOrder,
};
pub use qr::{add_frame, generate_default_frame, generate_qr_image};
pub use validation::validate_payment_request;

/// Generates a complete PayBySquare QR code image with optional frame
pub fn generate_pay_by_square_qr(
    payment: &PaymentRequest,
    opts: QrOptions,
    frame_data: Option<&[u8]>,
) -> Result<Vec<u8>> {
    // Validate payment request
    validate_payment_request(payment)?;

    // Generate PayBySquare code
    let code = generate_pay_by_square_code(payment)?;

    // Generate QR image
    let qr_data = generate_qr_image(&code, opts.qr_size)?;

    // Add frame if requested
    if opts.with_frame {
        add_frame(qr_data, frame_data)
    } else {
        Ok(qr_data)
    }
}

/// Generates only the PayBySquare code string
pub fn generate_code_only(payment: &PaymentRequest) -> Result<String> {
    // Validate payment request
    validate_payment_request(payment)?;

    // Generate PayBySquare code
    generate_pay_by_square_code(payment)
}
