#![allow(clippy::arithmetic_side_effects)]
//! Utilities for the [borsh] serialization format, version 0.9.
//!
//! This file is provided for backwards compatibility with types that still use
//! borsh 0.9, even though this crate canonically uses borsh 0.10.
//!
//! [borsh]: https://borsh.io/

use alloc::borrow::ToOwned;
use alloc::string::String;
use core::ops::Deref;
use borsh0_9::BorshSerialize;
use borsh0_9::schema::{Declaration, Definition, Fields};
use hashbrown::HashMap;
use {
    crate::borsh::{
        impl_get_instance_packed_len, impl_get_packed_len_v0, impl_try_from_slice_unchecked,
    },
    borsh0_9::maybestd::io,
};

// ///   Get the worst-case packed length for the given BorshSchema
// ///
// ///   Note: due to the serializer currently used by Borsh, this function cannot
// ///   be used on-chain in the Solana SBF execution environment.
// #[deprecated(
//     since = "1.17.0",
//     note = "Please upgrade to Borsh 1.X and use `borsh1::get_packed_len` instead"
// )]
// pub fn get_packed_len<S: borsh0_9::BorshSchema>() -> usize {
//     let borsh0_9::schema::BorshSchemaContainer { declaration, definitions } =
//         &S::schema_container();
//     
//     // TODO get rid of pumping over
//     let mut hashbrown_definitions = hashbrown::hash_map::HashMap::with_capacity(definitions.capacity());
//     definitions.iter().for_each(|(decl, def)| {
//         let def_new = match def {
//             Definition::Array { length, elements } => {
//                 borsh0_9::schema::Definition::Array {length: *length, elements: elements.clone()}
//             }
//             Definition::Sequence { elements } => {
//                 borsh0_9::schema::Definition::Sequence {elements: elements.clone()}
//             }
//             Definition::Tuple { elements } => {borsh0_9::schema::Definition::Tuple {elements: elements.clone()}}
//             Definition::Enum { variants } => {borsh0_9::schema::Definition::Enum {variants: variants.clone()}}
//             Definition::Struct { fields } => {borsh0_9::schema::Definition::Struct { fields: match fields {
//                 Fields::NamedFields(v) => {Fields::NamedFields(v.clone())}
//                 Fields::UnnamedFields(v) => {Fields::UnnamedFields(v.clone())}
//                 Fields::Empty => {Fields::Empty}
//             } }}
//         };
//         hashbrown_definitions.insert(decl.clone(), def_new).unwrap();
//     });
//     get_declaration_packed_len(declaration, &hashbrown_definitions)
// }
// ///   Get packed length for the given BorshSchema Declaration
// fn get_declaration_packed_len(
//     declaration: &str,
//     definitions: &hashbrown::HashMap<borsh0_9::schema::Declaration, borsh0_9::schema::Definition>,
// ) -> usize {
//     match definitions.get(declaration) {
//         Some(borsh0_9::schema::Definition::Array { length, elements }) => {
//             *length as usize * get_declaration_packed_len(elements, definitions)
//         }
//         Some(borsh0_9::schema::Definition::Enum { variants }) => {
//             1 + variants
//                 .iter()
//                 .map(|(_, declaration)| get_declaration_packed_len(declaration, definitions))
//                 .max()
//                 .unwrap_or(0)
//         }
//         Some(borsh0_9::schema::Definition::Struct { fields }) => match fields {
//             borsh0_9::schema::Fields::NamedFields(named_fields) => named_fields
//                 .iter()
//                 .map(|(_, declaration)| get_declaration_packed_len(declaration, definitions))
//                 .sum(),
//             borsh0_9::schema::Fields::UnnamedFields(declarations) => declarations
//                 .iter()
//                 .map(|declaration| get_declaration_packed_len(declaration, definitions))
//                 .sum(),
//             borsh0_9::schema::Fields::Empty => 0,
//         },
//         Some(borsh0_9::schema::Definition::Sequence {
//                  elements: _elements,
//              }) => panic!("Missing support for Definition::Sequence"),
//         Some(borsh0_9::schema::Definition::Tuple { elements }) => elements
//             .iter()
//             .map(|element| get_declaration_packed_len(element, definitions))
//             .sum(),
//         None => match declaration {
//             "bool" | "u8" | "i8" => 1,
//             "u16" | "i16" => 2,
//             "u32" | "i32" => 4,
//             "u64" | "i64" => 8,
//             "u128" | "i128" => 16,
//             "nil" => 0,
//             _ => panic!("Missing primitive type: {declaration}", declaration = declaration),
//         },
//     }
// }

impl_get_packed_len_v0!(
    borsh0_9,
    #[deprecated(
        since = "1.17.0",
        note = "Please upgrade to Borsh 1.X and use `borsh1::get_packed_len` instead"
    )]
);
impl_try_from_slice_unchecked!(
    borsh0_9,
    io,
    #[deprecated(
        since = "1.17.0",
        note = "Please upgrade to Borsh 1.X and use `borsh1::try_from_slice_unchecked` instead"
    )]
);
impl_get_instance_packed_len!(
    borsh0_9,
    io,
    #[deprecated(
        since = "1.17.0",
        note = "Please upgrade to Borsh 1.X and use `borsh1::get_instance_packed_len` instead"
    )]
);

// #[cfg(test)]
// #[allow(deprecated)]
// mod tests {
//     use crate::alloc::string::ToString;
//     use {crate::borsh::impl_tests, borsh0_9::maybestd::io};
//     use alloc::vec;
//     impl_tests!(borsh0_9, io);
// }
