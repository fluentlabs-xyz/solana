// //! solana-program Javascript interface
// #![cfg(target_arch = "wasm32")]
// use wasm_bindgen::prelude::*;

pub mod hash;
pub mod instructions;
pub mod pubkey;
pub mod system_instruction;
//
// /// Initialize Javascript logging and panic handler
// #[cfg_attr(wbg, feature(wasm_bindgen))]
// #[wasm_bindgen]
// pub fn solana_program_init() {
//     use spin::Once;
//     static INIT: Once = Once::new();
//
//     INIT.call_once(|| {
//         alloc::panic::set_hook(Box::new(console_error_panic_hook::hook));
//         console_log::init_with_level(log::Level::Info).unwrap();
//     });
// }

// pub fn display_to_jsvalue<T: core::fmt::Display>(display: T) -> JsValue {
//     display.to_string().into()
// }
