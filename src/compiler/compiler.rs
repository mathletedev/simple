use super::{
	commands::COMMAND_TABLE,
	symbol_table::SymbolTable,
	table_entry::{TableEntry, TableEntryType},
};
use crate::{
	config::{INSTRUCTIONS_SEP, MEMORY},
	types::MyError,
};
use std::{
	fs::{File, OpenOptions},
	io::{prelude::*, BufReader, BufWriter},
	path::PathBuf,
};

pub struct Compiler {
	pub instruction_counter: u32,
	pub data_counter: u32,
	instructions: Vec<i32>,
	symbol_table: SymbolTable,
	flags: Vec<i32>,
}

impl Compiler {
	pub fn new() -> Compiler {
		Compiler {
			instruction_counter: 0,
			data_counter: MEMORY - 1,
			instructions: vec![0; MEMORY as usize],
			symbol_table: SymbolTable::new(),
			flags: vec![-1; MEMORY as usize],
		}
	}

	pub fn compile(&mut self, in_path: PathBuf, out_path: PathBuf) {
		let file = match File::open(&in_path) {
			Ok(file) => file,
			Err(_) => {
				println!(
					"*** Failed to open file {} for reading ***",
					&in_path.to_string_lossy()
				);
				return;
			}
		};

		let reader = BufReader::new(file);

		// first pass
		for (i, line) in reader.lines().enumerate() {
			let tokens: Vec<String> = line.unwrap().split(" ").map(|x| x.to_string()).collect();

			if tokens.len() < 2 {
				println!("*** Syntax error on line {i}: Incomplete statement ***");
				return;
			}

			// get line number
			let line_number: u32 = match tokens[0].parse() {
				Ok(line_number) => line_number,
				Err(_) => {
					println!("*** Syntax error on line {i}: Invalid line number ***");
					return;
				}
			};

			if self
				.symbol_table
				.find(line_number as i32, TableEntryType::LineNumber)
				.is_some()
			{
				println!("*** Syntax error on line {i}: Line number already used ***");
				return;
			}

			// insert line number into symbol table
			self.symbol_table.insert(TableEntry {
				symbol: line_number as i32,
				entry_type: TableEntryType::LineNumber,
				location: self.instruction_counter,
			});

			// find command in uppercase
			let command = COMMAND_TABLE.get(&tokens[1].to_uppercase()).map(|x| *x);

			match command {
				// call the command
				Some(command) => match command(self, &tokens[2..]) {
					Ok(()) => {}
					Err(error) => {
						println!("*** Syntax error on line {i}: {} ***", error.details);
						return;
					}
				},
				None => {
					println!(
						"*** Syntax error on line {i}: Invalid symbol {} ***",
						tokens[1]
					);
					return;
				}
			}

			// program ran out of memory
			if self.data_counter <= self.instruction_counter {
				println!("*** Memory limit exceeded ***");
				return;
			}
		}

		// second pass
		for (i, x) in self.flags.iter().enumerate() {
			if *x == -1 {
				continue;
			}

			let table_entry = match self.find_line_number(*x) {
				Some(table_entry) => table_entry,
				None => {
					println!("*** GOTO failed: Line number {x} does not exist ***");
					return;
				}
			};

			// add location to BRANCH operation
			self.instructions[i] += table_entry.location as i32;
		}

		match self.write(out_path) {
			Ok(()) => {}
			Err(error) => {
				println!("*** {} ***", error.details);
			}
		}

		println!("*** Compilation finished successfully! ***");
	}

	pub fn add_instruction(&mut self, operation_code: u32, operand: u32) {
		self.instructions[self.instruction_counter as usize] =
			(operation_code * INSTRUCTIONS_SEP + operand) as i32;
		self.instruction_counter += 1;
	}

	pub fn add_flag(&mut self, symbol: i32) {
		// assume add_flag() is called after add_instruction(), so subtract 1 from index
		self.flags[self.instruction_counter as usize - 1] = symbol;
	}

	pub fn find_line_number(&self, symbol: i32) -> Option<TableEntry> {
		self.symbol_table.find(symbol, TableEntryType::LineNumber)
	}

	pub fn find_or_create_symbol(&mut self, symbol: i32, entry_type: TableEntryType) -> TableEntry {
		match self.symbol_table.find(symbol, entry_type) {
			Some(table_entry) => table_entry,
			None => {
				let new_table_entry = TableEntry {
					symbol,
					entry_type,
					location: self.data_counter,
				};

				self.symbol_table.insert(new_table_entry.clone());

				// directly set constants
				if entry_type == TableEntryType::Constant {
					self.instructions[self.data_counter as usize] = symbol;
				}

				self.data_counter -= 1;

				new_table_entry
			}
		}
	}

	// write finished instructions to output file
	fn write(&self, out_path: PathBuf) -> Result<(), MyError> {
		let file = match OpenOptions::new()
			.write(true)
			.create(true)
			.open(out_path.clone())
		{
			Ok(file) => file,
			Err(_) => {
				return Err(MyError::new(
					format!(
						"Failed to open file {} for writing",
						&out_path.to_string_lossy()
					)
					.as_str(),
				));
			}
		};

		let mut writer = BufWriter::new(file);

		match writer.write_all(
			self.instructions
				.iter()
				.map(|x| format!("{x:x}"))
				.collect::<Vec<String>>()
				.join("\n")
				.as_bytes(),
		) {
			Ok(()) => Ok(()),
			Err(_) => {
				return Err(MyError::new("Failed to write to file"));
			}
		}
	}
}
