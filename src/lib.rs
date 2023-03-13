pub mod webcrypto;
mod session;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn thing() {
    let key = webcrypto::aes_gcm::Key::new_128().await.unwrap();
    let _cipher = key.encrypt(&[1, 2, 3], &[4, 5, 6]).await.unwrap();
}
