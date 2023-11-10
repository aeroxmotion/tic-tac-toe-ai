use std::{
	cmp::Ordering,
	io::{self, BufRead, Write},
};

use board::Board;

use crate::{ai::move_ai, player::Player};

mod ai;
mod board;
mod player;

fn main() {
	let mut board = Board::default();

	loop {
		let mut input = String::new();

		println!("--- Game ---\n{}", board.to_string());

		if board.finished() {
			println!(
				"Result: {}",
				match board.score() {
					Ordering::Less => "You won!",
					Ordering::Greater => "You lose!",
					Ordering::Equal => "Draw.",
				}
			);
			break;
		}

		print!("Enter number: ");

		io::stdout().flush().unwrap();
		io::stdin().lock().read_line(&mut input).unwrap();
		let player_action = input.trim().parse::<usize>().unwrap() - 1;
		board.place_at(player_action, Player::O).unwrap();

		if board.finished() {
			println!("--- Game ---\n{}", board.to_string());
			println!(
				"Result: {}",
				match board.score() {
					Ordering::Less => "You won!",
					Ordering::Greater => "You lose!",
					Ordering::Equal => "Draw.",
				}
			);

			break;
		}

		move_ai(&mut board);
	}
}
