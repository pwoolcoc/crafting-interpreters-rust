#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(Number),
}

#[derive(Debug)]
pub enum Number {
    Int(i64),
    Float(f64),
}

