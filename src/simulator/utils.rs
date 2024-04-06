use crate::{config::INSTRUCTIONS_RADIX, types::error::MyError};
use std::{i16, io};

pub fn read_instruction() -> Result<i16, MyError> {
	let mut data = String::new();
	if io::stdin().read_line(&mut data).is_err() {
		Err(MyError::new("Failed to read line"))
	} else {
		Ok(i16::from_str_radix(data.trim(), INSTRUCTIONS_RADIX).expect("Invalid input"))
	}
}

pub fn read_decimal() -> Result<i16, MyError> {
	let mut data = String::new();
	if io::stdin().read_line(&mut data).is_err() {
		Err(MyError::new("Failed to read line"))
	} else {
		Ok(i16::from_str_radix(data.trim(), 10).expect("Invalid input"))
	}
}

pub fn read_string() -> Result<String, MyError> {
	let mut data = String::new();
	if io::stdin().read_line(&mut data).is_err() {
		Err(MyError::new("Failed to read line"))
	} else {
		Ok(data)
	}
}

pub fn sign(x: i16) -> char {
	if x < 0 {
		'-'
	} else {
		'+'
	}
}
