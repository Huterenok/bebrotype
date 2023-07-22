use near_sdk::{near_bindgen, Promise};
use near_sdk::{env};

use crate::*;

#[near_bindgen]
impl Contract {
	pub fn play(&mut self) -> Option<Game> {
		let account_id = env::predecessor_account_id();
		let random_seed = env::random_seed();
		let block_timestamp = env::block_timestamp();

		let value = match random_seed[0] {
			0..=10 => 10f64,
			11..=30 => 5f64,
			31..=70 => 2.5f64,
			71..=150 => 1f64,
			_ => 0.5f64
		} * 10f64.powi(24);

		self.global_id_counter += 1;

		let game = Game::new(self.global_id_counter, block_timestamp, value as u128);
		let mut user_games = self.games_by_id.get(&account_id).unwrap_or(UnorderedMap::new(b"g"));
		user_games.insert(&self.global_id_counter, &game);
		self.games_by_id.insert(&account_id, &user_games);

		env::log_str(&format!("Game was successfully created!\n{:#?}", game));

		Promise::new(account_id).transfer(value as u128);

		Some(game)
	}
}