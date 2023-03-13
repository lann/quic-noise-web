use js_sys::{JsString, Uint8Array};
use wasm_bindgen::{
    convert::{FromWasmAbi, IntoWasmAbi, RefFromWasmAbi},
    prelude::*,
};

#[wasm_bindgen(module = "/js/webcrypto.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn aesGcmGenerateKey(bits: u16) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch)]
    async fn aesGcmEncrypt(key: &JsValue, iv: &[u8], data: &[u8]) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch)]
    async fn aesGcmDecrypt(key: &JsValue, iv: &[u8], data: &[u8]) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch)]
    async fn aesGcmDeriveKeyHkdfSha256(salt: &[u8], info: &[u8], ) -> Result<JsValue, JsValue>;
}

mod aes_gcm {
    use super::*;

    pub struct Key {
        key: u32,
    }

    impl Drop for Key {
        fn drop(&mut self) {
            unsafe { drop(JsValue::from_abi(self.key)) }
        }
    }

    impl Key {
        pub async fn new_128() -> Result<Self, String> {
            Self::new(128).await
        }

        pub async fn new_192() -> Result<Self, String> {
            Self::new(192).await
        }

        pub async fn new_256() -> Result<Self, String> {
            Self::new(256).await
        }

        async fn new(bits: u16) -> Result<Self, String> {
            let key = aesGcmGenerateKey(bits).await.map_err(value_to_string)?;
            let key = key.into_abi();
            Ok(Self { key })
        }

        pub async fn encrypt(&self, iv: &[u8], data: &[u8]) -> Result<Vec<u8>, String> {
            let key = unsafe { JsValue::ref_from_abi(self.key) };
            let output = aesGcmEncrypt(&key, iv, data)
                .await
                .map_err(value_to_string)?;
            Ok(Uint8Array::new(&output).to_vec())
        }

        pub async fn decrypt(&self, iv: &[u8], data: &[u8]) -> Result<Vec<u8>, String> {
            let key = unsafe { JsValue::ref_from_abi(self.key) };
            let output = aesGcmDecrypt(&key, iv, data)
                .await
                .map_err(value_to_string)?;
            Ok(Uint8Array::new(&output).to_vec())
        }
    }
}

fn value_to_string(err: JsValue) -> String {
    JsString::from(err).into()
}
