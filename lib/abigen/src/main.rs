use ethers::{
    prelude::{abigen, Abigen},
    providers::{Http, Provider},
    types::Address,
};
use ethers::prelude::*;
use ethers::solc::artifacts::ContractBytecode;


pub fn contract_bytecode() {
    let artifact: &str = include_str!("../../contracts/cozy-protocol-v2/out/SetTest.sol");
    println!({artifact});
}


#[cfg(test)]
mod tests {
    #[test]
    fn contract_bytecode();
}
