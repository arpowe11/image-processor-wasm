use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::{ encode, decode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;

// Note:
// * Anywhere in the code, log(); is used to call the JS console.log() func

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"Grayscale called".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());

    img = img.grayscale();
    log(&"Grayscale effect applied".into());

    let mut buffer = vec![];
    img.write_to(&mut buffer, Png).unwrap();
    log(&"New image written".into());

    let encoded_image = encode(&buffer);
    let data_url: String = format!(
        "data:image/png;base64,{}",
        encoded_image
    );

    data_url // return data_url
}


