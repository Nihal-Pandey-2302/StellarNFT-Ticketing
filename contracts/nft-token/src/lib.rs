#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, Env, Symbol, Address, String, log};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotOwner = 1,
    NoOwner = 2,
}

#[contract]
pub struct WCNFTMatch;

#[contractimpl]
impl WCNFTMatch {
    pub fn initialize(env: Env, admin: Address, name: String, symbol: Symbol) {
        log!(&env, "Initializing contract with admin: {}, name: {}, symbol: {}:", admin, name, symbol);
    }

    /// Mint a seat token for World Cup Finals
    pub fn mint(env: Env, owner: Address, seat: u32) -> u32 {
        // Check if the seat is occupied
        if env.storage().persistent().has(&seat) {
            panic!("This seat is already occupied");
        }
        env.storage().persistent().set(&seat, &owner);
        seat
    }

    /// Owner of the World Cup match NFT Ticket token
    pub fn owner_of(env: Env, seat: u32) -> Option<Address> {
        env.storage().persistent().get(&seat)
    }

    /// Transfer Address "from" to Address "to"
    pub fn transfer(env: Env, seat: u32, from: Address, to: Address) -> Result<u32, Error> {
        let current_owner: Option<Address> = env.storage().persistent().get(&seat);
        log!(&env, "Current owner of seat {}: {:?}", seat, current_owner);

        match current_owner {
            Some(owner_address) if owner_address == from => {
                env.storage().persistent().set(&seat, &to);
                Ok(seat)
            },
            Some(_) => Err(Error::NotOwner),
            None => Err(Error::NoOwner),
        }
    }
}

mod test;
