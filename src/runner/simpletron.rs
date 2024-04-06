use super::{
	operations::OPERATION_TABLE,
	utils::{read_instruction, sign},
};
use crate::{config::INSTRUCTIONS_RADIX, types::error::MyError};
use std::{
	fs::File,
	i16,
	io::{self, prelude::*, BufReader},
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
	pub instruction_counter: u16,
	instruction_register: i16,
	operation_code: u8,
	pub operand: u8,
	pub memory: Vec<i16>,
	pub debug: bool,
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
			memory: vec![0; 1000],
			debug: false,
		}
	}

	// load program from file
	pub fn load(&mut self, file_path: String) -> Result<(), MyError> {
		let file = match File::open(&file_path) {
			Ok(file) => file,
			Err(_) => {
				println!("*** Failed to open file {} ***", &file_path);
				return Err(MyError::new("Failed to open file"));
			}
		};

		let reader = BufReader::new(file);

		for (i, line) in reader.lines().enumerate() {
			self.memory[i] = i16::from_str_radix(line.unwrap().trim(), INSTRUCTIONS_RADIX)
				.expect(format!("Invalid token on line {}", i + 1).as_str())
		}

		println!("*** Program loading completed ***");
		println!();

		Ok(())
	}

	// load program from command-line input
	pub fn input(&mut self) -> Result<(), MyError> {
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

			let data = match read_instruction() {
				Ok(data) => data,
				Err(_) => {
					println!("*** Invalid token ***");
					return Err(MyError::new("Invalid token"));
				}
			};

			if data == -10000 {
				break;
			}

			self.memory[i] = data;
		}

		println!();
		println!("*** Program loading completed ***");
		println!();

		Ok(())
	}

	pub fn execute(&mut self) {
		println!("*** Program execution begins ***");
		println!();

		self.state = State::RUNNING;

		while self.state == State::RUNNING {
			match self.step() {
				Ok(()) => {}
				Err(error) => {
					println!();
					println!("*** {} ***", error.details);
					println!("*** Simpletron execution abnormally terminated ***");
				}
			}
		}
	}

	// executes current instruction
	fn step(&mut self) -> Result<(), MyError> {
		self.instruction_register = self.memory[self.instruction_counter as usize];

		// INSTRUCTIONS_RADIX^2 is the 3rd position
		let separator = INSTRUCTIONS_RADIX * INSTRUCTIONS_RADIX;
		self.operation_code = (self.instruction_register / separator as i16) as u8;
		self.operand = (self.instruction_register % separator as i16) as u8;

		// find operation in operation table
		let operation = OPERATION_TABLE.get(&self.operation_code).map(|x| *x);

		// check if operation exists
		match operation {
			// call operation on self
			Some(operation) => match operation(self) {
				Ok(()) => {}
				// error handling
				Err(error) => {
					self.state = State::CRASHED;
					return Err(error);
				}
			},
			None => {
				println!("Invalid operation");
				self.state = State::HALTED;
			}
		}

		// move to next instruction
		self.instruction_counter += 1;

		if self.debug {
			self.dump();
		}

		Ok(())
	}

	pub fn dump(&self) {
		println!("REGISTERS:");
		println!(
			"accumulator\t\t{}{x:0>4x}",
			sign(self.accumulator),
			x = self.accumulator
		);
		println!(
			"instruction_counter\t   {x:0>2x}",
			x = self.instruction_counter
		);
		println!(
			"instruction_register\t{}{x:0>4x}",
			sign(self.instruction_register),
			x = self.instruction_register
		);
		println!(
			"operation_code\t\t   {x:0>2x}",
			x = self.instruction_counter
		);
		println!("operand\t\t\t   {x:0>2x}", x = self.instruction_counter);

		println!();
		println!("MEMORY");
		print!("  ");
		for i in 0..10 {
			print!("     {i}");
		}
		println!();
		for i in 0..10 {
			print!("{i}0");

			for j in 0..10 {
				let data = self.memory[i * 10 + j];
				print!(" {}{x:0>4x}", sign(data), x = data)
			}
			println!();
		}
	}
}
