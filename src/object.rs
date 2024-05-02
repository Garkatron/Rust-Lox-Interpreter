#[derive(Debug)]
pub enum Object {
    Integer(i32),
    String(String),
    Float(f64),
    None(None),
}