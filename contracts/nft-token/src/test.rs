#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String, Symbol,  Address, symbol_short};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[test]

fn test_initialize() {
    // Create a mock environment
    let env = Env::default();
    let name = String::from_str(&env, "WCNFTMatch");
    let symbol = symbol_short!("WC");
    let address = String::from_str(&env,"GCC267YZPB4KLA264X5LXFXLAAUC3LBY2K72OJEIXKLLJ5UO764G2WX3");
    WCNFTMatch::initialize(env, Address::from_string(&address), name, symbol);
}
#[test]
fn test_mint_and_owner_of() {
    // Create a mock environment
    let env = Env::default();

    // Mint a seat token
    let address_str = String::from_str(&env, "GCC267YZPB4KLA264X5LXFXLAAUC3LBY2K72OJEIXKLLJ5UO764G2WX3");
    let owner = Address::from_string( &address_str);
    let seat = 1;
    WCNFTMatch::mint(env.clone(), owner.clone(), seat);

    // Check if the owner of the seat is correct
    let owner_of_seat = WCNFTMatch::owner_of(env.clone(), seat);
    assert_eq!(owner_of_seat, Some(owner.clone()));
}

#[test]
fn test_transfer() {
    let env = Env::default();

    let address_str = String::from_str(&env, "GCC267YZPB4KLA264X5LXFXLAAUC3LBY2K72OJEIXKLLJ5UO764G2WX3");
    let owner = Address::from_string( &address_str);
    let seat = 1;

    let contract_id = env.register_contract(None, WCNFTMatch);
    WCNFTMatch::mint(env.clone(), owner.clone(), seat);

    let new_address_str = String::from_str(&env, "GCM3XYZP456ZP6LJ8K4J6H5L4X5LXFXLAAUC3LBY2K72OJEIXKLLJ5UO764G2WXY");
    let new_owner = Address::from_string( &new_address_str);

    let transfer_result = WCNFTMatch::transfer(env.clone(), seat, owner.clone(), new_owner.clone());
    assert!(transfer_result.is_ok());

    let owner_of_seat = WCNFTMatch::owner_of(env.clone(), seat);
    assert_eq!(owner_of_seat, Some(new_owner));
}

