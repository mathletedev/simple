pub mod runner;
pub mod types;

use runner::simpletron::Simpletron;

fn main() {
	let mut simpletron = Simpletron::new();

	match simpletron.load("examples/add.msl".to_string()) {
		Ok(()) => {}
		Err(_) => {
			println!("*** Failed to open file ***");
			return;
		}
	}

	// simpletron.input();

	match simpletron.execute() {
		Ok(()) => {}
		Err(error) => {
			println!("*** {} ***", error.details);
			println!("*** Simpletron execution abnormally terminated ***");
		}
	}
}
