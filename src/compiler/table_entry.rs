#[derive(PartialEq, Eq)]
enum TableEntryType {
	Constant,
	LineNumber,
	Variable,
}

pub struct TableEntry {
	symbol: char,
	entry_type: TableEntryType,
	location: u16,
}
