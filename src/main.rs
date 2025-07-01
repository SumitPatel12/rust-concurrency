#![allow(unused_imports)]
use std::fs;
use std::io::Read;
use std::sync::Arc;
use std::thread::{self};

pub mod atomics;
pub mod crust_of_rust;
pub mod interior_mutability;
pub mod lock;
pub mod memory_ordering;
pub mod parking_and_condition_variables;
pub mod spin_lock_guard;
pub mod spin_lock_guard_without_lifetime;

fn main() {
    crust_of_rust::ref_cell::test_ref_cell();
}
