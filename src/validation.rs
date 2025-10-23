use crate::errors::{PayBySquareError, Result};
use crate::models::PaymentRequest;

/// Validates a payment request
pub fn validate_payment_request(payment: &PaymentRequest) -> Result<()> {
    // Validate amount
    if payment.amount <= 0.0 {
        return Err(PayBySquareError::InvalidAmount);
    }

    // Validate that either iban or bank_accounts is provided
    if payment.iban.is_none() && payment.bank_accounts.is_none() {
        return Err(PayBySquareError::MissingBankAccount);
    }

    // Validate IBAN format
    if let Some(ref iban) = payment.iban {
        validate_iban(iban)?;
    }

    // Validate bank accounts
    if let Some(ref accounts) = payment.bank_accounts {
        for account in accounts {
            validate_iban(&account.iban)?;
            if let Some(ref swift) = account.swift {
                validate_swift(swift)?;
            }
        }
    }

    // Validate SWIFT if provided
    if let Some(ref swift) = payment.swift {
        validate_swift(swift)?;
    }

    // Validate field lengths
    if let Some(ref invoice_id) = payment.invoice_id {
        validate_length("invoice_id", invoice_id, 10)?;
    }

    if let Some(ref beneficiary_name) = payment.beneficiary_name {
        validate_length("beneficiary_name", beneficiary_name, 70)?;
    }

    if let Some(ref addr1) = payment.beneficiary_address_1 {
        validate_length("beneficiary_address_1", addr1, 70)?;
    }

    if let Some(ref addr2) = payment.beneficiary_address_2 {
        validate_length("beneficiary_address_2", addr2, 70)?;
    }

    if let Some(ref vs) = payment.variable_symbol {
        validate_length("variable_symbol", vs, 10)?;
    }

    if let Some(ref cs) = payment.constant_symbol {
        validate_length("constant_symbol", cs, 4)?;
    }

    if let Some(ref ss) = payment.specific_symbol {
        validate_length("specific_symbol", ss, 10)?;
    }

    if let Some(ref ref_info) = payment.originators_reference_information {
        validate_length("originators_reference_information", ref_info, 35)?;
    }

    if let Some(ref note) = payment.note {
        validate_length("note", note, 140)?;
    }

    Ok(())
}

/// Validates IBAN format (basic validation)
fn validate_iban(iban: &str) -> Result<()> {
    let iban_clean = iban.replace(' ', "");

    // IBAN must be 15-34 characters
    if iban_clean.len() < 15 || iban_clean.len() > 34 {
        return Err(PayBySquareError::InvalidIban(
            "IBAN must be between 15 and 34 characters".to_string(),
        ));
    }

    // IBAN must start with 2 letters (country code)
    if !iban_clean.chars().take(2).all(|c| c.is_ascii_alphabetic()) {
        return Err(PayBySquareError::InvalidIban(
            "IBAN must start with a 2-letter country code".to_string(),
        ));
    }

    // Next 2 characters must be digits (check digits)
    if !iban_clean
        .chars()
        .skip(2)
        .take(2)
        .all(|c| c.is_ascii_digit())
    {
        return Err(PayBySquareError::InvalidIban(
            "IBAN check digits must be numeric".to_string(),
        ));
    }

    // Rest must be alphanumeric
    if !iban_clean.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(PayBySquareError::InvalidIban(
            "IBAN contains invalid characters".to_string(),
        ));
    }

    Ok(())
}

/// Validates SWIFT/BIC format
fn validate_swift(swift: &str) -> Result<()> {
    let swift_clean = swift.replace(' ', "");

    // SWIFT must be 8 or 11 characters
    if swift_clean.len() != 8 && swift_clean.len() != 11 {
        return Err(PayBySquareError::InvalidSwift(
            "SWIFT/BIC must be 8 or 11 characters".to_string(),
        ));
    }

    // Must be alphanumeric
    if !swift_clean.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(PayBySquareError::InvalidSwift(
            "SWIFT/BIC contains invalid characters".to_string(),
        ));
    }

    Ok(())
}

/// Validates string length
fn validate_length(field: &str, value: &str, max: usize) -> Result<()> {
    if value.len() > max {
        return Err(PayBySquareError::FieldTooLong {
            field: field.to_string(),
            max,
            actual: value.len(),
        });
    }
    Ok(())
}
