#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!("./bindings.rs");

pub const BDADDR_ANY: *const bdaddr_t = &BDADDR_ANY_VAL;
const BDADDR_ANY_VAL: bdaddr_t = bdaddr_t{ b: [0,0,0,0,0,0]};