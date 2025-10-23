use crate::errors::{PayBySquareError, Result};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use qrcode::QrCode;

/// Generates a QR code image from a code string
pub fn generate_qr_image(code: &str, size: u32) -> Result<Vec<u8>> {
    // Generate QR code
    let qr = QrCode::new(code.as_bytes()).map_err(|e| PayBySquareError::QrError(e.to_string()))?;

    // Convert to image
    let qr_image = qr.render::<image::Luma<u8>>().build();

    // Resize to desired size
    let resized =
        image::imageops::resize(&qr_image, size, size, image::imageops::FilterType::Nearest);

    // Convert to RGBA for consistency
    let rgba_image = DynamicImage::ImageLuma8(resized).to_rgba8();

    // Encode to PNG
    let mut png_data = Vec::new();
    rgba_image
        .write_to(
            &mut std::io::Cursor::new(&mut png_data),
            image::ImageFormat::Png,
        )
        .map_err(|e| PayBySquareError::ImageError(e.to_string()))?;

    Ok(png_data)
}

/// Adds a frame around the QR code
pub fn add_frame(qr_data: Vec<u8>, frame_data: Option<&[u8]>) -> Result<Vec<u8>> {
    // If no frame data provided, return QR as-is
    let frame_bytes = match frame_data {
        Some(data) => data,
        None => return Ok(qr_data),
    };

    // Load QR code image
    let qr_img = image::load_from_memory(&qr_data)
        .map_err(|e| PayBySquareError::ImageError(format!("Failed to load QR image: {}", e)))?;

    // Load frame image
    let frame_img = image::load_from_memory(frame_bytes)
        .map_err(|e| PayBySquareError::ImageError(format!("Failed to load frame image: {}", e)))?;

    // Calculate QR position (centered at 85% of frame size)
    let frame_width = frame_img.width();
    let frame_height = frame_img.height();

    let target_qr_size = ((frame_width.min(frame_height) as f32) * 0.85) as u32;

    // Resize QR to fit in frame
    let qr_resized = image::imageops::resize(
        &qr_img,
        target_qr_size,
        target_qr_size,
        image::imageops::FilterType::Nearest,
    );

    // Calculate center position
    let x_offset = (frame_width - target_qr_size) / 2;
    let y_offset = (frame_height - target_qr_size) / 2;

    // Create a new image with the frame
    let mut result = frame_img.to_rgba8();

    // Overlay QR code on frame
    image::imageops::overlay(&mut result, &qr_resized, x_offset as i64, y_offset as i64);

    // Encode to PNG
    let mut png_data = Vec::new();
    result
        .write_to(
            &mut std::io::Cursor::new(&mut png_data),
            image::ImageFormat::Png,
        )
        .map_err(|e| PayBySquareError::ImageError(e.to_string()))?;

    Ok(png_data)
}

/// Generates a simple frame if none exists
pub fn generate_default_frame(size: u32) -> Vec<u8> {
    // Create a white background with a border
    let mut img: RgbaImage = ImageBuffer::new(size, size);

    // Fill with white
    for pixel in img.pixels_mut() {
        *pixel = Rgba([255, 255, 255, 255]);
    }

    // Draw border (10 pixels thick)
    let border_thickness = 10;
    for y in 0..size {
        for x in 0..size {
            if x < border_thickness
                || x >= size - border_thickness
                || y < border_thickness
                || y >= size - border_thickness
            {
                img.put_pixel(x, y, Rgba([0, 102, 204, 255])); // Blue border
            }
        }
    }

    // Encode to PNG
    let mut png_data = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut png_data),
        image::ImageFormat::Png,
    )
    .unwrap();

    png_data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_qr_image() {
        let result = generate_qr_image("TEST", 300);
        assert!(result.is_ok());
        let png_data = result.unwrap();
        assert!(!png_data.is_empty());
    }

    #[test]
    fn test_generate_default_frame() {
        let frame = generate_default_frame(400);
        assert!(!frame.is_empty());
    }
}
