use super::{compiler::Compiler, table_entry::TableEntryType, utils::to_symbol};
use crate::types::MyError;
use lazy_static::lazy_static;
use std::collections::HashMap;

type Command = fn(&mut Compiler, &[String]) -> Result<(), MyError>;

const REM: Command = |_, _| Ok(());

const INPUT: Command = |compiler, args| {
	if args.len() != 1 {
		return Err(MyError::new("INPUT command takes one argument"));
	}

	let symbol = match to_symbol(args[0].to_owned()) {
		Ok(symbol) => symbol,
		Err(error) => {
			return Err(error);
		}
	};

	if symbol.1 == TableEntryType::Constant {
		return Err(MyError::new("Cannot read into constant"));
	}

	let table_entry = compiler.find_or_create_symbol(symbol.0, TableEntryType::Variable);

	compiler.add_instruction(0x10, table_entry.location);

	Ok(())
};

const PRINT: Command = |compiler, args| {
	if args.len() != 1 {
		return Err(MyError::new("PRINT command takes one argument"));
	}

	let symbol = match to_symbol(args[0].to_owned()) {
		Ok(symbol) => symbol,
		Err(error) => {
			return Err(error);
		}
	};

	let table_entry = compiler.find_or_create_symbol(symbol.0, symbol.1);

	compiler.add_instruction(0x11, table_entry.location);

	Ok(())
};

const IF: Command = |compiler, args| {
	if args.len() != 5 {
		return Err(MyError::new("IF...GOTO command takes 4 arguments"));
	}

	let symbol1 = match to_symbol(args[0].to_owned()) {
		Ok(symbol) => symbol,
		Err(error) => {
			return Err(error);
		}
	};
	let symbol2 = match to_symbol(args[2].to_owned()) {
		Ok(symbol) => symbol,
		Err(error) => {
			return Err(error);
		}
	};
	let symbol3 = match to_symbol(args[4].to_owned()) {
		Ok(symbol) => symbol,
		Err(error) => {
			return Err(error);
		}
	};

	if symbol3.1 != TableEntryType::Constant {
		return Err(MyError::new("Cannot GOTO a variable"));
	}

	let table_entry1 = compiler.find_or_create_symbol(symbol1.0, symbol1.1);
	let table_entry2 = compiler.find_or_create_symbol(symbol2.0, symbol2.1);

	let mut needs_flag = false;
	let goto_pos = match compiler.find_line_number(symbol3.0) {
		Some(goto_pos) => goto_pos.symbol,
		None => {
			needs_flag = true;
			0
		}
	};

	match args[1].as_str() {
		"==" => {
			compiler.add_instruction(0x20, table_entry1.location);
			compiler.add_instruction(0x31, table_entry2.location);
			compiler.add_instruction(0x42, goto_pos as u32);
		}
		// TODO: other comparison operators
		_ => {
			return Err(MyError::new("Invalid comparison operator"));
		}
	}

	// process add_instruction() before adding flag
	// otherwise compiler.instruction_counter will be off
	if needs_flag {
		compiler.add_flag(symbol3.0);
	}

	Ok(())
};

const END: Command = |compiler, args| {
	if args.len() != 0 {
		return Err(MyError::new("PRINT command takes no arguments"));
	}

	compiler.add_instruction(0x43, 0);

	Ok(())
};

lazy_static! {
	pub static ref COMMAND_TABLE: HashMap<String, Command> = HashMap::from([
		("REM".to_string(), REM),
		("INPUT".to_string(), INPUT),
		("PRINT".to_string(), PRINT),
		("IF".to_string(), IF),
		("END".to_string(), END)
	]);
}
