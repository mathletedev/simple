// https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html
#[derive(Debug)]
pub struct MyError {
	pub details: String,
}

impl MyError {
	pub fn new(msg: &str) -> MyError {
		MyError {
			details: msg.to_string(),
		}
	}
}
