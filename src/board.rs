use crate::player::{Player, PlayerScore};

#[derive(Clone)]
pub struct Board(Vec<Player>);

impl Default for Board {
	fn default() -> Self {
		Board(vec![Player::Empty; 9])
	}
}

#[derive(Debug)]
pub enum PlaceError {
	AlreadyPlaced,
	InvalidPosition,
}

impl ToString for Board {
	fn to_string(&self) -> String {
		let mut result = String::new();

		for (i, v) in self.0.iter().enumerate() {
			result.push(match v {
				Player::X => 'X',
				Player::O => 'O',
				_ => (i + 1).to_string().chars().next().unwrap(),
			});

			if i % 3 == 2 {
				result.push('\n');
			}
		}

		result
	}
}

const TARGET_SCORE: [i32; 2] = [-3, 3];

pub struct EmptyPositionsIter<'a> {
	curr: usize,
	state: &'a Vec<Player>,
}

impl Iterator for EmptyPositionsIter<'_> {
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {
		let len = self.state.len();

		while self.curr < len {
			self.curr += 1;

			if self.state[self.curr - 1] == Player::Empty {
				return Some(self.curr - 1);
			}
		}

		None
	}
}

impl Board {
	pub fn place_at(&mut self, index: usize, player: Player) -> Result<(), PlaceError> {
		let state = &mut self.0;

		if index >= state.len() {
			return Err(PlaceError::InvalidPosition);
		}

		if state[index] != Player::Empty {
			return Err(PlaceError::AlreadyPlaced);
		}

		state[index] = player;
		Ok(())
	}

	pub fn empty_positions(&self) -> EmptyPositionsIter<'_> {
		EmptyPositionsIter {
			curr: 0,
			state: &self.0,
		}
	}

	pub fn score(&self) -> PlayerScore {
		let state = &self.0;
		let mut tr_score = 0;
		let mut tl_score = 0;

		for i in 0..3 {
			let y_initial_score = state[i].score();
			let x_initial_score = state[i * 3].score();

			let mut y_score = y_initial_score as i32;
			let mut x_score = x_initial_score as i32;

			for delta in 1..3 {
				// Check:
				// x . .   . x .   . . x
				// x . .   . x .   . . x
				// x . .   . x .   . . x
				y_score += state[i + delta * 3].score() as i32;

				// Check:
				// x x x   . . .   . . .
				// . . .   x x x   . . .
				// . . .   . . .   x x x
				x_score += state[i * 3 + delta].score() as i32;
			}

			if TARGET_SCORE.contains(&y_score) {
				return y_initial_score;
			}

			if TARGET_SCORE.contains(&x_score) {
				return x_initial_score;
			}

			// Check:
			// x . .
			// . x .
			// . . x
			tr_score += state[i * 4].score() as i32;

			// Check:
			// . . x
			// . x .
			// x . .
			tl_score += state[i * 2 + 2].score() as i32;
		}

		if TARGET_SCORE.contains(&tr_score) {
			return state[0].score();
		}

		if TARGET_SCORE.contains(&tl_score) {
			return state[2].score();
		}

		PlayerScore::Equal
	}

	pub fn finished(&self) -> bool {
		self.score().is_ne() || self.empty_positions().next().is_none()
	}
}
