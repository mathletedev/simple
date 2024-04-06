use super::{
	simpletron::{Simpletron, State},
	utils::read_word,
};
use lazy_static::lazy_static;
use std::collections::HashMap;

type Operation = fn(&mut Simpletron);

const READ: Operation = |simpletron: &mut Simpletron| {
	let data = read_word();

	simpletron.memory[simpletron.operand as usize] = data.unwrap();
};

const WRITE: Operation = |simpletron: &mut Simpletron| {
	println!("{}", simpletron.memory[simpletron.operand as usize]);
};

const LOAD: Operation = |simpletron: &mut Simpletron| {
	simpletron.accumulator = simpletron.memory[simpletron.operand as usize];
};

const STORE: Operation = |simpletron: &mut Simpletron| {
	simpletron.memory[simpletron.operand as usize] = simpletron.accumulator;
};

const ADD: Operation = |simpletron: &mut Simpletron| {
	simpletron.accumulator += simpletron.memory[simpletron.operand as usize];
};

const SUBTRACT: Operation = |simpletron: &mut Simpletron| {
	simpletron.accumulator -= simpletron.memory[simpletron.operand as usize];
};

const DIVIDE: Operation = |simpletron: &mut Simpletron| {
	simpletron.accumulator /= simpletron.memory[simpletron.operand as usize];
};

const MULTIPLY: Operation = |simpletron: &mut Simpletron| {
	simpletron.accumulator *= simpletron.memory[simpletron.operand as usize];
};

const BRANCH: Operation = |simpletron: &mut Simpletron| {
	// go to one instruction before because it will be incremented
	simpletron.instruction_counter = simpletron.operand - 1;
};

const BRANCH_NEG: Operation = |simpletron: &mut Simpletron| {
	if simpletron.accumulator < 0 {
		simpletron.instruction_counter = simpletron.operand - 1;
	}
};

const BRANCH_ZERO: Operation = |simpletron: &mut Simpletron| {
	if simpletron.accumulator == 0 {
		simpletron.instruction_counter = simpletron.operand - 1;
	}
};

const HALT: Operation = |simpletron: &mut Simpletron| {
	println!();
	println!("*** Simpletron execution terminated ***");
	simpletron.state = State::HALTED;
};

// collect all operations into a single table for easy lookup
lazy_static! {
	pub static ref OPERATION_TABLE: HashMap<u8, Operation> = HashMap::from([
		(10, READ),
		(11, WRITE),
		(20, LOAD),
		(21, STORE),
		(30, ADD),
		(31, SUBTRACT),
		(32, DIVIDE),
		(33, MULTIPLY),
		(40, BRANCH),
		(41, BRANCH_NEG),
		(42, BRANCH_ZERO),
		(43, HALT)
	]);
}
