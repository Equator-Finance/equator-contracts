#![no_std]

mod types;
mod contract;

#[cfg(test)]
mod test;

pub use contract::EquatorEscrowContract;

#[cfg(any(test, feature = "testutils"))]
pub use contract::EquatorEscrowContractClient;
