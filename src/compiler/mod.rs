mod commands;
mod symbol_table;
mod table_entry;

use crate::{
    config::{INSTRUCTIONS_SEP, MEMORY},
    types::MyError,
};
use commands::COMMAND_TABLE;
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{prelude::*, BufReader, BufWriter},
    path::PathBuf,
};
use symbol_table::SymbolTable;
use table_entry::{TableEntry, TableEntryType};

pub struct Compiler {
    instruction_counter: u32,
    data_counter: u32,
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
            let tokens: Vec<String> = line.unwrap().split(' ').map(|x| x.to_string()).collect();

            if tokens.len() < 2 {
                println!(
                    "*** Syntax error on line {}: Incomplete statement ***",
                    i + 1
                );
                return;
            }

            // get line number
            let line_number: u32 = match tokens[0].parse() {
                Ok(line_number) => line_number,
                Err(_) => {
                    println!(
                        "*** Syntax error on line {}: Invalid line number ***",
                        i + 1
                    );
                    return;
                }
            };

            if self
                .symbol_table
                .find(line_number as i32, TableEntryType::LineNumber)
                .is_some()
            {
                println!(
                    "*** Syntax error on line {}: Line number already used ***",
                    i + 1
                );
                return;
            }

            // insert line number into symbol table
            self.symbol_table.insert(TableEntry {
                symbol: line_number as i32,
                entry_type: TableEntryType::LineNumber,
                location: self.instruction_counter,
            });

            // find command in uppercase
            let command = COMMAND_TABLE.get(&tokens[1].to_uppercase()).copied();

            match command {
                // call the command
                Some(command) => match command(self, &tokens[2..]) {
                    Ok(()) => {}
                    Err(error) => {
                        println!("*** Syntax error on line {}: {} ***", i + 1, error.details);
                        return;
                    }
                },
                None => {
                    println!(
                        "*** Syntax error on line {}: Invalid symbol {} ***",
                        i + 1,
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

    // returns current value of data counter and moves it up
    pub fn use_data_counter(&mut self) -> u32 {
        self.data_counter -= 1;
        self.data_counter + 1
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
            .truncate(true)
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
            Err(_) => Err(MyError::new("Failed to write to file")),
        }
    }

    pub fn to_symbol(&self, token: String) -> Result<(i32, TableEntryType), MyError> {
        // check if variable
        if token.len() != 1 || !token.chars().nth(0).unwrap().is_alphabetic() {
            // check if constant
            match token.parse::<i32>() {
                Ok(number) => Ok((number, TableEntryType::Constant)),
                Err(_) => Err(MyError::new("Invalid symbol")),
            }
        } else {
            Ok((
                token.chars().nth(0).unwrap() as i32,
                TableEntryType::Variable,
            ))
        }
    }

    pub fn infix_to_postfix(&self, infix: Vec<String>) -> Result<Vec<String>, MyError> {
        // infix to postfix: https://www.geeksforgeeks.org/convert-infix-expression-to-postfix-expression/
        let mut postfix: Vec<String> = vec![];
        let mut stack: Vec<String> = vec![];
        for token in infix {
            match self.to_symbol(token.clone()) {
                Ok(_) => {
                    postfix.push(token);
                }
                Err(_) => match token.as_str() {
                    "(" => {
                        stack.push(token);
                    }
                    ")" => loop {
                        let last = match stack.last() {
                            Some(last) => last,
                            None => {
                                return Err(MyError::new("Mismatched brackets"));
                            }
                        };
                        if last == "(" {
                            stack.pop();
                            break;
                        }

                        // pop all operators between brackets
                        postfix.push(stack.pop().unwrap());
                    },
                    "+" | "-" | "/" | "*" => {
                        // operator precedence table
                        let precedence: HashMap<&str, u8> =
                            HashMap::from([("(", 0), ("+", 1), ("-", 1), ("/", 2), ("*", 2)]);

                        // pop all operators with higher precedence
                        while !stack.is_empty()
                            && precedence.get(stack.last().unwrap().as_str())
                                >= precedence.get(token.as_str())
                        {
                            postfix.push(stack.pop().unwrap());
                        }

                        stack.push(token);
                    }
                    _ => {
                        return Err(MyError::new(format!("Unexpected token {}", token).as_str()));
                    }
                },
            }
        }

        while let Some(token) = stack.pop() {
            postfix.push(token);
        }

        Ok(postfix)
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
