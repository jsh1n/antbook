fn main() {
	// argument
	let l: u8 = 10;
	let x: Vec<u8> = vec![2, 6, 7];

	// exec
	println!("(min, max)={:?}", solve(l, x));
}

use std::cmp;

fn solve(l: u8, x: Vec<u8>) -> (u8, u8) {
	//min steps
	let mut min_steps = 0;
	for position in &x {
		min_steps = cmp::max(cmp::min(*position, l - *position), min_steps);
	}

	//max steps
	let mut max_steps = 0;
	for position in &x {
		max_steps = cmp::max(cmp::max(*position, l - *position), max_steps);
	}

	return (min_steps, max_steps);
}

#[test]
fn answer_check() {
	assert_eq!(solve(10, vec![2, 6, 7]), (4, 8));
}
