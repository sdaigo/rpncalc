enum MyError {
	Io(std::io::Error),
	Num(std::num::ParseIntError),
}

fn get_int_from_file() -> Result<i32, MyError> {
	let path = "number.txt";

	let num_str = std::fs::read_to_string(path)
		// read_to_stringの返り値のResultがErrの場合
		// Errを返して早期リターンする
		.map_err(|e| MyError::Io(e))?;

	num_str
		.trim()
		.parse::<i32>()
		.map(|t| t * 2) // parseの結果が Ok
		.map_err(|e| MyError::Num(e)) // parseの結果が Err
}

fn main() {
	match get_int_from_file() {
		Ok(x) => println!("{}", x),
		Err(e) => match e {
			MyError::Io(cause) => println!("I/O Error: {}", cause),
			MyError::Num(cause) => println!("Parse Error: {}", cause),
		},
	}
}
