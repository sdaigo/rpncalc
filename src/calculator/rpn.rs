use anyhow::{bail, ensure, Context, Result};

pub struct RpnCalculator(bool);

impl RpnCalculator {
	pub fn new(verbose: bool) -> Self {
		Self(verbose)
	}

	fn calc(&self, left: i32, right: i32, token: &str) -> Result<i32> {
		match token {
			"+" => Ok(left + right),
			"-" => Ok(left - right),
			"*" => Ok(left * right),
			"/" => Ok(left / right),
			"%" => Ok(left % right),
			_ => bail!("invalid token: \"{}\"", token),
		}
	}

	fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
		let mut stack = vec![];
		let mut pos = 0;

		while let Some(token) = tokens.pop() {
			pos += 1;
			// i32にparseできれば被演算子
			if let Ok(x) = token.parse::<i32>() {
				stack.push(x);
			} else {
				// token = 演算子なので、stackの1番目, 2番目を取り出して計算する
				let y = stack.pop().context(format!("invalid syntax at {}", pos))?;
				let x = stack.pop().context(format!("invalid syntax at {}", pos))?;

				let res = self.calc(x, y, token)?;

				// 計算結果をスタックに置く
				stack.push(res)
			}

			if self.0 {
				println!("{:?} {:?}", tokens, stack);
			}
		}

		ensure!(stack.len() == 1, "invalid syntax");

		Ok(stack[0])
	}

	pub fn eval(&self, formula: &str) -> Result<i32> {
		let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
		self.eval_inner(&mut tokens)
	}
}
