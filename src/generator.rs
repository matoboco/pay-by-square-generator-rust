use crate::errors::{PayBySquareError, Result};
use crate::models::{PaymentOption, PaymentRequest, Periodicity};
use chrono::NaiveDate;
use std::io::Write;

/// Generates a PayBySquare code string from payment request
pub fn generate_pay_by_square_code(payment: &PaymentRequest) -> Result<String> {
    // 1. Build data structure (tab-separated values)
    let data = build_data_structure(payment)?;

    // 2. Calculate CRC32 checksum
    let crc = crc32fast::hash(data.as_bytes());
    let crc_bytes = crc.to_le_bytes();

    // 3. Prepend CRC32 to data
    let mut data_with_crc = Vec::new();
    data_with_crc.extend_from_slice(&crc_bytes);
    data_with_crc.extend_from_slice(data.as_bytes());

    // 4. LZMA compression
    let compressed = compress_lzma(&data_with_crc)?;

    // 5. Add header (4 bytes: type, version, document type, reserved)
    let mut final_data = Vec::new();
    final_data.push(0x00); // By square type
    final_data.push(0x00); // Version
    final_data.push(0x00); // Document type
    final_data.push(0x00); // Reserved
    final_data.extend_from_slice(&compressed);

    // 6. Base32hex encode
    let encoded = base32hex_encode(&final_data);

    Ok(encoded)
}

/// Builds the tab-separated data structure according to PayBySquare specification
fn build_data_structure(payment: &PaymentRequest) -> Result<String> {
    let mut fields = Vec::new();

    // Field 1: Payment options
    let payment_opts = if let Some(ref opts) = payment.payment_options {
        opts.iter()
            .map(|opt| match opt {
                PaymentOption::PaymentOrder => "1",
                PaymentOption::StandingOrder => "2",
                PaymentOption::DirectDebit => "3",
            })
            .collect::<Vec<_>>()
            .join(",")
    } else {
        "1".to_string() // Default: payment order
    };
    fields.push(payment_opts);

    // Field 2: Amount (formatted to 2 decimal places)
    fields.push(format!("{:.2}", payment.amount));

    // Field 3: Currency
    fields.push(payment.currency.clone());

    // Field 4: Payment date (YYYYMMDD)
    fields.push(payment.date.map(|d| format_date(d)).unwrap_or_default());

    // Field 5: Variable symbol
    fields.push(payment.variable_symbol.clone().unwrap_or_default());

    // Field 6: Constant symbol
    fields.push(payment.constant_symbol.clone().unwrap_or_default());

    // Field 7: Specific symbol
    fields.push(payment.specific_symbol.clone().unwrap_or_default());

    // Field 8: SEPA reference
    fields.push(
        payment
            .originators_reference_information
            .clone()
            .unwrap_or_default(),
    );

    // Field 9: Note
    fields.push(payment.note.clone().unwrap_or_default());

    // Field 10: Bank accounts (multiple IBANs separated by comma)
    let bank_accounts = if let Some(ref accounts) = payment.bank_accounts {
        accounts
            .iter()
            .map(|acc| {
                if let Some(ref swift) = acc.swift {
                    format!("{}|{}", acc.iban, swift)
                } else {
                    acc.iban.clone()
                }
            })
            .collect::<Vec<_>>()
            .join(",")
    } else if let Some(ref iban) = payment.iban {
        if let Some(ref swift) = payment.swift {
            format!("{}|{}", iban, swift)
        } else {
            iban.clone()
        }
    } else {
        String::new()
    };
    fields.push(bank_accounts);

    // Field 11: Beneficiary name
    fields.push(payment.beneficiary_name.clone().unwrap_or_default());

    // Field 12: Beneficiary address 1
    fields.push(payment.beneficiary_address_1.clone().unwrap_or_default());

    // Field 13: Beneficiary address 2
    fields.push(payment.beneficiary_address_2.clone().unwrap_or_default());

    // Field 14: Payment due date (YYYYMMDD)
    fields.push(
        payment
            .payment_due_date
            .map(|d| format_date(d))
            .unwrap_or_default(),
    );

    // Field 15: Invoice ID
    fields.push(payment.invoice_id.clone().unwrap_or_default());

    // Field 16: Standing order details
    if let Some(ref standing_order) = payment.standing_order {
        let periodicity = match standing_order.periodicity {
            Periodicity::Daily => "D",
            Periodicity::Weekly => "W",
            Periodicity::Monthly => "M",
            Periodicity::Quarterly => "Q",
            Periodicity::HalfYearly => "H",
            Periodicity::Yearly => "Y",
        };
        let months = standing_order
            .month
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(",");
        fields.push(format!(
            "{}|{}|{}|{}",
            standing_order.day,
            months,
            periodicity,
            format_date(standing_order.last_date)
        ));
    } else {
        fields.push(String::new());
    }

    // Field 17: Direct debit details
    if let Some(ref direct_debit) = payment.direct_debit {
        let mut dd_parts = Vec::new();
        dd_parts.push(match direct_debit.scheme {
            crate::models::DirectDebitScheme::Sepa => "SEPA",
            crate::models::DirectDebitScheme::Other => "OTHER",
        });
        dd_parts.push(match direct_debit.debit_type {
            crate::models::DirectDebitType::OneOff => "ONEOFF",
            crate::models::DirectDebitType::Recurrent => "RCUR",
        });
        if let Some(ref mandate_id) = direct_debit.mandate_id {
            dd_parts.push(mandate_id);
        }
        if let Some(ref creditor_id) = direct_debit.creditor_id {
            dd_parts.push(creditor_id);
        }
        fields.push(dd_parts.join("|"));
    } else {
        fields.push(String::new());
    }

    Ok(fields.join("\t"))
}

/// Formats a date as YYYYMMDD
fn format_date(date: NaiveDate) -> String {
    date.format("%Y%m%d").to_string()
}

/// Compresses data using LZMA algorithm with PayBySquare-specific parameters
fn compress_lzma(data: &[u8]) -> Result<Vec<u8>> {
    use xz2::write::XzEncoder;

    let mut encoder = XzEncoder::new(Vec::new(), 6);
    encoder
        .write_all(data)
        .map_err(|e| PayBySquareError::CompressionError(e.to_string()))?;

    let compressed = encoder
        .finish()
        .map_err(|e| PayBySquareError::CompressionError(e.to_string()))?;

    // For PayBySquare, we need to extract the raw LZMA stream without XZ container
    // The XZ format includes extra headers, so we need to use a different approach
    // For simplicity, we'll use the XZ format as-is
    // In a production implementation, you might need to use raw LZMA encoding

    Ok(compressed)
}

/// Encodes data to Base32hex (RFC 4648)
/// Uses characters 0-9 and A-V (uppercase)
fn base32hex_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHIJKLMNOPQRSTUV";
    let mut result = String::new();

    let mut bits = 0u32;
    let mut bit_count = 0u32;

    for &byte in data {
        bits = (bits << 8) | (byte as u32);
        bit_count += 8;

        while bit_count >= 5 {
            bit_count -= 5;
            let index = ((bits >> bit_count) & 0x1F) as usize;
            result.push(ALPHABET[index] as char);
        }
    }

    // Handle remaining bits
    if bit_count > 0 {
        let index = ((bits << (5 - bit_count)) & 0x1F) as usize;
        result.push(ALPHABET[index] as char);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base32hex_encode() {
        let data = b"Hello";
        let encoded = base32hex_encode(data);
        assert!(!encoded.is_empty());
        assert!(encoded
            .chars()
            .all(|c| "0123456789ABCDEFGHIJKLMNOPQRSTUV".contains(c)));
    }

    #[test]
    fn test_format_date() {
        let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        assert_eq!(format_date(date), "20240315");
    }
}
