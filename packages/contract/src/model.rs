use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Game {
	id: u64,
	block_timestamp: u64,
	value: u128,
}

impl Game {
	pub fn new(id: u64, block_timestamp: u64, value: u128,) -> Self {
		Game { 
			id, 
			block_timestamp, 
			value
		}
	}
}
