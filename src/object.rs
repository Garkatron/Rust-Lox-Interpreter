#[derive(Debug, Clone)]
pub enum Object {
    String(String),
    Number(f64),
    None,
}