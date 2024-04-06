use super::{operations::OPERATION_TABLE, utils::sign};
use crate::{runner::utils::read_word, types::error::MyError};
use std::{
	fs::File,
	io::{self, prelude::*, BufReader, Error},
};

#[derive(PartialEq, Eq)]
pub enum State {
	RUNNING,
	HALTED,
	CRASHED,
}

pub struct Simpletron {
	pub state: State,
	pub accumulator: i16,
	pub instruction_counter: u8,
	instruction_register: i16,
	operation_code: u8,
	pub operand: u8,
	pub memory: Vec<i16>,
}

impl Simpletron {
	pub fn new() -> Simpletron {
		println!("*** Welcome to Simpletron! ***");
		println!();

		Simpletron {
			state: State::HALTED,
			accumulator: 0,
			instruction_counter: 0,
			instruction_register: 0,
			operation_code: 0,
			operand: 0,
			memory: vec![0; 100],
		}
	}

	// load program from file
	pub fn load(&mut self, file_path: String) -> Result<(), Error> {
		let file = File::open(file_path)?;

		let reader = BufReader::new(file);

		for (i, line) in reader.lines().enumerate() {
			self.memory[i] = line
				.unwrap()
				.trim()
				.parse()
				.expect(format!("Invalid token on line {}", i + 1).as_str())
		}

		println!("*** Program loading completed ***");
		println!();

		Ok(())
	}

	// load program from command-line input
	pub fn input(&mut self) {
		println!("*** Please enter your program one instruction ***");
		println!("*** (or data word) at a time. I will type the ***");
		println!("*** location number and a question mark (?). ***");
		println!("*** You then type the word for that location. ***");
		println!("*** Type the sentinel -10000 to stop entering ***");
		println!("*** your program. ***");
		println!();

		for i in 0..self.memory.len() {
			print!("{i:0>2} ? ");
			io::stdout().flush().unwrap();

			// TODO: input error handling
			let data = read_word().unwrap();

			if data == -10000 {
				break;
			}

			self.memory[i] = data;
		}

		println!();
		println!("*** Program loading completed ***");
		println!();
	}

	pub fn execute(&mut self) -> Result<(), MyError> {
		println!("*** Program execution begins ***");
		println!();

		self.state = State::RUNNING;

		while self.state == State::RUNNING {
			self.step();
		}

		if self.state == State::CRASHED {
			Err(MyError::new(""))
		} else {
			Ok(())
		}
	}

	// executes current instruction
	fn step(&mut self) {
		self.instruction_register = self.memory[self.instruction_counter as usize];

		self.operation_code = (self.instruction_register / 100) as u8;
		self.operand = (self.instruction_register % 100) as u8;

		// find operation in operation table
		let operation = OPERATION_TABLE.get(&self.operation_code).map(|x| *x);

		// check if operation exists
		match operation {
			// call operation on self
			Some(operation) => operation(self),
			None => {
				println!("Invalid operation");
				self.state = State::HALTED;
			}
		}

		// move to next instruction
		self.instruction_counter += 1;
	}

	pub fn dump(&self) {
		println!("REGISTERS:");
		println!(
			"accumulator\t{}{x:0>4}",
			sign(self.accumulator),
			x = self.accumulator
		)
	}
}
