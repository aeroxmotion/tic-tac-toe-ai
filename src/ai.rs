use crate::{
	board::Board,
	player::{Player, PlayerScore},
};

enum MinimaxMode {
	Min,
	Max,
}

pub fn move_ai(board: &mut Board) {
	if !board.finished() {
		let (action, _) = minimax(board.clone(), MinimaxMode::Max);
		board.place_at(action, Player::X).unwrap();
	}
}

fn minimax(board: Board, mode: MinimaxMode) -> (usize, PlayerScore) {
	if board.finished() {
		return (0, board.score());
	}

	let mut score = (
		0,
		match mode {
			MinimaxMode::Min => PlayerScore::Greater,
			MinimaxMode::Max => PlayerScore::Less,
		},
	);

	for i in board.empty_positions() {
		let next_state = result(
			board.clone(),
			i,
			match mode {
				MinimaxMode::Min => Player::O,
				MinimaxMode::Max => Player::X,
			},
		);
		let next_score = minimax(
			next_state,
			match mode {
				MinimaxMode::Min => MinimaxMode::Max,
				MinimaxMode::Max => MinimaxMode::Min,
			},
		);

		let result = match mode {
			MinimaxMode::Min => next_score.1 < score.1,
			MinimaxMode::Max => next_score.1 > score.1,
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
