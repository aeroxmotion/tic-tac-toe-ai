use crate::{
	board::Board,
	player::{Player, PlayerScore},
};

type HeuristicScore = (usize, PlayerScore);

const MAX_DEPTH: usize = 8;
const LESS_PLAYER_SCORE: HeuristicScore = (0, PlayerScore::Less);
const GREATER_PLAYER_SCORE: HeuristicScore = (0, PlayerScore::Greater);

pub fn move_ai(board: &mut Board, ai_player: Player, optimized: bool) {
	if !board.finished() {
		let (action, _) = if !optimized {
			minimax(board.clone(), MAX_DEPTH, ai_player)
		} else {
			alphabeta(
				board.clone(),
				LESS_PLAYER_SCORE,
				GREATER_PLAYER_SCORE,
				MAX_DEPTH,
				ai_player,
			)
		};

		board.place_at(action, ai_player).unwrap();
	}
}

// Thanks to: https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning#Pseudocode
fn alphabeta(
	board: Board,
	mut α: HeuristicScore,
	mut β: HeuristicScore,
	depth: usize,
	player: Player,
) -> HeuristicScore {
	if depth == 0 || board.finished() {
		return (0, board.score());
	}

	match player {
		Player::X => {
			let mut score = LESS_PLAYER_SCORE;

			for i in board.empty_positions() {
				let next_score =
					alphabeta(result(board.clone(), i, player), α, β, depth - 1, Player::O);

				if next_score.1 >= score.1 {
					score = (i, next_score.1);
				}

				if score.1 > β.1 {
					break;
				}

				if score.1 >= α.1 {
					α = score;
				}
			}

			return score;
		}
		_ => {
			let mut score = GREATER_PLAYER_SCORE;

			for i in board.empty_positions() {
				let next_score =
					alphabeta(result(board.clone(), i, player), α, β, depth - 1, Player::X);

				if next_score.1 <= score.1 {
					score = (i, next_score.1);
				}

				if score.1 < α.1 {
					break;
				}

				if score.1 <= β.1 {
					β = score;
				}
			}

			return score;
		}
	}
}

fn minimax(board: Board, depth: usize, player: Player) -> HeuristicScore {
	if depth == 0 || board.finished() {
		return (0, board.score());
	}

	let mut score = match player {
		Player::O => GREATER_PLAYER_SCORE,
		_ => LESS_PLAYER_SCORE,
	};

	for i in board.empty_positions() {
		let next_state = result(board.clone(), i, player);
		let next_score = minimax(
			next_state,
			depth - 1,
			match player {
				Player::O => Player::X,
				_ => Player::O,
			},
		);

		let result = match player {
			Player::O => next_score.1 <= score.1,
			_ => next_score.1 >= score.1,
		};

		if result {
			score = (i, next_score.1);
		}
	}

	score
}

fn result(mut board: Board, action: usize, player: Player) -> Board {
	board.place_at(action, player).unwrap();
	board
}
