use crate::types::error::MyError;
use std::io;

pub fn read_word() -> Result<i16, MyError> {
	let mut data = String::new();
	if io::stdin().read_line(&mut data).is_err() {
		Err(MyError::new("Failed to read word"))
	} else {
		Ok(data.trim().parse().expect("Invalid input"))
	}
}

pub fn sign(x: i16) -> char {
	if x < 0 {
		'-'
	} else {
		'+'
	}
}
