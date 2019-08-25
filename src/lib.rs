#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
pub mod debug;
#[macro_use]
pub mod util;
pub mod addressing;
pub mod caller;
pub mod constant;
pub mod data;
pub mod proxy;
