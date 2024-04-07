use super::table_entry::TableEntryType;
use crate::types::MyError;

pub fn to_symbol(token: String) -> Result<(i32, TableEntryType), MyError> {
	// check if variable
	if token.len() != 1 || !token.chars().nth(0).unwrap().is_alphabetic() {
		// check if constant
		match token.parse::<i32>() {
			Ok(number) => Ok((number, TableEntryType::Constant)),
			Err(_) => Err(MyError::new("Invalid symbol")),
		}
	} else {
		Ok((
			token.chars().nth(0).unwrap() as i32,
			TableEntryType::Variable,
		))
	}
}
