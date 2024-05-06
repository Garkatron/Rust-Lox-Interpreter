#[derive(Debug)]
pub enum Object {
    String(String),
    Number(f64),
    None(None),
}