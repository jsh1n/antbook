use std::io;

struct Wallet {
	coins: Vec<(u32, u32)>,
}

impl Wallet {
	fn new(vec: Vec<u32>) -> Wallet {
		if vec.len() != 6 {
			panic!("WalletCreateErr");
		}
		let values = vec![1, 5, 10, 50, 100, 500];
		let mut coins = vec![];
		for i in 0..6 {
			coins.push((values[i], vec[i]));
		}
		Wallet { coins: coins }
	}
	fn is_payable(&self, value: u32) -> bool {
		let mut sum: u32 = 0;
		for i in 0..6 {
			sum += self.coins[i].0 * self.coins[i].1;
		}
		return sum >= value;
	}
	fn payment(&mut self, value: u32) -> bool {
		if !self.is_payable(value) {
			return false;
		}
		let mut remain: u32 = value;
		for i in 0..6 {
			let n = (remain - remain % self.coins[5 - i].0) / self.coins[5 - i].0;
			let n = std::cmp::min(n, self.coins[5 - i].1);
			remain = remain - n * self.coins[5 - i].0;
			self.coins[5 - i].1 -= n;
			if remain == 0 {
				return true;
			}
		}
		return false;
	}
	fn get_balance(&self) {
		let mut s = String::new();
		for coin in &self.coins {
			s = format!("({}:{}),{}", coin.0, coin.1, s);
		}
		println!("Wallet:{}", s);
	}
}

fn main() {
	println!("This is implementation of greedy method.");

	let mut wallet: Wallet = Wallet::new(vec![120, 302, 231, 111, 341, 121]);
	println!("you have ,");
	wallet.get_balance();
	loop {
		let mut payment = String::new();

		println!("Please input the amount of payment");
		io::stdin()
			.read_line(&mut payment)
			.expect("Fail to read line.");

		let payment: u32 = match payment.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		if !wallet.payment(payment) {
			println!("you dont have enough money");
		};
		wallet.get_balance();
	}
}
