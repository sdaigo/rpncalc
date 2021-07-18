fn get_int_from_file() -> Result<i32, String> {
	let path = "number.txt";

	let num_str = std::fs::read_to_string(path)
		// read_to_stringの返り値のResultがErrの場合
		// Errを返して早期リターンする
		.map_err(|e| e.to_string())?;

	num_str
		.trim()
		.parse::<i32>()
		.map(|t| t * 2) // parseの結果が Ok
		.map_err(|e| e.to_string()) // parseの結果が Err
}

fn main() {
	match get_int_from_file() {
		Ok(x) => println!("{}", x),
		Err(e) => println!("{}", e),
	}
}
