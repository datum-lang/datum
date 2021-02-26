#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    Bool,
    Int(u16),
    Void,
    String,
    Bytes(u8),
}
