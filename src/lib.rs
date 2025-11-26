use std::io::Cursor;

use image::{
    codecs::png::{CompressionType, FilterType, PngEncoder},
    guess_format,
    imageops::FilterType::Lanczos3,
    load_from_memory_with_format,
};
use wasm_bindgen::prelude::*;

mod helpers;
use helpers::image_to;

#[wasm_bindgen(js_name = getPreview)]
pub fn get_preview(image: &[u8]) -> Result<Vec<u8>, JsError> {
    let mut converted_image: Vec<u8> = Vec::new();
    let encoder = PngEncoder::new_with_quality(
        &mut converted_image,
        CompressionType::Fast,
        FilterType::NoFilter,
    );
    image_to(image, encoder)?;
    Ok(converted_image)
}

#[wasm_bindgen(js_name = resizeImage)]
pub fn resize_image(image: &[u8], width: u32, height: u32) -> Result<Vec<u8>, JsError> {
    let image_format = match guess_format(image) {
        Ok(format) => format,
        Err(message) => return Err(JsError::new(&message.to_string())),
    };
    let dynamic_image = load_from_memory_with_format(image, image_format)?;

    let new_image = dynamic_image.resize(width, height, Lanczos3);

    let mut bytes: Vec<u8> = Vec::new();
    let mut buffer = Cursor::new(&mut bytes);

    new_image.write_to(&mut buffer, image_format)?;
    Ok(bytes)
}

#[wasm_bindgen(js_name = convertImage)]
pub fn convert_image(
    #[wasm_bindgen(js_name = newFormat)] new_format: SupportedTypes,
    image: &[u8],
    quality: Option<u8>,
) -> Result<Vec<u8>, JsError> {
    let quality = quality.unwrap_or(100);

    match new_format {
        SupportedTypes::Bmp => helpers::image_to_bmp(image),
        SupportedTypes::Jpeg => helpers::image_to_jpeg(image, quality),
        SupportedTypes::Png => helpers::image_to_png(image),
        SupportedTypes::Webp => helpers::image_to_webp(image),
        SupportedTypes::Avif => helpers::image_to_avif(image, quality),
        SupportedTypes::Ico => helpers::image_to_ico(image),
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub enum SupportedTypes {
    Bmp,
    Jpeg,
    Png,
    Webp,
    Avif,
    Ico,
}
