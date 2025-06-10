use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use wasm_bindgen::prelude::*;


mod helpers;
use helpers::image_to;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn get_preview(image: &[u8]) -> Result<Vec<u8>, JsError> {
    let mut converted_image: Vec<u8> = Vec::new();
    let encoder = PngEncoder::new_with_quality(
        &mut converted_image,
        CompressionType::Fast,
        FilterType::NoFilter,
    );
    image_to(image.to_vec(), encoder)?;
    Ok(converted_image)
}

#[wasm_bindgen]
pub fn convert_image(
    new_format: SupportedTypes,
    image: &[u8],
    quality: Option<u8>,
) -> Result<Vec<u8>, JsError> {
    log(&format!("{:?}", new_format));
    let quality = quality.unwrap_or(100);
    let image = image.to_vec();

    match new_format {
        SupportedTypes::Bmp => helpers::image_to_bmp(image),
        SupportedTypes::Jpeg => helpers::image_to_jpeg(image, quality),
        SupportedTypes::Png=> helpers::image_to_png(image),
        SupportedTypes::Webp=> helpers::image_to_webp(image),
        SupportedTypes::Avif=> helpers::image_to_avif(image, quality),
        SupportedTypes::Ico=> helpers::image_to_ico(image)
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
