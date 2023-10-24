#[derive(PartialEq, Eq)]
pub enum Value {
    U8(u8),
    U64(u64),
    I8(i8),
    I64(i64),
    Bool(bool),
}
