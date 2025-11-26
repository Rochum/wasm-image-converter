use image::codecs::*;
use image::{guess_format, load_from_memory_with_format, ImageEncoder};
use wasm_bindgen::JsError;

pub fn image_to(image: &[u8], encoder: impl ImageEncoder) -> Result<(), JsError> {
    let image_format = match guess_format(&image) {
        Ok(format) => format,
        Err(message) => return Err(JsError::new(&message.to_string())),
    };
    let dynamic_image = load_from_memory_with_format(&image, image_format)?;
    dynamic_image.write_with_encoder(encoder)?;
    Ok(())
}
pub fn image_to_png(image: &[u8]) -> Result<Vec<u8>, JsError> {
    let mut converted_image: Vec<u8> = Vec::new();
    let encoder = png::PngEncoder::new_with_quality(
        &mut converted_image,
        png::CompressionType::Best,
        png::FilterType::NoFilter,
    );
    image_to(image, encoder)?;
    Ok(converted_image)
}

pub fn image_to_jpeg(image: &[u8], quaity: u8) -> Result<Vec<u8>, JsError> {
    let mut converted_image: Vec<u8> = Vec::new();
    let encoder = jpeg::JpegEncoder::new_with_quality(&mut converted_image, quaity);
    image_to(image, encoder)?;
    Ok(converted_image)
}

pub fn image_to_webp(image: &[u8]) -> Result<Vec<u8>, JsError> {
    let mut converted_image: Vec<u8> = Vec::new();
    let encoder = webp::WebPEncoder::new_lossless(&mut converted_image);
    image_to(image, encoder)?;
    Ok(converted_image)
}

pub fn image_to_avif(image: &[u8], quality: u8) -> Result<Vec<u8>, JsError> {
    let speed = 10;
    let mut converted_image: Vec<u8> = Vec::new();
    let encoder = avif::AvifEncoder::new_with_speed_quality(&mut converted_image, speed, quality);
    image_to(image, encoder)?;
    Ok(converted_image)
}

pub fn image_to_ico(image: &[u8]) -> Result<Vec<u8>, JsError> {
    let mut converted_image: Vec<u8> = Vec::new();
    let encoder = ico::IcoEncoder::new(&mut converted_image);
    image_to(image, encoder)?;
    Ok(converted_image)
}

pub fn image_to_bmp(image: &[u8]) -> Result<Vec<u8>, JsError> {
    let mut converted_image: Vec<u8> = Vec::new();
    let encoder = bmp::BmpEncoder::new(&mut converted_image);
    image_to(image, encoder)?;
    Ok(converted_image)
}
