#[derive(PartialEq, Eq, Clone, Copy)]
pub enum TableEntryType {
    Constant,
    LineNumber,
    Variable,
}

#[derive(Clone)]
pub struct TableEntry {
    pub symbol: i32,
    pub entry_type: TableEntryType,
    pub location: u32,
}
