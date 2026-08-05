#![no_std]

mod types;
mod contract;

#[cfg(test)]
mod test;

pub use contract::EquatorEscrowContractClient;
