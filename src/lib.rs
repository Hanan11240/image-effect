use wasm_bindgen::convert::IntoWasmAbi;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::{decode,encode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
#[wasm_bindgen]
pub fn grayscale(encoded_file:&str) -> String{
    // log(&"grayscale called".into());
    let base64_to_vector:Vec<u8> = decode(encoded_file).unwrap();
    // log(&"Image decode".into());
    let mut img  = load_from_memory(&base64_to_vector).unwrap();
    // log(&"Image loaded".into());
    img = img.grayscale();
    // log(&"Grayscale effect applied".into());
    let  mut buffer = vec![];
    img.write_to(&mut buffer,Png).unwrap();
    // log(&"New Image written".into());
    let endoded_img = encode(&buffer);
    let data_url = format!("data:image/png;base64,{}",endoded_img);
    data_url
}