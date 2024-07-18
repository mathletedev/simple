use super::{Simulator, State};
use crate::types::MyError;
use lazy_static::lazy_static;
use std::collections::HashMap;

type Operation = fn(&mut Simulator) -> Result<(), MyError>;

const READ: Operation = |simulator| {
    let data = match simulator.read_decimal() {
        Ok(data) => data,
        Err(_) => return Err(MyError::new("Invalid token")),
    };

    simulator.memory[simulator.operand as usize] = data;

    Ok(())
};

const WRITE: Operation = |simulator| {
    // TODO: output to disk
    println!("{}", simulator.memory[simulator.operand as usize]);

    Ok(())
};

const READ_STR: Operation = |simulator| {
    let data = match simulator.read_string() {
        Ok(data) => data,
        Err(_) => return Err(MyError::new("Invalid token")),
    };

    // first address = length of string
    let ptr = simulator.operand as usize;
    simulator.memory[ptr] = data.len() as i32;

    // assign each character to memory
    for (i, char) in data.chars().enumerate() {
        simulator.memory[ptr + i + 1] = char as i32;
    }

    Ok(())
};

const WRITE_STR: Operation = |simulator| {
    let ptr = simulator.operand as usize;
    let length = simulator.memory[ptr] as usize;

    // write each character individually
    for i in 1..=length {
        print!("{}", simulator.memory[ptr + i] as u8 as char);
    }

    Ok(())
};

const LOAD: Operation = |simulator| {
    simulator.accumulator = simulator.memory[simulator.operand as usize];

    Ok(())
};

const STORE: Operation = |simulator| {
    simulator.memory[simulator.operand as usize] = simulator.accumulator;

    Ok(())
};

const ADD: Operation = |simulator| {
    simulator.accumulator += simulator.memory[simulator.operand as usize];

    Ok(())
};

const SUBTRACT: Operation = |simulator| {
    simulator.accumulator -= simulator.memory[simulator.operand as usize];

    Ok(())
};

const DIVIDE: Operation = |simulator| {
    if simulator.memory[simulator.operand as usize] == 0 {
        return Err(MyError::new("Attempt to divide by zero"));
    }

    simulator.accumulator /= simulator.memory[simulator.operand as usize];

    Ok(())
};

const MULTIPLY: Operation = |simulator| {
    simulator.accumulator *= simulator.memory[simulator.operand as usize];

    Ok(())
};

const MODULUS: Operation = |simulator| {
    if simulator.memory[simulator.operand as usize] == 0 {
        return Err(MyError::new("Attempt to modulo by zero"));
    }

    simulator.accumulator %= simulator.memory[simulator.operand as usize];

    Ok(())
};

const EXPONENTIATE: Operation = |simulator| {
    let base = simulator.accumulator;
    simulator.accumulator = 1;

    for _ in 0..simulator.memory[simulator.operand as usize] {
        simulator.accumulator *= base;
    }

    Ok(())
};

const BRANCH: Operation = |simulator| {
    // go to one instruction before because it will be incremented
    simulator.set_instruction_counter(simulator.operand - 1);

    Ok(())
};

const BRANCH_NEG: Operation = |simulator| {
    if simulator.accumulator < 0 {
        simulator.set_instruction_counter(simulator.operand - 1);
    }

    Ok(())
};

const BRANCH_ZERO: Operation = |simulator| {
    if simulator.accumulator == 0 {
        simulator.set_instruction_counter(simulator.operand - 1);
    }

    Ok(())
};

const HALT: Operation = |simulator| {
    println!();
    println!("*** Simpletron execution terminated ***");

    simulator.set_state(State::Halted);

    Ok(())
};

const SML_DEBUG: Operation = |simulator| {
    simulator.set_debug(simulator.operand != 0);

    Ok(())
};

// collect all operations into a single table for easy lookup
lazy_static! {
    pub static ref OPERATION_TABLE: HashMap<u32, Operation> = HashMap::from([
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
