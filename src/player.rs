use std::cmp::Ordering;

pub type PlayerScore = Ordering;

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Player {
	X = 'X' as u8,
	O = 'O' as u8,
	Empty = '.' as u8,
}

impl Player {
	pub fn score(&self) -> PlayerScore {
		match self {
			Player::X => PlayerScore::Greater,
			Player::O => PlayerScore::Less,
			Player::Empty => PlayerScore::Equal,
		}
	}
}
