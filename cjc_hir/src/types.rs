#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    Bool,
    Void,
    String,
    Bytes(u8),
}
