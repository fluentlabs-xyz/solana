//! `Hash` Javascript interface
#![cfg(target_arch = "wasm32")]
#![allow(non_snake_case)]
use {
    crate::hash::*,
};
use alloc::format;
use alloc::vec;
// #[cfg(feature = "wbg")]
// use crate::wasm::display_to_jsvalue;
#[cfg(feature = "wbg")]
use js_sys::{Array, Uint8Array};
#[cfg(feature = "wbg")]
use wasm_bindgen::{prelude::*, JsCast};
use crate::alloc::string::ToString;
use alloc::boxed::Box;
use alloc::string::String;

// #[cfg_attr(wbg, feature(wasm_bindgen))]
// #[wasm_bindgen]
impl Hash {
    // /// Create a new Hash object
    // ///
    // /// * `value` - optional hash as a base58 encoded string, `Uint8Array`, `[number]`
    // #[cfg(feature = "wbg")]
    // #[cfg_attr(wbg, feature(wasm_bindgen(skip)))]
    // #[wasm_bindgen(constructor)]
    // pub fn constructor(value: JsValue) -> Result<Hash, JsValue> {
    //     if let Some(base58_str) = value.as_string() {
    //         base58_str.parse::<Hash>().map_err(display_to_jsvalue)
    //     } else if let Some(uint8_array) = value.dyn_ref::<Uint8Array>() {
    //         Ok(Hash::new(&uint8_array.to_vec()))
    //     } else if let Some(array) = value.dyn_ref::<Array>() {
    //         let mut bytes = vec![];
    //         let iterator = js_sys::try_iter(&array.values())?.expect("array to be iterable");
    //         for x in iterator {
    //             let x = x?;
    //
    //             if let Some(n) = x.as_f64() {
    //                 if n >= 0. && n <= 255. {
    //                     bytes.push(n as u8);
    //                     continue;
    //                 }
    //             }
    //             return Err(format!("Invalid array argument: {:?}", x).into());
    //         }
    //         Ok(Hash::new(&bytes))
    //     } else if value.is_undefined() {
    //         Ok(Hash::default())
    //     } else {
    //         Err("Unsupported argument".into())
    //     }
    // }

    /// Return the base58 string representation of the hash
    pub fn toString(&self) -> String {
        self.to_string()
    }

    /// Checks if two `Hash`s are equal
    pub fn equals(&self, other: &Hash) -> bool {
        self == other
    }

    /// Return the `Uint8Array` representation of the hash
    pub fn toBytes(&self) -> Box<[u8]> {
        self.0.clone().into()
    }
}
