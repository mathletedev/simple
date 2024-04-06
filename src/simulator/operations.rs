use super::{
	simpletron::{Simpletron, State},
	utils::{read_decimal, read_string},
};
use crate::types::error::MyError;
use lazy_static::lazy_static;
use std::collections::HashMap;

type Operation = fn(&mut Simpletron) -> Result<(), MyError>;

const READ: Operation = |simpletron| {
	let data = match read_decimal() {
		Ok(data) => data,
		Err(_) => return Err(MyError::new("Invalid token")),
	};

	simpletron.memory[simpletron.operand as usize] = data;

	Ok(())
};

const WRITE: Operation = |simpletron| {
	// TODO: output to disk
	println!("{}", simpletron.memory[simpletron.operand as usize]);

	Ok(())
};

const READ_STR: Operation = |simpletron| {
	let data = match read_string() {
		Ok(data) => data,
		Err(_) => return Err(MyError::new("Invalid token")),
	};

	// first address = length of string
	let ptr = simpletron.operand as usize;
	simpletron.memory[ptr] = data.len() as i16;

	// assign each character to memory
	for (i, char) in data.chars().enumerate() {
		simpletron.memory[ptr + i + 1] = char as i16;
	}

	Ok(())
};

const WRITE_STR: Operation = |simpletron| {
	let ptr = simpletron.operand as usize;
	let length = simpletron.memory[ptr] as usize;

	// write each character individually
	for i in 1..=length {
		print!("{}", simpletron.memory[ptr + i] as u8 as char);
	}

	Ok(())
};

const LOAD: Operation = |simpletron| {
	simpletron.accumulator = simpletron.memory[simpletron.operand as usize];

	Ok(())
};

const STORE: Operation = |simpletron| {
	simpletron.memory[simpletron.operand as usize] = simpletron.accumulator;

	Ok(())
};

const ADD: Operation = |simpletron| {
	simpletron.accumulator += simpletron.memory[simpletron.operand as usize];

	Ok(())
};

const SUBTRACT: Operation = |simpletron| {
	simpletron.accumulator -= simpletron.memory[simpletron.operand as usize];

	Ok(())
};

const DIVIDE: Operation = |simpletron| {
	if simpletron.memory[simpletron.operand as usize] == 0 {
		return Err(MyError::new("Attempt to divide by zero"));
	}

	simpletron.accumulator /= simpletron.memory[simpletron.operand as usize];

	Ok(())
};

const MULTIPLY: Operation = |simpletron| {
	simpletron.accumulator *= simpletron.memory[simpletron.operand as usize];

	Ok(())
};

const MODULUS: Operation = |simpletron| {
	if simpletron.memory[simpletron.operand as usize] == 0 {
		return Err(MyError::new("Attempt to modulo by zero"));
	}

	simpletron.accumulator %= simpletron.memory[simpletron.operand as usize];

	Ok(())
};

const EXPONENTIATE: Operation = |simpletron| {
	let base = simpletron.accumulator;
	simpletron.accumulator = 1;

	for _ in 0..simpletron.memory[simpletron.operand as usize] {
		simpletron.accumulator *= base;
	}

	Ok(())
};

const BRANCH: Operation = |simpletron| {
	// go to one instruction before because it will be incremented
	simpletron.instruction_counter = simpletron.operand as u16 - 1;

	Ok(())
};

const BRANCH_NEG: Operation = |simpletron| {
	if simpletron.accumulator < 0 {
		simpletron.instruction_counter = simpletron.operand as u16 - 1;
	}

	Ok(())
};

const BRANCH_ZERO: Operation = |simpletron| {
	if simpletron.accumulator == 0 {
		simpletron.instruction_counter = simpletron.operand as u16 - 1;
	}

	Ok(())
};

const HALT: Operation = |simpletron| {
	println!();
	println!("*** Simpletron execution terminated ***");
	simpletron.state = State::Halted;

	Ok(())
};

const SML_DEBUG: Operation = |simpletron| {
	simpletron.debug = !(simpletron.operand == 0);

	Ok(())
};

// collect all operations into a single table for easy lookup
lazy_static! {
	pub static ref OPERATION_TABLE: HashMap<u8, Operation> = HashMap::from([
		(0x10, READ),
		(0x11, WRITE),
		(0x12, READ_STR),
		(0x13, WRITE_STR),
		(0x20, LOAD),
		(0x21, STORE),
		(0x30, ADD),
		(0x31, SUBTRACT),
		(0x32, DIVIDE),
		(0x33, MULTIPLY),
		(0x34, MODULUS),
		(0x35, EXPONENTIATE),
		(0x40, BRANCH),
		(0x41, BRANCH_NEG),
		(0x42, BRANCH_ZERO),
		(0x43, HALT),
		(0x44, SML_DEBUG),
	]);
}
