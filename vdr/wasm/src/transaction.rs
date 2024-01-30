use indy_besu_vdr::{SignatureData, Transaction, TransactionEndorsingData};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use crate::error::Result;

#[wasm_bindgen(js_name = Transaction)]
pub struct TransactionWrapper(pub(crate) Rc<Transaction>);

#[wasm_bindgen(js_class = Transaction)]
impl TransactionWrapper {
    pub fn to(&self) -> Result<String> {
        Ok(self.0.to.to_string())
    }

    #[wasm_bindgen(js_name = getSigningBytes)]
    pub fn get_signing_bytes(&self) -> Result<Vec<u8>> {
        let bytes = self.0.get_signing_bytes().unwrap();
        Ok(bytes)
    }

    #[wasm_bindgen(js_name = setSignature)]
    pub fn set_signature(&mut self, signature_data: JsValue) -> Result<()> {
        let signature_data: SignatureData = serde_wasm_bindgen::from_value(signature_data)?;
        self.0.set_signature(signature_data);
        Ok(())
    }
}

#[wasm_bindgen(js_name = TransactionEndorsingData)]
pub struct TransactionEndorsingDataWrapper(pub(crate) Rc<TransactionEndorsingData>);

#[wasm_bindgen(js_class = TransactionEndorsingData)]
impl TransactionEndorsingDataWrapper {
    #[wasm_bindgen(js_name = getSigningBytes)]
    pub fn get_signing_bytes(&self) -> Result<Vec<u8>> {
        let bytes = self.0.get_signing_bytes().unwrap();
        Ok(bytes)
    }
}
