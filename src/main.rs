pub mod config;
pub mod runner;
pub mod types;

use runner::simpletron::Simpletron;

// TODO: implement floating-point numbers

fn main() {
	let mut simpletron = Simpletron::new();

	if simpletron.load("examples/echo.msl".to_string()).is_err() {
		return;
	}

	// simpletron.input();

	simpletron.execute();
}
