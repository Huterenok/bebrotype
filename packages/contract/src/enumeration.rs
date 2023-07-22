use crate::*;
use near_sdk::json_types::U64;

#[near_bindgen]
impl Contract {
	pub fn get_owner(&self) -> &AccountId {
		&self.owner_id
	}

	pub fn get_all_games_by_id(&self, account_id: AccountId) -> Vec<Game>{
		let games = self.games_by_id.get(&account_id).unwrap();

		games.iter().map(|(_, game)| game).collect()
	}
	
	pub fn get_game_by_id(&self, account_id: AccountId, game_id: U64) -> Option<Game> {
		let user_games = self.games_by_id.get(&account_id).unwrap();
		let game = user_games.get(&game_id.0);

		game
	}
}