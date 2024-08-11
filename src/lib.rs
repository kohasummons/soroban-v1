#![no_std]
use soroban_sdk::{ contract, contractimpl};

#[contract]
pub struct Calculator;

#[contractimpl]
impl Calculator {
    pub fn add(left: i64, right: i64) -> i64 {
        left + right
    }
}

mod test;
