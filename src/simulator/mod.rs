mod operations;

use crate::config::{INSTRUCTIONS_RADIX, INSTRUCTIONS_SEP, MEMORY};
use anyhow::{bail, Result};
use operations::OPERATION_TABLE;
use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
    path::PathBuf,
};

#[derive(PartialEq, Eq)]
pub enum State {
    Running,
    Halted,
    Crashed,
}

pub struct Simulator {
    state: State,
    pub(super) accumulator: i32,
    instruction_counter: u32,
    instruction_register: i32,
    operation_code: u32,
    pub(super) operand: u32,
    pub(super) memory: Vec<i32>,
    debug: bool,
}

impl Simulator {
    pub fn new() -> Simulator {
        println!("*** Welcome to Simpletron! ***");
        println!();

        Simulator {
            state: State::Halted,
            accumulator: 0,
            instruction_counter: 0,
            instruction_register: 0,
            operation_code: 0,
            operand: 0,
            memory: vec![0; MEMORY as usize],
            debug: false,
        }
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    pub fn set_instruction_counter(&mut self, instruction_counter: u32) {
        self.instruction_counter = instruction_counter;
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    // load program from file
    pub fn load(&mut self, path: PathBuf) -> Result<()> {
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => {
                println!("*** Failed to open file {} ***", &path.to_string_lossy());
                bail!("Failed to open file");
            }
        };

        let reader = BufReader::new(file);

        for (i, line) in reader.lines().enumerate() {
            self.memory[i] = i32::from_str_radix(line.unwrap().trim(), INSTRUCTIONS_RADIX)
                .unwrap_or_else(|_| panic!("Invalid token on line {}", i + 1))
        }

        println!("*** Program loading completed ***");
        println!();

        Ok(())
    }

    // load program from command-line input
    pub fn input(&mut self) -> Result<()> {
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

            let data = match self.read_instruction() {
                Ok(data) => data,
                Err(_) => {
                    println!("*** Invalid token ***");
                    bail!("Invalid token");
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

    pub fn simulate(&mut self) {
        println!("*** Program execution begins ***");
        println!();

        self.state = State::Running;

        while self.state == State::Running {
            match self.step() {
                Ok(()) => {}
                Err(error) => {
                    println!();
                    println!("*** {} ***", error);
                    println!("*** Simpletron execution abnormally terminated ***");
                }
            }
        }
    }

    // executes current instruction
    fn step(&mut self) -> Result<()> {
        self.instruction_register = self.memory[self.instruction_counter as usize];

        self.operation_code = (self.instruction_register / INSTRUCTIONS_SEP as i32) as u32;
        self.operand = (self.instruction_register % INSTRUCTIONS_SEP as i32) as u32;

        // find operation in operation table
        let operation = OPERATION_TABLE.get(&self.operation_code).copied();

        // check if operation exists
        match operation {
            // call operation on self
            Some(operation) => match operation(self) {
                Ok(()) => {}
                // error handling
                Err(error) => {
                    self.state = State::Crashed;
                    return Err(error);
                }
            },
            None => {
                println!("Invalid operation {:x}", self.operation_code);
                self.state = State::Halted;
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
            "accumulator\t\t{}{:0>4x}",
            self.sign(self.accumulator),
            self.accumulator
        );
        println!("instruction_counter\t   {:0>2x}", self.instruction_counter);
        println!(
            "instruction_register\t{}{:0>4x}",
            self.sign(self.instruction_register),
            self.instruction_register
        );
        println!("operation_code\t\t   {:0>2x}", self.instruction_counter);
        println!("operand\t\t\t   {:0>2x}", self.instruction_counter);

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
                print!(" {}{:0>4x}", self.sign(data), data)
            }
            println!();
        }
    }

    pub fn read_instruction(&self) -> Result<i32> {
        let mut data = String::new();
        if io::stdin().read_line(&mut data).is_err() {
            bail!("Failed to read line")
        }

        Ok(i32::from_str_radix(data.trim(), INSTRUCTIONS_RADIX).expect("Invalid input"))
    }

    pub fn read_decimal(&self) -> Result<i32> {
        let mut data = String::new();
        if io::stdin().read_line(&mut data).is_err() {
            bail!("Failed to read line")
        }

        Ok(data.trim().parse::<i32>().expect("Invalid input"))
    }

    pub fn read_string(&self) -> Result<String> {
        let mut data = String::new();
        if io::stdin().read_line(&mut data).is_err() {
            bail!("Failed to read line")
        }

        Ok(data)
    }

    pub fn sign(&self, x: i32) -> char {
        if x < 0 {
            '-'
        } else {
            '+'
        }
    }
}

impl Default for Simulator {
    fn default() -> Self {
        Self::new()
    }
}
