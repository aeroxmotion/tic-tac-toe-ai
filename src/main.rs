use std::{
	cmp::{self, Ordering},
	io::{self, BufRead, Write},
};

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
enum Player {
	X = 'X' as u8,
	O = 'O' as u8,
	Empty = '.' as u8,
}

fn state_to_string(state: State) -> String {
	let mut result = String::new();

	for (i, v) in state.iter().enumerate() {
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

type State = Vec<Player>;
type Action = usize;

fn actions(state: State) -> Vec<Action> {
	let mut actions = vec![];

	for (i, cell) in state.into_iter().enumerate() {
		if cell == Player::Empty {
			actions.push(i)
		}
	}

	actions
}

fn move_ai(state: State) -> State {
	let mut max_score = Ordering::Less;
	let mut target_action = None;

	for action in actions(state.clone()) {
		let next_state = result(state.clone(), action, Player::X);
		let score = min(next_state);

		if score > max_score {
			max_score = score;
			target_action = Some(action);
		}
	}

	let mut final_state = state.clone();
	final_state[target_action.unwrap()] = Player::X;
	final_state
}

fn max(state: State) -> Ordering {
	if terminal(state.clone()) {
		return utility(state.clone());
	}

	let mut max_score = Ordering::Less;

	for action in actions(state.clone()) {
		let next_state = result(state.clone(), action, Player::X);
		max_score = cmp::max(max_score, min(next_state));
	}

	max_score
}

fn min(state: State) -> Ordering {
	if terminal(state.clone()) {
		return utility(state.clone());
	}

	let mut min_score = Ordering::Greater;

	for action in actions(state.clone()) {
		let next_state = result(state.clone(), action, Player::O);
		min_score = cmp::min(min_score, max(next_state));
	}

	min_score
}

fn terminal(state: State) -> bool {
	utility(state.clone()).is_ne() || actions(state).len() == 0
}

fn result(state: State, action: Action, player: Player) -> State {
	let mut next_state = state.clone();
	next_state[action] = player;
	next_state
}

fn utility(state: State) -> Ordering {
	for action in 0..3 {
		match y_score(state.clone(), action) {
			Ordering::Equal => {}
			score => return score,
		}
	}

	for action in [0, 3, 6] {
		match x_score(state.clone(), action) {
			Ordering::Equal => {}
			score => return score,
		}
	}

	d_score(state)
}

fn d_score(state: State) -> Ordering {
	// Check:
	// x . .
	// . x .
	// . . x
	for i in [4, 8] {
		if state[0] != state[i] {
			// Check:
			// . . x
			// . x .
			// x . .
			for i in [4, 6] {
				if state[2] != state[i] {
					return Ordering::Equal;
				}
			}

			return player_score(state[2]);
		}
	}

	player_score(state[0])
}

fn x_score(state: State, action: Action) -> Ordering {
	// Check:
	// x x x   . . .   . . .
	// . . .   x x x   . . .
	// . . .   . . .   x x x
	for delta in 1..3 {
		if state[action] != state[action + delta] {
			return Ordering::Equal;
		}
	}

	player_score(state[action])
}

fn y_score(state: State, action: Action) -> Ordering {
	// Check:
	// x . .   . x .   . . x
	// x . .   . x .   . . x
	// x . .   . x .   . . x
	for delta in 1..3 {
		if state[action] != state[action + delta * 3] {
			return Ordering::Equal;
		}
	}

	player_score(state[action])
}

fn player_score(player: Player) -> Ordering {
	match player {
		Player::X => Ordering::Greater,
		Player::O => Ordering::Less,
		Player::Empty => Ordering::Equal,
	}
}

fn main() {
	let mut state: State = vec![Player::Empty; 9];

	loop {
		let mut input = String::new();

		println!("--- Game ---\n{}", state_to_string(state.clone()));

		if terminal(state.clone()) {
			println!(
				"Result: {}",
				match utility(state.clone()) {
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
		let next_state = result(state.clone(), player_action, Player::O);

		if terminal(next_state.clone()) {
			println!("--- Game ---\n{}", state_to_string(next_state.clone()));
			println!(
				"Result: {}",
				match utility(next_state.clone()) {
					Ordering::Less => "You won!",
					Ordering::Greater => "You lose!",
					Ordering::Equal => "Draw.",
				}
			);
			break;
		}

		state = move_ai(next_state);
	}
}
