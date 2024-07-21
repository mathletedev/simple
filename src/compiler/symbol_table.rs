use super::table_entry::{TableEntry, TableEntryType};

pub struct SymbolTable {
    data: Vec<TableEntry>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable { data: vec![] }
    }

    pub fn insert(&mut self, table_entry: TableEntry) {
        self.data.push(table_entry);
    }

    pub fn find(&self, symbol: i32, entry_type: TableEntryType) -> Option<TableEntry> {
        self.data
            .iter()
            .find(|table_entry| {
                table_entry.symbol == symbol && table_entry.entry_type == entry_type
            })
            .cloned()
    }
}
