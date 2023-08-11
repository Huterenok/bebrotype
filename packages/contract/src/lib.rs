use near_sdk::collections::{UnorderedMap, LookupMap};
use near_sdk::{near_bindgen, PanicOnDefault, AccountId};
use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::{env};

mod model;
mod play;
mod enumeration;

use model::Game;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
	pub owner_id: AccountId,
	pub games_by_id: LookupMap<AccountId, UnorderedMap<u64, Game>>,
	pub global_id_counter: u64
}

#[near_bindgen]
impl Contract { 
	#[init]
	pub fn new(owner_id: AccountId) -> Self {
		Contract {
			owner_id,	
			games_by_id: LookupMap::new(b"s".to_vec()),
			global_id_counter: 0 
		}
	}

	pub fn set_owner(&mut self, new_owner: AccountId) {
		if self.owner_id == env::signer_account_id() {
			self.owner_id = new_owner
		} else {
			env::panic_str("You are not an owner");
		}
	}
}