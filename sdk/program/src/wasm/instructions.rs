//! The `Instructions` struct is a workaround for the lack of Vec<T> support in wasm-bindgen
//! (ref: https://github.com/rustwasm/wasm-bindgen/issues/111)
#![cfg(target_arch = "wasm32")]
use crate::instruction::Instruction;

#[cfg(feature="wbg")]
use wasm_bindgen::prelude::*;

#[cfg_attr(wbg, feature(wasm_bindgen))]
// #[wasm_bindgen]
#[derive(Default)]
pub struct Instructions {
    instructions: Vec<Instruction>,
}
use alloc::vec::Vec;

#[cfg_attr(wbg, feature(wasm_bindgen))]
// #[wasm_bindgen]
impl Instructions {
    #[cfg_attr(wbg, feature(wasm_bindgen(skip)))]
    // #[wasm_bindgen(constructor)]
    pub fn constructor() -> Instructions {
        Instructions::default()
    }

    pub fn push(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
}

impl From<Instructions> for Vec<Instruction> {
    fn from(instructions: Instructions) -> Self {
        instructions.instructions
    }
}
